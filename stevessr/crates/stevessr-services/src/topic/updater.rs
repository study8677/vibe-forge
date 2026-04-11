use sqlx::PgPool;
use stevessr_core::error::Result;

pub struct UpdateTopicParams {
    pub topic_id: i64,
    pub title: Option<String>,
    pub category_id: Option<i64>,
}

pub struct TopicUpdater;

impl TopicUpdater {
    pub async fn update(pool: &PgPool, params: UpdateTopicParams) -> Result<()> {
        let slug = params.title.as_ref().map(|t| slug::slugify(t));
        sqlx::query(
            "UPDATE topics SET title = COALESCE($2, title), slug = COALESCE($3, slug), category_id = COALESCE($4, category_id), updated_at = NOW() WHERE id = $1"
        )
        .bind(params.topic_id)
        .bind(&params.title)
        .bind(&slug)
        .bind(&params.category_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
