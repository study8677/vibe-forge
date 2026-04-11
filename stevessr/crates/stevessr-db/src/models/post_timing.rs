use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PostTiming {
    pub id: i64,
    pub topic_id: i64,
    pub post_number: i32,
    pub user_id: i64,
    pub msecs: i64,
}

impl PostTiming {
    pub async fn find_by_topic_and_user(pool: &PgPool, topic_id: i64, user_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM post_timings WHERE topic_id = $1 AND user_id = $2 ORDER BY post_number ASC")
            .bind(topic_id)
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn upsert(pool: &PgPool, topic_id: i64, post_number: i32, user_id: i64, msecs: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"INSERT INTO post_timings (topic_id, post_number, user_id, msecs)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (topic_id, post_number, user_id) DO UPDATE SET msecs = post_timings.msecs + $4"#,
        )
        .bind(topic_id)
        .bind(post_number)
        .bind(user_id)
        .bind(msecs)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn total_time_for_user(pool: &PgPool, topic_id: i64, user_id: i64) -> Result<Option<i64>, sqlx::Error> {
        let row: Option<(Option<i64>,)> = sqlx::query_as(
            "SELECT SUM(msecs) FROM post_timings WHERE topic_id = $1 AND user_id = $2",
        )
        .bind(topic_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        Ok(row.and_then(|r| r.0))
    }
}
