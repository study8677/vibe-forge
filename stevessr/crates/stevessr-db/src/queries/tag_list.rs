use crate::models::tag::Tag;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct TagListQuery {
    filter: Option<String>,
    category_id: Option<i64>,
    tag_group_id: Option<i64>,
    order: TagOrder,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum TagOrder {
    Name,
    TopicCount,
}

impl Default for TagListQuery {
    fn default() -> Self {
        Self {
            filter: None,
            category_id: None,
            tag_group_id: None,
            order: TagOrder::TopicCount,
            limit: 100,
            offset: 0,
        }
    }
}

impl TagListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_string());
        self
    }

    pub fn category(mut self, category_id: i64) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn tag_group(mut self, tag_group_id: i64) -> Self {
        self.tag_group_id = Some(tag_group_id);
        self
    }

    pub fn order_by(mut self, order: TagOrder) -> Self {
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Tag>, sqlx::Error> {
        let mut sql = String::from("SELECT t.* FROM tags t");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if self.tag_group_id.is_some() {
            sql.push_str(" INNER JOIN tag_group_memberships tgm ON tgm.tag_id = t.id");
            conditions.push(format!("tgm.tag_group_id = ${}", param_idx));
            param_idx += 1;
        }

        if self.category_id.is_some() {
            sql.push_str(" INNER JOIN category_tags ct ON ct.tag_id = t.id");
            conditions.push(format!("ct.category_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(ref _filter) = self.filter {
            conditions.push(format!("LOWER(t.name) LIKE ${}", param_idx));
            param_idx += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            TagOrder::Name => "t.name ASC",
            TagOrder::TopicCount => "t.topic_count DESC",
        };
        sql.push_str(&format!(" ORDER BY {}", order_clause));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, Tag>(&sql);

        if let Some(tag_group_id) = self.tag_group_id {
            query = query.bind(tag_group_id);
        }
        if let Some(category_id) = self.category_id {
            query = query.bind(category_id);
        }
        if let Some(ref filter) = self.filter {
            let pattern = format!("%{}%", filter.to_lowercase());
            query = query.bind(pattern);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
