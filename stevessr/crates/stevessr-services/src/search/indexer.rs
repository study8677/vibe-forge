use sqlx::PgPool;
use stevessr_core::error::Result;

/// Indexes posts and topics into the search_data table using PostgreSQL
/// full-text search vectors.
pub struct SearchIndexer;

impl SearchIndexer {
    /// Index or re-index a single post.
    pub async fn index_post(pool: &PgPool, post_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO post_search_data (post_id, search_data, raw_data, locale)
             SELECT p.id,
                    setweight(to_tsvector('english', COALESCE(t.title, '')), 'A') ||
                    setweight(to_tsvector('english', COALESCE(p.raw, '')), 'B'),
                    COALESCE(t.title, '') || ' ' || COALESCE(p.raw, ''),
                    'en'
             FROM posts p
             JOIN topics t ON t.id = p.topic_id
             WHERE p.id = $1
             ON CONFLICT (post_id) DO UPDATE SET
                search_data = EXCLUDED.search_data,
                raw_data = EXCLUDED.raw_data"
        )
        .bind(post_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Index or re-index a topic (updates all posts in the topic).
    pub async fn index_topic(pool: &PgPool, topic_id: i64) -> Result<()> {
        let post_ids: Vec<(i64,)> = sqlx::query_as(
            "SELECT id FROM posts WHERE topic_id = $1 AND deleted_at IS NULL"
        )
        .bind(topic_id)
        .fetch_all(pool)
        .await?;

        for (pid,) in post_ids {
            Self::index_post(pool, pid).await?;
        }

        Ok(())
    }

    /// Re-index a user's profile for people search.
    pub async fn index_user(pool: &PgPool, user_id: i64) -> Result<()> {
        sqlx::query(
            "INSERT INTO user_search_data (user_id, search_data, raw_data, locale)
             SELECT u.id,
                    setweight(to_tsvector('english', COALESCE(u.username, '')), 'A') ||
                    setweight(to_tsvector('english', COALESCE(u.name, '')), 'B') ||
                    setweight(to_tsvector('english', COALESCE(up.bio_raw, '')), 'C'),
                    COALESCE(u.username, '') || ' ' || COALESCE(u.name, '') || ' ' || COALESCE(up.bio_raw, ''),
                    'en'
             FROM users u
             LEFT JOIN user_profiles up ON up.user_id = u.id
             WHERE u.id = $1
             ON CONFLICT (user_id) DO UPDATE SET
                search_data = EXCLUDED.search_data,
                raw_data = EXCLUDED.raw_data"
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Rebuild the entire search index. Should be run as a background job.
    pub async fn rebuild_all(pool: &PgPool) -> Result<()> {
        // Re-index all posts
        let post_ids: Vec<(i64,)> = sqlx::query_as(
            "SELECT id FROM posts WHERE deleted_at IS NULL ORDER BY id"
        )
        .fetch_all(pool)
        .await?;

        for (pid,) in post_ids {
            Self::index_post(pool, pid).await?;
        }

        // Re-index all users
        let user_ids: Vec<(i64,)> = sqlx::query_as(
            "SELECT id FROM users WHERE active = TRUE ORDER BY id"
        )
        .fetch_all(pool)
        .await?;

        for (uid,) in user_ids {
            Self::index_user(pool, uid).await?;
        }

        Ok(())
    }
}
