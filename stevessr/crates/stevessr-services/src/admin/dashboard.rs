use sqlx::PgPool;
use stevessr_core::error::Result;

#[derive(Debug)]
pub struct DashboardStats {
    pub total_users: i64,
    pub active_users_7d: i64,
    pub active_users_30d: i64,
    pub new_users_7d: i64,
    pub total_topics: i64,
    pub new_topics_7d: i64,
    pub total_posts: i64,
    pub new_posts_7d: i64,
    pub total_likes: i64,
    pub likes_7d: i64,
    pub pending_flags: i64,
    pub pending_reviewables: i64,
}

pub struct AdminDashboard;

impl AdminDashboard {
    pub async fn stats(pool: &PgPool) -> Result<DashboardStats> {
        let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE active = TRUE")
            .fetch_one(pool).await?;

        let active_users_7d: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE last_seen_at > NOW() - INTERVAL '7 days'"
        ).fetch_one(pool).await?;

        let active_users_30d: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE last_seen_at > NOW() - INTERVAL '30 days'"
        ).fetch_one(pool).await?;

        let new_users_7d: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE created_at > NOW() - INTERVAL '7 days'"
        ).fetch_one(pool).await?;

        let total_topics: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM topics WHERE deleted_at IS NULL"
        ).fetch_one(pool).await?;

        let new_topics_7d: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM topics WHERE created_at > NOW() - INTERVAL '7 days' AND deleted_at IS NULL"
        ).fetch_one(pool).await?;

        let total_posts: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM posts WHERE deleted_at IS NULL"
        ).fetch_one(pool).await?;

        let new_posts_7d: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM posts WHERE created_at > NOW() - INTERVAL '7 days' AND deleted_at IS NULL"
        ).fetch_one(pool).await?;

        let total_likes: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM post_actions WHERE post_action_type_id = 2 AND deleted_at IS NULL"
        ).fetch_one(pool).await?;

        let likes_7d: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM post_actions WHERE post_action_type_id = 2 AND created_at > NOW() - INTERVAL '7 days' AND deleted_at IS NULL"
        ).fetch_one(pool).await?;

        let pending_flags: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM post_actions WHERE post_action_type_id IN (3,6,7,8) AND deleted_at IS NULL AND agreed_by_id IS NULL AND deferred_by_id IS NULL"
        ).fetch_one(pool).await?;

        let pending_reviewables: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM reviewables WHERE status = 0"
        ).fetch_one(pool).await?;

        Ok(DashboardStats {
            total_users: total_users.0,
            active_users_7d: active_users_7d.0,
            active_users_30d: active_users_30d.0,
            new_users_7d: new_users_7d.0,
            total_topics: total_topics.0,
            new_topics_7d: new_topics_7d.0,
            total_posts: total_posts.0,
            new_posts_7d: new_posts_7d.0,
            total_likes: total_likes.0,
            likes_7d: likes_7d.0,
            pending_flags: pending_flags.0,
            pending_reviewables: pending_reviewables.0,
        })
    }

    /// Get the top referrers for the last N days.
    pub async fn top_referrers(pool: &PgPool, days: i32, limit: i64) -> Result<Vec<(String, i64)>> {
        let rows: Vec<(String, i64)> = sqlx::query_as(
            "SELECT incoming_referer, COUNT(*) as count
             FROM incoming_links
             WHERE created_at > NOW() - ($1 || ' days')::INTERVAL
             GROUP BY incoming_referer
             ORDER BY count DESC
             LIMIT $2"
        )
        .bind(days.to_string())
        .bind(limit)
        .fetch_all(pool)
        .await?;
        Ok(rows)
    }
}

// Facade function for the API layer
pub async fn get_dashboard_stats(pool: &PgPool) -> Result<serde_json::Value> { todo!() }
