use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};
use stevessr_core::constants::*;
use stevessr_db::models::post::Post;
use stevessr_db::models::topic::Topic;

pub struct CreatePostParams {
    pub topic_id: i64,
    pub user_id: i64,
    pub raw: String,
    pub reply_to_post_number: Option<i32>,
}

pub struct PostCreator;

impl PostCreator {
    pub async fn create(pool: &PgPool, params: CreatePostParams) -> Result<Post> {
        Self::validate(&params)?;

        // Verify topic exists and is not closed/archived
        let topic = Topic::find_by_id(pool, params.topic_id).await?.ok_or(Error::NotFound {
            resource: "topic",
            id: params.topic_id.to_string(),
        })?;

        if topic.closed {
            return Err(Error::Forbidden("topic is closed".into()));
        }
        if topic.archived {
            return Err(Error::Forbidden("topic is archived".into()));
        }

        // Determine post number
        let next_post_number = Self::next_post_number(pool, params.topic_id).await?;

        // Cook the raw markdown into HTML
        let cooked = Self::cook(&params.raw);

        // Create the post
        let post = Post::create(
            pool,
            params.user_id,
            params.topic_id,
            next_post_number,
            &params.raw,
            &cooked,
            params.reply_to_post_number,
            1, // post_type: regular
        )
        .await?;

        // Set reply_to_post_number if replying
        if let Some(reply_to) = params.reply_to_post_number {
            sqlx::query("UPDATE posts SET reply_to_post_number = $2 WHERE id = $1")
                .bind(post.id)
                .bind(reply_to)
                .execute(pool)
                .await?;

            // Update reply counts on the parent post
            sqlx::query(
                "UPDATE posts SET reply_count = reply_count + 1 WHERE topic_id = $1 AND post_number = $2"
            )
            .bind(params.topic_id)
            .bind(reply_to)
            .execute(pool)
            .await?;
        }

        // Update topic stats
        sqlx::query(
            "UPDATE topics SET posts_count = posts_count + 1, last_posted_at = NOW(), bumped_at = NOW(), updated_at = NOW() WHERE id = $1"
        )
        .bind(params.topic_id)
        .execute(pool)
        .await?;

        // Update user stats
        sqlx::query("UPDATE user_stats SET post_count = post_count + 1 WHERE user_id = $1")
            .bind(params.user_id)
            .execute(pool)
            .await?;

        // Create notifications for mentioned users
        Self::create_mention_notifications(pool, &post, &params.raw).await?;

        // Create reply notification
        if let Some(reply_to) = params.reply_to_post_number {
            Self::create_reply_notification(pool, &post, reply_to).await?;
        }

        Ok(post)
    }

    fn validate(params: &CreatePostParams) -> Result<()> {
        let mut errors = ValidationErrors::new();

        if params.raw.len() < POST_MIN_LENGTH {
            errors.add("raw", format!("must be at least {} characters", POST_MIN_LENGTH));
        }
        if params.raw.len() > POST_MAX_LENGTH {
            errors.add("raw", format!("must be at most {} characters", POST_MAX_LENGTH));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }

    async fn next_post_number(pool: &PgPool, topic_id: i64) -> Result<i32> {
        let row: (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(post_number), 0) + 1 FROM posts WHERE topic_id = $1"
        )
        .bind(topic_id)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    /// Basic markdown to HTML cooking. In production, this would use a full
    /// CommonMark pipeline with Discourse-specific extensions.
    fn cook(raw: &str) -> String {
        // Minimal cooking: wrap paragraphs in <p> tags, escape HTML
        let escaped = raw
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");

        escaped
            .split("\n\n")
            .map(|para| format!("<p>{}</p>", para.trim()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    async fn create_mention_notifications(pool: &PgPool, post: &Post, raw: &str) -> Result<()> {
        // Extract @mentions from raw text
        let mention_re = regex::Regex::new(r"@(\w+)").unwrap();
        for cap in mention_re.captures_iter(raw) {
            let username = &cap[1];
            if let Ok(Some(mentioned_user)) = stevessr_db::models::user::User::find_by_username(pool, username).await {
                if mentioned_user.id != post.user_id {
                    sqlx::query(
                        "INSERT INTO notifications (notification_type, user_id, data, topic_id, post_number, created_at, updated_at)
                         VALUES (1, $1, $2, $3, $4, NOW(), NOW())"
                    )
                    .bind(mentioned_user.id)
                    .bind(serde_json::json!({
                        "display_username": post.user_id.to_string(),
                        "topic_title": "",
                    }).to_string())
                    .bind(post.topic_id)
                    .bind(post.post_number)
                    .execute(pool)
                    .await?;
                }
            }
        }
        Ok(())
    }

    async fn create_reply_notification(pool: &PgPool, post: &Post, reply_to_post_number: i32) -> Result<()> {
        // Find the author of the post being replied to
        let parent: Option<(i64,)> = sqlx::query_as(
            "SELECT user_id FROM posts WHERE topic_id = $1 AND post_number = $2"
        )
        .bind(post.topic_id)
        .bind(reply_to_post_number)
        .fetch_optional(pool)
        .await?;

        if let Some((parent_user_id,)) = parent {
            if parent_user_id != post.user_id {
                sqlx::query(
                    "INSERT INTO notifications (notification_type, user_id, data, topic_id, post_number, created_at, updated_at)
                     VALUES (6, $1, $2, $3, $4, NOW(), NOW())"
                )
                .bind(parent_user_id)
                .bind(serde_json::json!({
                    "display_username": post.user_id.to_string(),
                }).to_string())
                .bind(post.topic_id)
                .bind(post.post_number)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }
}
