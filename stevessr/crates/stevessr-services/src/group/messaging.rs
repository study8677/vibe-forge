use sqlx::PgPool;
use stevessr_core::error::{Error, Result};

/// Handles group messaging (sending PMs to all group members).
pub struct GroupMessaging;

impl GroupMessaging {
    /// Create a private message topic addressed to all members of a group.
    pub async fn message_group(
        pool: &PgPool,
        from_user_id: i64,
        group_id: i64,
        title: &str,
        raw: &str,
    ) -> Result<i64> {
        // Verify group exists and is messageable
        let group: Option<(i64, i32)> = sqlx::query_as(
            "SELECT id, messageable_level FROM groups WHERE id = $1"
        )
        .bind(group_id)
        .fetch_optional(pool)
        .await?;

        let (_gid, messageable_level) = group.ok_or(Error::NotFound {
            resource: "group",
            id: group_id.to_string(),
        })?;

        // messageable_level: 0=nobody, 1=only admins, 2=staff, 3=members, 4=everyone, 99=owners
        if messageable_level == 0 {
            return Err(Error::Forbidden("this group cannot receive messages".into()));
        }

        let slug = slug::slugify(title);

        // Create the PM topic
        let topic_row: (i64,) = sqlx::query_as(
            "INSERT INTO topics (title, slug, user_id, archetype, visible, created_at, updated_at, bumped_at)
             VALUES ($1, $2, $3, 'private_message', TRUE, NOW(), NOW(), NOW()) RETURNING id"
        )
        .bind(title)
        .bind(&slug)
        .bind(from_user_id)
        .fetch_one(pool)
        .await?;
        let topic_id = topic_row.0;

        // Create the first post
        sqlx::query(
            "INSERT INTO posts (topic_id, user_id, post_number, raw, cooked, created_at, updated_at)
             VALUES ($1, $2, 1, $3, $3, NOW(), NOW())"
        )
        .bind(topic_id)
        .bind(from_user_id)
        .bind(raw)
        .execute(pool)
        .await?;

        // Allow the sender
        sqlx::query(
            "INSERT INTO topic_allowed_users (user_id, topic_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())"
        )
        .bind(from_user_id)
        .bind(topic_id)
        .execute(pool)
        .await?;

        // Allow the group
        sqlx::query(
            "INSERT INTO topic_allowed_groups (group_id, topic_id, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())"
        )
        .bind(group_id)
        .bind(topic_id)
        .execute(pool)
        .await?;

        Ok(topic_id)
    }
}
