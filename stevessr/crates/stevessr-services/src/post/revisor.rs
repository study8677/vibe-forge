use sqlx::PgPool;
use stevessr_core::error::{Error, Result};
use stevessr_db::models::post::Post;

pub struct RevisePostParams {
    pub post_id: i64,
    pub editor_id: i64,
    pub raw: String,
    pub edit_reason: Option<String>,
}

pub struct PostRevisor;

impl PostRevisor {
    pub async fn revise(pool: &PgPool, params: RevisePostParams) -> Result<Post> {
        let post = Post::find_by_id(pool, params.post_id).await?.ok_or(Error::NotFound {
            resource: "post",
            id: params.post_id.to_string(),
        })?;

        // Store previous version as a revision
        Self::create_revision(pool, &post, params.editor_id).await?;

        // Cook the new raw content
        let cooked = Self::cook(&params.raw);

        // Update the post
        sqlx::query(
            "UPDATE posts SET raw = $2, cooked = $3, last_editor_id = $4, edit_reason = $5, version = version + 1, updated_at = NOW() WHERE id = $1"
        )
        .bind(params.post_id)
        .bind(&params.raw)
        .bind(&cooked)
        .bind(params.editor_id)
        .bind(&params.edit_reason)
        .execute(pool)
        .await?;

        // Return the updated post
        Post::find_by_id(pool, params.post_id)
            .await?
            .ok_or(Error::Internal("post disappeared after update".into()))
    }

    async fn create_revision(pool: &PgPool, post: &Post, editor_id: i64) -> Result<()> {
        let current_version = post.version;

        sqlx::query(
            "INSERT INTO post_revisions (post_id, user_id, number, modifications, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())"
        )
        .bind(post.id)
        .bind(editor_id)
        .bind(current_version)
        .bind(serde_json::json!({
            "raw": [&post.raw, ""],
            "cooked": [&post.cooked, ""],
        }).to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    fn cook(raw: &str) -> String {
        let escaped = raw
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");

        escaped
            .split("\n\n")
            .map(|para| format!("<p>{}</p>", para.trim()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
