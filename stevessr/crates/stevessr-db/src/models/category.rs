use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub color: String,
    pub text_color: String,
    pub topic_count: i32,
    pub post_count: i32,
    pub position: Option<i32>,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub description_excerpt: Option<String>,
    pub topic_url: Option<String>,
    pub read_restricted: bool,
    pub auto_close_hours: Option<f64>,
    pub auto_close_based_on_last_post: bool,
    pub topic_template: Option<String>,
    pub contains_messages: bool,
    pub sort_order: Option<String>,
    pub sort_ascending: Option<bool>,
    pub uploaded_logo_id: Option<i64>,
    pub uploaded_logo_dark_id: Option<i64>,
    pub uploaded_background_id: Option<i64>,
    pub all_topics_wiki: bool,
    pub allow_badges: bool,
    pub parent_category_id: Option<i64>,
    pub topics_day: i32,
    pub topics_week: i32,
    pub topics_month: i32,
    pub topics_year: i32,
    pub topics_all_time: i32,
    pub default_view: Option<String>,
    pub subcategory_list_style: Option<String>,
    pub default_top_period: Option<String>,
    pub minimum_required_tags: i32,
    pub navigate_to_first_post_after_read: bool,
    pub num_featured_topics: i32,
    pub default_slow_mode_seconds: Option<i32>,
    pub allow_unlimited_owner_edits_on_first_post: bool,
    pub reviewable_by_group_id: Option<i64>,
    pub search_priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Category {
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_slug(pool: &PgPool, slug: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM categories WHERE slug = $1")
            .bind(slug)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM categories ORDER BY position ASC NULLS LAST, name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_subcategories(pool: &PgPool, parent_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM categories WHERE parent_category_id = $1 ORDER BY position ASC NULLS LAST")
            .bind(parent_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        name: &str,
        slug: &str,
        color: &str,
        text_color: &str,
        parent_category_id: Option<i64>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            r#"INSERT INTO categories (name, slug, color, text_color, parent_category_id)
               VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
        )
        .bind(name)
        .bind(slug)
        .bind(color)
        .bind(text_color)
        .bind(parent_category_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_topic_count(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE categories SET topic_count = (SELECT COUNT(*) FROM topics WHERE category_id = $1 AND deleted_at IS NULL), updated_at = NOW() WHERE id = $1",
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
