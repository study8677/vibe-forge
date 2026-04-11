use sqlx::PgPool;
use stevessr_core::error::Result;
use rand::Rng;

pub struct UserAnonymizer;

impl UserAnonymizer {
    pub async fn anonymize(pool: &PgPool, user_id: i64) -> Result<()> {
        let anon_username = format!("anon{}", rand::thread_rng().gen_range(100000..999999));
        sqlx::query(
            "UPDATE users SET username = $2, username_lower = $3, name = NULL, title = NULL, uploaded_avatar_id = NULL, updated_at = NOW() WHERE id = $1"
        )
        .bind(user_id)
        .bind(&anon_username)
        .bind(anon_username.to_lowercase())
        .execute(pool)
        .await?;

        sqlx::query("UPDATE user_profiles SET bio_raw = NULL, bio_cooked = NULL, location = NULL, website = NULL WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
