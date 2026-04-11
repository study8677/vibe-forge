use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicViewStat {
    pub id: i64,
    pub topic_id: i64,
    pub viewed_at: NaiveDate,
    pub anonymous_views: i32,
    pub logged_in_views: i32,
}

impl TopicViewStat {
    pub async fn find_by_topic(pool: &PgPool, topic_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topic_view_stats WHERE topic_id = $1 ORDER BY viewed_at DESC")
            .bind(topic_id)
            .fetch_all(pool)
            .await
    }

    pub async fn increment(pool: &PgPool, topic_id: i64, date: NaiveDate, logged_in: bool) -> Result<(), sqlx::Error> {
        if logged_in {
            sqlx::query(
                r#"INSERT INTO topic_view_stats (topic_id, viewed_at, logged_in_views, anonymous_views)
                   VALUES ($1, $2, 1, 0)
                   ON CONFLICT (topic_id, viewed_at) DO UPDATE SET logged_in_views = topic_view_stats.logged_in_views + 1"#,
            )
            .bind(topic_id)
            .bind(date)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                r#"INSERT INTO topic_view_stats (topic_id, viewed_at, anonymous_views, logged_in_views)
                   VALUES ($1, $2, 1, 0)
                   ON CONFLICT (topic_id, viewed_at) DO UPDATE SET anonymous_views = topic_view_stats.anonymous_views + 1"#,
            )
            .bind(topic_id)
            .bind(date)
            .execute(pool)
            .await?;
        }
        Ok(())
    }
}
