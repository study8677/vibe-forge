use crate::models::reviewable::Reviewable;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct ReviewableListQuery {
    status: Option<i32>,
    reviewable_type: Option<String>,
    category_id: Option<i64>,
    topic_id: Option<i64>,
    created_by_id: Option<i64>,
    order: ReviewableOrder,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum ReviewableOrder {
    Score,
    Created,
    Priority,
}

impl Default for ReviewableListQuery {
    fn default() -> Self {
        Self {
            status: Some(0), // pending by default
            reviewable_type: None,
            category_id: None,
            topic_id: None,
            created_by_id: None,
            order: ReviewableOrder::Priority,
            limit: 60,
            offset: 0,
        }
    }
}

impl ReviewableListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    pub fn all_statuses(mut self) -> Self {
        self.status = None;
        self
    }

    pub fn reviewable_type(mut self, rtype: &str) -> Self {
        self.reviewable_type = Some(rtype.to_string());
        self
    }

    pub fn category(mut self, category_id: i64) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn topic(mut self, topic_id: i64) -> Self {
        self.topic_id = Some(topic_id);
        self
    }

    pub fn created_by(mut self, user_id: i64) -> Self {
        self.created_by_id = Some(user_id);
        self
    }

    pub fn order_by(mut self, order: ReviewableOrder) -> Self {
        self.order = order;
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = offset;
        self
    }

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Reviewable>, sqlx::Error> {
        let mut sql = String::from("SELECT * FROM reviewables");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if let Some(_) = self.status {
            conditions.push(format!("status = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(ref _rtype) = self.reviewable_type {
            conditions.push(format!("reviewable_type = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.category_id {
            conditions.push(format!("category_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.topic_id {
            conditions.push(format!("topic_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.created_by_id {
            conditions.push(format!("created_by_id = ${}", param_idx));
            param_idx += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            ReviewableOrder::Score => "score DESC",
            ReviewableOrder::Created => "created_at DESC",
            ReviewableOrder::Priority => "score DESC, created_at ASC",
        };
        sql.push_str(&format!(" ORDER BY {}", order_clause));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, Reviewable>(&sql);

        if let Some(status) = self.status {
            query = query.bind(status);
        }
        if let Some(ref rtype) = self.reviewable_type {
            query = query.bind(rtype.clone());
        }
        if let Some(category_id) = self.category_id {
            query = query.bind(category_id);
        }
        if let Some(topic_id) = self.topic_id {
            query = query.bind(topic_id);
        }
        if let Some(created_by_id) = self.created_by_id {
            query = query.bind(created_by_id);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
