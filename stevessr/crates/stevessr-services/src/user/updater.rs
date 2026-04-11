use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct UpdateUserParams {
    pub user_id: i64,
    pub name: Option<String>,
    pub title: Option<String>,
    pub locale: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub bio_raw: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
}

pub struct UserUpdater;

impl UserUpdater {
    pub async fn update(pool: &PgPool, params: UpdateUserParams) -> Result<()> {
        if params.name.is_some() || params.title.is_some() || params.locale.is_some() || params.date_of_birth.is_some() {
            sqlx::query(
                "UPDATE users SET name = COALESCE($2, name), title = COALESCE($3, title), locale = COALESCE($4, locale), date_of_birth = COALESCE($5, date_of_birth), updated_at = NOW() WHERE id = $1"
            )
            .bind(params.user_id)
            .bind(&params.name)
            .bind(&params.title)
            .bind(&params.locale)
            .bind(&params.date_of_birth)
            .execute(pool)
            .await?;
        }

        if params.bio_raw.is_some() || params.location.is_some() || params.website.is_some() {
            sqlx::query(
                "UPDATE user_profiles SET bio_raw = COALESCE($2, bio_raw), location = COALESCE($3, location), website = COALESCE($4, website) WHERE user_id = $1"
            )
            .bind(params.user_id)
            .bind(&params.bio_raw)
            .bind(&params.location)
            .bind(&params.website)
            .execute(pool)
            .await?;
        }

        Ok(())
    }
}
