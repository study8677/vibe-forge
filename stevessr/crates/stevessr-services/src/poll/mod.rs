pub mod vote_handler;

use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};

pub struct CreatePollParams {
    pub post_id: i64,
    pub name: String,
    pub poll_type: String, // "regular", "multiple", "number"
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub step: Option<i32>,
    pub close_at: Option<chrono::DateTime<chrono::Utc>>,
    pub options: Vec<String>,
}

pub struct PollManager;

impl PollManager {
    pub async fn create(pool: &PgPool, params: CreatePollParams) -> Result<i64> {
        Self::validate(&params)?;

        let row: (i64,) = sqlx::query_as(
            "INSERT INTO polls (post_id, name, type, min, max, step, close_at, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'open', NOW(), NOW()) RETURNING id"
        )
        .bind(params.post_id)
        .bind(&params.name)
        .bind(&params.poll_type)
        .bind(params.min)
        .bind(params.max)
        .bind(params.step)
        .bind(params.close_at)
        .fetch_one(pool)
        .await?;

        let poll_id = row.0;

        // Create poll options
        for (idx, option_text) in params.options.iter().enumerate() {
            let digest = format!("{:x}", md5::compute(option_text.as_bytes()));
            sqlx::query(
                "INSERT INTO poll_options (poll_id, digest, html, position, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, NOW(), NOW())"
            )
            .bind(poll_id)
            .bind(&digest)
            .bind(option_text)
            .bind(idx as i32)
            .execute(pool)
            .await?;
        }

        Ok(poll_id)
    }

    pub async fn close(pool: &PgPool, poll_id: i64) -> Result<()> {
        sqlx::query("UPDATE polls SET status = 'closed', updated_at = NOW() WHERE id = $1")
            .bind(poll_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_results(pool: &PgPool, poll_id: i64) -> Result<Vec<(String, String, i64)>> {
        let rows: Vec<(String, String, i64)> = sqlx::query_as(
            "SELECT po.digest, po.html,
                    (SELECT COUNT(*) FROM poll_votes pv WHERE pv.poll_option_id = po.id) as vote_count
             FROM poll_options po
             WHERE po.poll_id = $1
             ORDER BY po.position"
        )
        .bind(poll_id)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }

    fn validate(params: &CreatePollParams) -> Result<()> {
        let mut errors = ValidationErrors::new();
        if params.name.is_empty() {
            errors.add("name", "must not be empty");
        }
        if params.options.len() < 2 {
            errors.add("options", "must have at least 2 options");
        }
        if params.options.len() > 20 {
            errors.add("options", "must have at most 20 options");
        }
        if !["regular", "multiple", "number"].contains(&params.poll_type.as_str()) {
            errors.add("type", "must be regular, multiple, or number");
        }
        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
