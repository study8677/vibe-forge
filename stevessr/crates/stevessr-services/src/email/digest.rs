use sqlx::PgPool;
use stevessr_core::error::Result;

/// Generates email digest summaries for users who haven't visited recently.
pub struct DigestGenerator;

impl DigestGenerator {
    /// Find users who should receive a digest email.
    pub async fn users_needing_digest(pool: &PgPool, digest_type: &str) -> Result<Vec<(i64, String)>> {
        let interval = match digest_type {
            "daily" => "1 day",
            "weekly" => "7 days",
            _ => "7 days",
        };

        let rows: Vec<(i64, String)> = sqlx::query_as(
            "SELECT u.id, ue.email
             FROM users u
             JOIN user_emails ue ON ue.user_id = u.id AND ue.primary_email = TRUE
             JOIN user_options uo ON uo.user_id = u.id
             WHERE u.active = TRUE
               AND u.silenced_till IS NULL
               AND u.suspended_till IS NULL
               AND uo.email_digests = TRUE
               AND uo.digest_after_minutes IS NOT NULL
               AND (u.last_seen_at IS NULL OR u.last_seen_at < NOW() - ($1)::INTERVAL)
               AND (u.last_emailed_at IS NULL OR u.last_emailed_at < NOW() - ($1)::INTERVAL)"
        )
        .bind(interval)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// Generate digest content for a specific user.
    pub async fn generate_digest(pool: &PgPool, user_id: i64, since_hours: i32) -> Result<DigestContent> {
        // Popular topics since last digest
        let popular_topics: Vec<(i64, String, i32, i32)> = sqlx::query_as(
            "SELECT t.id, t.title, t.posts_count, t.like_count
             FROM topics t
             WHERE t.visible = TRUE AND t.deleted_at IS NULL AND t.archetype = 'regular'
               AND t.created_at > NOW() - ($1 || ' hours')::INTERVAL
             ORDER BY t.like_count DESC, t.posts_count DESC
             LIMIT 5"
        )
        .bind(since_hours.to_string())
        .fetch_all(pool)
        .await?;

        // New posts in topics the user is watching
        let new_in_watched: Vec<(i64, String, i64, String)> = sqlx::query_as(
            "SELECT p.id, p.raw, p.topic_id, t.title
             FROM posts p
             JOIN topics t ON t.id = p.topic_id
             JOIN topic_users tu ON tu.topic_id = t.id AND tu.user_id = $1
             WHERE tu.notification_level >= 3
               AND p.created_at > NOW() - ($2 || ' hours')::INTERVAL
               AND p.user_id != $1
               AND p.deleted_at IS NULL
             ORDER BY p.created_at DESC
             LIMIT 10"
        )
        .bind(user_id)
        .bind(since_hours.to_string())
        .fetch_all(pool)
        .await?;

        // Mark user as emailed
        sqlx::query("UPDATE users SET last_emailed_at = NOW() WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(DigestContent {
            popular_topics: popular_topics
                .into_iter()
                .map(|(id, title, posts, likes)| DigestTopic { id, title, posts_count: posts, like_count: likes })
                .collect(),
            new_in_watched: new_in_watched
                .into_iter()
                .map(|(post_id, excerpt, topic_id, topic_title)| DigestPost {
                    post_id,
                    excerpt: excerpt.chars().take(200).collect(),
                    topic_id,
                    topic_title,
                })
                .collect(),
        })
    }
}

pub struct DigestContent {
    pub popular_topics: Vec<DigestTopic>,
    pub new_in_watched: Vec<DigestPost>,
}

pub struct DigestTopic {
    pub id: i64,
    pub title: String,
    pub posts_count: i32,
    pub like_count: i32,
}

pub struct DigestPost {
    pub post_id: i64,
    pub excerpt: String,
    pub topic_id: i64,
    pub topic_title: String,
}
