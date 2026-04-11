use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Represents a parsed incoming email.
pub struct IncomingEmail {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body_plain: String,
    pub body_html: Option<String>,
    pub message_id: String,
    pub in_reply_to: Option<String>,
}

/// Handles incoming emails (email-in feature for creating posts via email).
pub struct EmailReceiver;

impl EmailReceiver {
    /// Process an incoming email and create a post or topic.
    pub async fn process(pool: &PgPool, email: &IncomingEmail) -> Result<()> {
        // Log the incoming email
        sqlx::query(
            "INSERT INTO incoming_emails (from_address, to_addresses, subject, raw, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())"
        )
        .bind(&email.from)
        .bind(&email.to)
        .bind(&email.subject)
        .bind(&email.body_plain)
        .execute(pool)
        .await?;

        // Find the user by email
        let user: Option<(i64,)> = sqlx::query_as(
            "SELECT user_id FROM user_emails WHERE email = $1"
        )
        .bind(&email.from)
        .fetch_optional(pool)
        .await?;

        let user_id = match user {
            Some((uid,)) => uid,
            None => {
                tracing::warn!(from = %email.from, "incoming email from unknown address");
                return Err(Error::NotFound {
                    resource: "user",
                    id: email.from.clone(),
                });
            }
        };

        // Check if this is a reply to an existing topic
        if let Some(ref in_reply_to) = email.in_reply_to {
            if let Some(topic_id) = Self::find_topic_from_message_id(pool, in_reply_to).await? {
                return Self::create_reply(pool, user_id, topic_id, &email.body_plain).await;
            }
        }

        // Check if the to-address encodes a topic reply key
        if let Some(topic_id) = Self::parse_reply_key(&email.to) {
            return Self::create_reply(pool, user_id, topic_id, &email.body_plain).await;
        }

        // Otherwise, create a new topic
        Self::create_topic_from_email(pool, user_id, &email.subject, &email.body_plain).await
    }

    async fn find_topic_from_message_id(pool: &PgPool, message_id: &str) -> Result<Option<i64>> {
        let row: Option<(i64,)> = sqlx::query_as(
            "SELECT topic_id FROM incoming_emails WHERE message_id = $1 LIMIT 1"
        )
        .bind(message_id)
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|(id,)| id))
    }

    fn parse_reply_key(to_address: &str) -> Option<i64> {
        // Expected format: reply+{topic_id}@example.com
        let local = to_address.split('@').next()?;
        let topic_str = local.strip_prefix("reply+")?;
        topic_str.parse::<i64>().ok()
    }

    async fn create_reply(pool: &PgPool, user_id: i64, topic_id: i64, body: &str) -> Result<()> {
        let next_post_number: (i32,) = sqlx::query_as(
            "SELECT COALESCE(MAX(post_number), 0) + 1 FROM posts WHERE topic_id = $1"
        )
        .bind(topic_id)
        .fetch_one(pool)
        .await?;

        sqlx::query(
            "INSERT INTO posts (topic_id, user_id, post_number, raw, cooked, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $4, NOW(), NOW())"
        )
        .bind(topic_id)
        .bind(user_id)
        .bind(next_post_number.0)
        .bind(body)
        .execute(pool)
        .await?;

        sqlx::query(
            "UPDATE topics SET posts_count = posts_count + 1, last_posted_at = NOW(), bumped_at = NOW() WHERE id = $1"
        )
        .bind(topic_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn create_topic_from_email(pool: &PgPool, user_id: i64, subject: &str, body: &str) -> Result<()> {
        let slug = slug::slugify(subject);

        let topic_row: (i64,) = sqlx::query_as(
            "INSERT INTO topics (title, slug, user_id, archetype, visible, created_at, updated_at, bumped_at)
             VALUES ($1, $2, $3, 'regular', TRUE, NOW(), NOW(), NOW()) RETURNING id"
        )
        .bind(subject)
        .bind(&slug)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        sqlx::query(
            "INSERT INTO posts (topic_id, user_id, post_number, raw, cooked, created_at, updated_at)
             VALUES ($1, $2, 1, $3, $3, NOW(), NOW())"
        )
        .bind(topic_row.0)
        .bind(user_id)
        .bind(body)
        .execute(pool)
        .await?;

        Ok(())
    }
}
