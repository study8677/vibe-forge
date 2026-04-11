use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserOption {
    pub id: i64,
    pub user_id: i64,
    pub mailing_list_mode: bool,
    pub mailing_list_mode_frequency: i16,
    pub email_digests: bool,
    pub email_level: i16,
    pub email_messages_level: i16,
    pub external_links_in_new_tab: bool,
    pub dark_scheme_id: Option<i64>,
    pub dynamic_favicon: bool,
    pub enable_quoting: bool,
    pub enable_defer: bool,
    pub digest_after_minutes: i32,
    pub automatically_unpin_topics: bool,
    pub auto_track_topics_after_msecs: i32,
    pub notification_level_when_replying: i16,
    pub new_topic_duration_minutes: i32,
    pub email_previous_replies: i16,
    pub email_in_reply_to: bool,
    pub like_notification_frequency: i16,
    pub include_tl0_in_digests: bool,
    pub theme_ids: Vec<i64>,
    pub theme_key_seq: i32,
    pub hide_profile_and_presence: bool,
    pub text_size: i16,
    pub title_count_mode: i16,
    pub timezone: Option<String>,
    pub skip_new_user_tips: bool,
    pub default_calendar: i16,
    pub oldest_search_log_date: Option<DateTime<Utc>>,
    pub sidebar_link_to_filtered_list: bool,
    pub sidebar_show_count_of_new_items: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserOption {
    pub async fn find_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM user_options WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(pool: &PgPool, user_id: i64) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO user_options (user_id) VALUES ($1) RETURNING *",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
    }

    pub async fn update_timezone(pool: &PgPool, user_id: i64, timezone: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_options SET timezone = $2, updated_at = NOW() WHERE user_id = $1")
            .bind(user_id)
            .bind(timezone)
            .execute(pool)
            .await?;
        Ok(())
    }
}
