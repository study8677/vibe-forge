use sqlx::PgPool;
use stevessr_core::error::Result;

/// Emits webhook events to registered webhook endpoints.
pub struct WebhookEmitter;

impl WebhookEmitter {
    /// Emit an event to all webhooks subscribed to this event type.
    pub async fn emit(pool: &PgPool, event_type: &str, payload: &serde_json::Value) -> Result<()> {
        // Find all active webhooks that subscribe to this event type
        let webhooks: Vec<(i64, String, String, String)> = sqlx::query_as(
            "SELECT wh.id, wh.payload_url, wh.secret, wh.content_type
             FROM web_hooks wh
             WHERE wh.active = TRUE
               AND (wh.wildcard_web_hook = TRUE
                    OR EXISTS (SELECT 1 FROM web_hook_event_types whet WHERE whet.web_hook_id = wh.id AND whet.name = $1))"
        )
        .bind(event_type)
        .fetch_all(pool)
        .await?;

        for (webhook_id, payload_url, secret, content_type) in webhooks {
            let result = Self::deliver(&payload_url, &secret, &content_type, event_type, payload).await;

            // Log the delivery attempt
            let (status, response_body) = match &result {
                Ok((status, body)) => (*status, body.clone()),
                Err(e) => (0i32, e.to_string()),
            };

            sqlx::query(
                "INSERT INTO web_hook_events (web_hook_id, event_type, payload, status, response_body, created_at)
                 VALUES ($1, $2, $3, $4, $5, NOW())"
            )
            .bind(webhook_id)
            .bind(event_type)
            .bind(payload.to_string())
            .bind(status)
            .bind(&response_body)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    async fn deliver(
        url: &str,
        secret: &str,
        content_type: &str,
        event_type: &str,
        payload: &serde_json::Value,
    ) -> std::result::Result<(i32, String), Box<dyn std::error::Error + Send + Sync>> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let body = payload.to_string();

        // Compute HMAC signature
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
        mac.update(body.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("Content-Type", content_type)
            .header("X-Discourse-Event", event_type)
            .header("X-Discourse-Event-Signature", format!("sha256={}", signature))
            .body(body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        let status = response.status().as_u16() as i32;
        let body = response.text().await.unwrap_or_default();

        Ok((status, body))
    }

    /// Redeliver a previously failed webhook event.
    pub async fn redeliver(pool: &PgPool, event_id: i64) -> Result<()> {
        let event: Option<(i64, String, String)> = sqlx::query_as(
            "SELECT web_hook_id, event_type, payload FROM web_hook_events WHERE id = $1"
        )
        .bind(event_id)
        .fetch_optional(pool)
        .await?;

        if let Some((webhook_id, event_type, payload_str)) = event {
            let payload: serde_json::Value = serde_json::from_str(&payload_str)
                .unwrap_or(serde_json::json!({}));

            let webhook: Option<(String, String, String)> = sqlx::query_as(
                "SELECT payload_url, secret, content_type FROM web_hooks WHERE id = $1 AND active = TRUE"
            )
            .bind(webhook_id)
            .fetch_optional(pool)
            .await?;

            if let Some((url, secret, content_type)) = webhook {
                let result = Self::deliver(&url, &secret, &content_type, &event_type, &payload).await;

                let (status, response_body) = match result {
                    Ok((s, b)) => (s, b),
                    Err(e) => (0, e.to_string()),
                };

                // Update the event record
                sqlx::query(
                    "UPDATE web_hook_events SET status = $2, response_body = $3 WHERE id = $1"
                )
                .bind(event_id)
                .bind(status)
                .bind(&response_body)
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }
}
