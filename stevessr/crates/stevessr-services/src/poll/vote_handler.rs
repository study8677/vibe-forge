use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

pub struct PollVoteHandler;

impl PollVoteHandler {
    /// Cast a vote on a poll.
    pub async fn vote(pool: &PgPool, poll_id: i64, user_id: i64, option_digests: &[String]) -> Result<()> {
        // Check poll is open
        let poll_status: Option<(String, String)> = sqlx::query_as(
            "SELECT status, type FROM polls WHERE id = $1"
        )
        .bind(poll_id)
        .fetch_optional(pool)
        .await?;

        let (status, poll_type) = poll_status.ok_or(Error::NotFound {
            resource: "poll",
            id: poll_id.to_string(),
        })?;

        if status != "open" {
            return Err(Error::Forbidden("poll is closed".into()));
        }

        // Validate option count for regular polls
        if poll_type == "regular" && option_digests.len() != 1 {
            return Err(Error::Validation({
                let mut e = stevessr_core::error::ValidationErrors::new();
                e.add("options", "must select exactly one option for regular polls");
                e
            }));
        }

        // Remove existing votes for this user on this poll
        sqlx::query(
            "DELETE FROM poll_votes WHERE poll_id = $1 AND user_id = $2"
        )
        .bind(poll_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        // Cast new votes
        for digest in option_digests {
            let option_id: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM poll_options WHERE poll_id = $1 AND digest = $2"
            )
            .bind(poll_id)
            .bind(digest)
            .fetch_optional(pool)
            .await?;

            let (opt_id,) = option_id.ok_or(Error::NotFound {
                resource: "poll_option",
                id: digest.clone(),
            })?;

            sqlx::query(
                "INSERT INTO poll_votes (poll_id, poll_option_id, user_id, created_at, updated_at)
                 VALUES ($1, $2, $3, NOW(), NOW())"
            )
            .bind(poll_id)
            .bind(opt_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        }

        // Update vote count on the poll
        sqlx::query(
            "UPDATE polls SET voters = (SELECT COUNT(DISTINCT user_id) FROM poll_votes WHERE poll_id = $1) WHERE id = $1"
        )
        .bind(poll_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Remove a user's vote from a poll.
    pub async fn remove_vote(pool: &PgPool, poll_id: i64, user_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM poll_votes WHERE poll_id = $1 AND user_id = $2")
            .bind(poll_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        sqlx::query(
            "UPDATE polls SET voters = (SELECT COUNT(DISTINCT user_id) FROM poll_votes WHERE poll_id = $1) WHERE id = $1"
        )
        .bind(poll_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get a user's current votes for a poll.
    pub async fn user_votes(pool: &PgPool, poll_id: i64, user_id: i64) -> Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT po.digest FROM poll_votes pv JOIN poll_options po ON po.id = pv.poll_option_id
             WHERE pv.poll_id = $1 AND pv.user_id = $2"
        )
        .bind(poll_id)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(d,)| d).collect())
    }
}
