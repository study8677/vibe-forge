use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub fancy_title: Option<String>,
    pub slug: String,
    pub user_id: i64,
    pub last_post_user_id: Option<i64>,
    pub reply_count: i32,
    pub posts_count: i32,
    pub highest_post_number: i32,
    pub highest_staff_post_number: i32,
    pub category_id: Option<i64>,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    pub moderator_posts_count: i32,
    pub bumped_at: DateTime<Utc>,
    pub pinned_at: Option<DateTime<Utc>>,
    pub pinned_globally: bool,
    pub pinned_until: Option<DateTime<Utc>>,
    pub image_upload_id: Option<i64>,
    pub word_count: Option<i32>,
    pub excerpt: Option<String>,
    pub participant_count: i32,
    pub like_count: i32,
    pub views: i32,
    pub incoming_link_count: i32,
    pub archetype: String,
    pub featured_link: Option<String>,
    pub notify_moderators_count: i32,
    pub spam_count: i32,
    pub score: f64,
    pub percent_rank: f64,
    pub subtype: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_id: Option<i64>,
    pub has_summary: bool,
    pub reviewable_score: f64,
    pub slow_mode_seconds: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Topic {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topics WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM topics WHERE slug = $1 AND deleted_at IS NULL")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_category(pool: &PgPool, category_id: i64, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT * FROM topics WHERE category_id = $1 AND deleted_at IS NULL AND visible = TRUE ORDER BY bumped_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(category_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        title: &str,
        slug: &str,
        user_id: i64,
        category_id: Option<i64>,
        archetype: &str,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO topics (title, slug, user_id, category_id, archetype, bumped_at)
               VALUES ($1, $2, $3, $4, $5, NOW()) RETURNING *"#,
        )
        .bind(title)
        .bind(slug)
        .bind(user_id)
        .bind(category_id)
        .bind(archetype)
        .fetch_one(pool)
        .await
    }

    pub async fn update_bumped_at(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET bumped_at = NOW(), updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_posts_count(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET posts_count = posts_count + 1, highest_post_number = highest_post_number + 1, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn increment_views(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET views = views + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn close(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET closed = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn archive(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET archived = TRUE, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn soft_delete(pool: &PgPool, id: i64, deleted_by_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET deleted_at = NOW(), deleted_by_id = $2, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .bind(deleted_by_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn recover(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE topics SET deleted_at = NULL, deleted_by_id = NULL, updated_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM topics WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
