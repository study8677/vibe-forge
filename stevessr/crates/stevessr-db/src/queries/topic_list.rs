use crate::models::topic::Topic;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct TopicListQuery {
    category_id: Option<i64>,
    tag_id: Option<i64>,
    user_id: Option<i64>,
    status: Option<TopicStatus>,
    order: TopicOrder,
    no_subcategories: bool,
    exclude_category_ids: Vec<i64>,
    before: Option<String>,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum TopicOrder {
    Default,
    Created,
    Activity,
    Views,
    Posts,
    Likes,
    OpLikes,
    Posters,
}

#[derive(Debug, Clone)]
pub enum TopicStatus {
    Open,
    Closed,
    Archived,
    Listed,
    Unlisted,
    Deleted,
}

impl Default for TopicListQuery {
    fn default() -> Self {
        Self {
            category_id: None,
            tag_id: None,
            user_id: None,
            status: None,
            order: TopicOrder::Default,
            no_subcategories: false,
            exclude_category_ids: Vec::new(),
            before: None,
            limit: 30,
            offset: 0,
        }
    }
}

impl TopicListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category_id: i64) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn tag(mut self, tag_id: i64) -> Self {
        self.tag_id = Some(tag_id);
        self
    }

    pub fn user(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn status(mut self, status: TopicStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn order_by(mut self, order: TopicOrder) -> Self {
        self.order = order;
        self
    }

    pub fn no_subcategories(mut self, val: bool) -> Self {
        self.no_subcategories = val;
        self
    }

    pub fn exclude_categories(mut self, ids: Vec<i64>) -> Self {
        self.exclude_category_ids = ids;
        self
    }

    pub fn before_cursor(mut self, cursor: String) -> Self {
        self.before = Some(cursor);
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Topic>, sqlx::Error> {
        let mut sql = String::from("SELECT t.* FROM topics t");
        let mut conditions: Vec<String> = vec!["t.deleted_at IS NULL".to_string(), "t.visible = TRUE".to_string()];
        let mut param_idx = 1u32;

        if self.tag_id.is_some() {
            sql.push_str(" INNER JOIN topic_tags tt ON tt.topic_id = t.id");
            conditions.push(format!("tt.tag_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.category_id {
            conditions.push(format!("t.category_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.user_id {
            conditions.push(format!("t.user_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(ref status) = self.status {
            match status {
                TopicStatus::Open => conditions.push("t.closed = FALSE AND t.archived = FALSE".to_string()),
                TopicStatus::Closed => conditions.push("t.closed = TRUE".to_string()),
                TopicStatus::Archived => conditions.push("t.archived = TRUE".to_string()),
                TopicStatus::Listed => conditions.push("t.visible = TRUE".to_string()),
                TopicStatus::Unlisted => conditions.push("t.visible = FALSE".to_string()),
                TopicStatus::Deleted => conditions.push("t.deleted_at IS NOT NULL".to_string()),
            }
        }

        if !self.exclude_category_ids.is_empty() {
            let placeholders: Vec<String> = self.exclude_category_ids.iter().enumerate()
                .map(|(i, _)| format!("${}", param_idx + i as u32))
                .collect();
            conditions.push(format!("(t.category_id IS NULL OR t.category_id NOT IN ({}))", placeholders.join(", ")));
            param_idx += self.exclude_category_ids.len() as u32;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            TopicOrder::Default => "t.bumped_at DESC",
            TopicOrder::Created => "t.created_at DESC",
            TopicOrder::Activity => "t.bumped_at DESC",
            TopicOrder::Views => "t.views DESC",
            TopicOrder::Posts => "t.posts_count DESC",
            TopicOrder::Likes => "t.like_count DESC",
            TopicOrder::OpLikes => "t.like_count DESC",
            TopicOrder::Posters => "t.participant_count DESC",
        };
        sql.push_str(&format!(" ORDER BY t.pinned_globally DESC, t.pinned_at IS NOT NULL DESC, {}", order_clause));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, Topic>(&sql);

        if let Some(tag_id) = self.tag_id {
            query = query.bind(tag_id);
        }
        if let Some(category_id) = self.category_id {
            query = query.bind(category_id);
        }
        if let Some(user_id) = self.user_id {
            query = query.bind(user_id);
        }
        for id in &self.exclude_category_ids {
            query = query.bind(*id);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
