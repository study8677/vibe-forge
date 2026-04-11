use crate::models::category::Category;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct CategoryListQuery {
    parent_category_id: Option<i64>,
    include_subcategories: bool,
    order: CategoryOrder,
}

#[derive(Debug, Clone)]
pub enum CategoryOrder {
    Position,
    Name,
    TopicCount,
    Created,
}

impl Default for CategoryListQuery {
    fn default() -> Self {
        Self {
            parent_category_id: None,
            include_subcategories: true,
            order: CategoryOrder::Position,
        }
    }
}

impl CategoryListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parent(mut self, parent_id: i64) -> Self {
        self.parent_category_id = Some(parent_id);
        self
    }

    pub fn include_subcategories(mut self, val: bool) -> Self {
        self.include_subcategories = val;
        self
    }

    pub fn order_by(mut self, order: CategoryOrder) -> Self {
        self.order = order;
        self
    }

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
        let mut sql = String::from("SELECT * FROM categories");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if let Some(_) = self.parent_category_id {
            conditions.push(format!("parent_category_id = ${}", param_idx));
            param_idx += 1;
        } else if !self.include_subcategories {
            conditions.push("parent_category_id IS NULL".to_string());
        }

        let _ = param_idx;

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            CategoryOrder::Position => "position ASC NULLS LAST, name ASC",
            CategoryOrder::Name => "name ASC",
            CategoryOrder::TopicCount => "topic_count DESC",
            CategoryOrder::Created => "created_at DESC",
        };
        sql.push_str(&format!(" ORDER BY {}", order_clause));

        let mut query = sqlx::query_as::<_, Category>(&sql);

        if let Some(parent_id) = self.parent_category_id {
            query = query.bind(parent_id);
        }

        query.fetch_all(pool).await
    }
}
