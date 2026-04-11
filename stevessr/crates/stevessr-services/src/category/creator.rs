use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};
use stevessr_db::models::category::Category;

pub struct CreateCategoryParams {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub color: String,
    pub text_color: String,
    pub parent_category_id: Option<i64>,
    pub position: Option<i32>,
    pub user_id: i64,
}

pub struct CategoryCreator;

impl CategoryCreator {
    pub async fn create(pool: &PgPool, params: CreateCategoryParams) -> Result<Category> {
        Self::validate(&params)?;

        let slug = params.slug.unwrap_or_else(|| slug::slugify(&params.name));

        // Check name uniqueness within parent scope
        let existing: Option<(i64,)> = sqlx::query_as(
            "SELECT id FROM categories WHERE name_lower = $1 AND COALESCE(parent_category_id, 0) = COALESCE($2, 0)"
        )
        .bind(params.name.to_lowercase())
        .bind(params.parent_category_id)
        .fetch_optional(pool)
        .await?;

        if existing.is_some() {
            return Err(Error::AlreadyExists {
                resource: "category",
                detail: format!("category '{}' already exists at this level", params.name),
            });
        }

        // Validate parent exists if specified
        if let Some(parent_id) = params.parent_category_id {
            let parent = Category::find_by_id(pool, parent_id).await?;
            if parent.is_none() {
                return Err(Error::NotFound {
                    resource: "parent_category",
                    id: parent_id.to_string(),
                });
            }
            // Discourse limits nesting to 1 level
            if let Some(p) = parent {
                if p.parent_category_id.is_some() {
                    return Err(Error::Validation({
                        let mut e = ValidationErrors::new();
                        e.add("parent_category_id", "subcategories cannot have subcategories");
                        e
                    }));
                }
            }
        }

        let position = params.position.unwrap_or(0);

        let category = Category::create(
            pool,
            &params.name,
            &slug,
            &params.color,
            &params.text_color,
            params.parent_category_id,
        )
        .await?;

        Ok(category)
    }

    fn validate(params: &CreateCategoryParams) -> Result<()> {
        let mut errors = ValidationErrors::new();

        if params.name.is_empty() {
            errors.add("name", "must not be empty");
        }
        if params.name.len() > 50 {
            errors.add("name", "must be at most 50 characters");
        }
        if params.color.len() != 6 || !params.color.chars().all(|c| c.is_ascii_hexdigit()) {
            errors.add("color", "must be a 6-character hex color code");
        }
        if params.text_color.len() != 6 || !params.text_color.chars().all(|c| c.is_ascii_hexdigit()) {
            errors.add("text_color", "must be a 6-character hex color code");
        }

        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
