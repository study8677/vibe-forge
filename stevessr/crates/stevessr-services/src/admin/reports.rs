use sqlx::PgPool;
use stevessr_core::error::Result;

#[derive(Debug)]
pub struct ReportDataPoint {
    pub date: chrono::NaiveDate,
    pub count: i64,
}

pub struct ReportGenerator;

impl ReportGenerator {
    /// Generate daily signups report.
    pub async fn daily_signups(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<ReportDataPoint>> {
        let rows: Vec<(chrono::NaiveDate, i64)> = sqlx::query_as(
            "SELECT DATE(created_at) as day, COUNT(*) as count
             FROM users
             WHERE DATE(created_at) BETWEEN $1 AND $2
             GROUP BY day
             ORDER BY day"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(date, count)| ReportDataPoint { date, count }).collect())
    }

    /// Generate daily topics report.
    pub async fn daily_topics(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<ReportDataPoint>> {
        let rows: Vec<(chrono::NaiveDate, i64)> = sqlx::query_as(
            "SELECT DATE(created_at) as day, COUNT(*) as count
             FROM topics
             WHERE DATE(created_at) BETWEEN $1 AND $2 AND deleted_at IS NULL
             GROUP BY day
             ORDER BY day"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(date, count)| ReportDataPoint { date, count }).collect())
    }

    /// Generate daily posts report.
    pub async fn daily_posts(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<ReportDataPoint>> {
        let rows: Vec<(chrono::NaiveDate, i64)> = sqlx::query_as(
            "SELECT DATE(created_at) as day, COUNT(*) as count
             FROM posts
             WHERE DATE(created_at) BETWEEN $1 AND $2 AND deleted_at IS NULL
             GROUP BY day
             ORDER BY day"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(date, count)| ReportDataPoint { date, count }).collect())
    }

    /// Generate daily active users report.
    pub async fn daily_active_users(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<ReportDataPoint>> {
        let rows: Vec<(chrono::NaiveDate, i64)> = sqlx::query_as(
            "SELECT DATE(last_seen_at) as day, COUNT(DISTINCT id) as count
             FROM users
             WHERE DATE(last_seen_at) BETWEEN $1 AND $2
             GROUP BY day
             ORDER BY day"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(date, count)| ReportDataPoint { date, count }).collect())
    }

    /// Generate daily likes report.
    pub async fn daily_likes(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<ReportDataPoint>> {
        let rows: Vec<(chrono::NaiveDate, i64)> = sqlx::query_as(
            "SELECT DATE(created_at) as day, COUNT(*) as count
             FROM post_actions
             WHERE post_action_type_id = 2 AND DATE(created_at) BETWEEN $1 AND $2 AND deleted_at IS NULL
             GROUP BY day
             ORDER BY day"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(date, count)| ReportDataPoint { date, count }).collect())
    }

    /// Generate daily page views report.
    pub async fn daily_page_views(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate) -> Result<Vec<ReportDataPoint>> {
        let rows: Vec<(chrono::NaiveDate, i64)> = sqlx::query_as(
            "SELECT DATE(viewed_at) as day, COUNT(*) as count
             FROM topic_views
             WHERE DATE(viewed_at) BETWEEN $1 AND $2
             GROUP BY day
             ORDER BY day"
        )
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(date, count)| ReportDataPoint { date, count }).collect())
    }

    /// Generate top users report for a period.
    pub async fn top_users(pool: &PgPool, start: chrono::NaiveDate, end: chrono::NaiveDate, limit: i64) -> Result<Vec<(i64, String, i64, i64, i64)>> {
        let rows: Vec<(i64, String, i64, i64, i64)> = sqlx::query_as(
            "SELECT u.id, u.username,
                    (SELECT COUNT(*) FROM posts p WHERE p.user_id = u.id AND DATE(p.created_at) BETWEEN $1 AND $2 AND p.deleted_at IS NULL) as posts,
                    (SELECT COUNT(*) FROM topics t WHERE t.user_id = u.id AND DATE(t.created_at) BETWEEN $1 AND $2 AND t.deleted_at IS NULL) as topics,
                    (SELECT COALESCE(SUM(p2.like_count), 0) FROM posts p2 WHERE p2.user_id = u.id AND DATE(p2.created_at) BETWEEN $1 AND $2) as likes_received
             FROM users u
             WHERE u.active = TRUE
             ORDER BY posts DESC
             LIMIT $3"
        )
        .bind(start)
        .bind(end)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }
}

// Facade function for the API layer
pub async fn generate_report(pool: &PgPool, _report_type: &str, _start_date: Option<&str>, _end_date: Option<&str>) -> Result<serde_json::Value> { todo!() }
