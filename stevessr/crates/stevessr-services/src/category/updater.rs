use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct UpdateCategoryParams {
    pub category_id: i64,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub position: Option<i32>,
}

pub struct CategoryUpdater;

impl CategoryUpdater {
    pub async fn update(pool: &PgPool, params: UpdateCategoryParams) -> Result<()> {
        let slug = params.name.as_ref().map(|n| slug::slugify(n)).or(params.slug.clone());

        sqlx::query(
            "UPDATE categories SET
                name = COALESCE($2, name),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description),
                color = COALESCE($5, color),
                text_color = COALESCE($6, text_color),
                position = COALESCE($7, position),
                updated_at = NOW()
             WHERE id = $1"
        )
        .bind(params.category_id)
        .bind(&params.name)
        .bind(&slug)
        .bind(&params.description)
        .bind(&params.color)
        .bind(&params.text_color)
        .bind(&params.position)
        .execute(pool)
        .await?;

        Ok(())
    }
}
