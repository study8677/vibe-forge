use sqlx::PgPool;
use stevessr_core::error::Result;

/// Basic spam detection and handling.
pub struct SpamHandler;

impl SpamHandler {
    /// Check if a post looks like spam.
    /// Returns true if the post should be flagged/hidden.
    pub fn is_spam(raw: &str, user_trust_level: i16) -> bool {
        // High trust level users are not checked for spam
        if user_trust_level >= 2 {
            return false;
        }

        let lowercase = raw.to_lowercase();

        // Count links
        let link_count = lowercase.matches("http://").count() + lowercase.matches("https://").count();

        // New users (TL0) posting many links
        if user_trust_level == 0 && link_count > 2 {
            return true;
        }

        // All caps check (more than 80% uppercase in a long message)
        if raw.len() > 50 {
            let upper_count = raw.chars().filter(|c| c.is_uppercase()).count();
            if upper_count as f64 / raw.len() as f64 > 0.8 {
                return true;
            }
        }

        // Repeated text patterns
        if Self::has_excessive_repetition(raw) {
            return true;
        }

        false
    }

    /// Check if user is posting too frequently (rate limit for new users).
    pub async fn is_rate_limited(pool: &PgPool, user_id: i64, trust_level: i16) -> Result<bool> {
        if trust_level >= 2 {
            return Ok(false);
        }

        let window_minutes = match trust_level {
            0 => 5,
            1 => 1,
            _ => 0,
        };

        if window_minutes == 0 {
            return Ok(false);
        }

        let max_posts = match trust_level {
            0 => 3,
            1 => 10,
            _ => 100,
        };

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM posts
             WHERE user_id = $1 AND created_at > NOW() - ($2 || ' minutes')::INTERVAL"
        )
        .bind(user_id)
        .bind(window_minutes.to_string())
        .fetch_one(pool)
        .await?;

        Ok(count.0 >= max_posts)
    }

    /// Auto-silence a user detected as a spammer.
    pub async fn silence_spammer(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE users SET silenced_till = NOW() + INTERVAL '999 years', updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        // Hide all their posts
        sqlx::query(
            "UPDATE posts SET hidden = TRUE, hidden_at = NOW(), hidden_reason_id = 1 WHERE user_id = $1 AND hidden = FALSE"
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    fn has_excessive_repetition(text: &str) -> bool {
        if text.len() < 20 {
            return false;
        }
        let chars: Vec<char> = text.chars().collect();
        let mut max_repeat = 0;
        let mut current_repeat = 1;
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                current_repeat += 1;
                if current_repeat > max_repeat {
                    max_repeat = current_repeat;
                }
            } else {
                current_repeat = 1;
            }
        }
        max_repeat > 10
    }
}
