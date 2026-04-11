pub mod emitter;

use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct WebhookManager;

impl WebhookManager {
    pub async fn create(
        pool: &PgPool,
        payload_url: &str,
        secret: &str,
        content_type: &str,
        wildcard_web_hook: bool,
        active: bool,
    ) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO web_hooks (payload_url, secret, content_type, wildcard_web_hook, active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) RETURNING id"
        )
        .bind(payload_url)
        .bind(secret)
        .bind(content_type)
        .bind(wildcard_web_hook)
        .bind(active)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    pub async fn subscribe_to_event(pool: &PgPool, webhook_id: i64, event_type: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO web_hook_event_types (web_hook_id, name, created_at, updated_at) VALUES ($1, $2, NOW(), NOW()) ON CONFLICT DO NOTHING"
        )
        .bind(webhook_id)
        .bind(event_type)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn destroy(pool: &PgPool, webhook_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM web_hook_event_types WHERE web_hook_id = $1")
            .bind(webhook_id).execute(pool).await?;
        sqlx::query("DELETE FROM web_hook_events WHERE web_hook_id = $1")
            .bind(webhook_id).execute(pool).await?;
        sqlx::query("DELETE FROM web_hooks WHERE id = $1")
            .bind(webhook_id).execute(pool).await?;
        Ok(())
    }
}
