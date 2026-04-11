use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use ipnetwork::IpNetwork;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScreenedIpAddress {
    pub id: i64,
    pub ip_address: IpNetwork,
    pub action_type: i32,
    pub match_count: i32,
    pub last_match_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ScreenedIpAddress {
    pub async fn find_match(pool: &PgPool, ip: IpNetwork) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM screened_ip_addresses WHERE ip_address >>= $1 LIMIT 1")
            .bind(ip)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM screened_ip_addresses ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &PgPool, ip_address: IpNetwork, action_type: i32) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO screened_ip_addresses (ip_address, action_type) VALUES ($1, $2) RETURNING *",
        )
        .bind(ip_address)
        .bind(action_type)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM screened_ip_addresses WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
