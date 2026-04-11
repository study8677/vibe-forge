use crate::models::post::Post;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostListQuery {
    topic_id: Option<i64>,
    user_id: Option<i64>,
    post_type: Option<i32>,
    include_deleted: bool,
    include_hidden: bool,
    order: PostOrder,
    limit: i64,
    offset: i64,
    before_post_number: Option<i32>,
    after_post_number: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum PostOrder {
    PostNumber,
    Created,
    Likes,
}

impl Default for PostListQuery {
    fn default() -> Self {
        Self {
            topic_id: None,
            user_id: None,
            post_type: None,
            include_deleted: false,
            include_hidden: false,
            order: PostOrder::PostNumber,
            limit: 20,
            offset: 0,
            before_post_number: None,
            after_post_number: None,
        }
    }
}

impl PostListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn topic(mut self, topic_id: i64) -> Self {
        self.topic_id = Some(topic_id);
        self
    }

    pub fn user(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn post_type(mut self, post_type: i32) -> Self {
        self.post_type = Some(post_type);
        self
    }

    pub fn include_deleted(mut self, val: bool) -> Self {
        self.include_deleted = val;
        self
    }

    pub fn include_hidden(mut self, val: bool) -> Self {
        self.include_hidden = val;
        self
    }

    pub fn order_by(mut self, order: PostOrder) -> Self {
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

    pub fn before_post(mut self, post_number: i32) -> Self {
        self.before_post_number = Some(post_number);
        self
    }

    pub fn after_post(mut self, post_number: i32) -> Self {
        self.after_post_number = Some(post_number);
        self
    }

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
        let mut sql = String::from("SELECT * FROM posts");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if let Some(_) = self.topic_id {
            conditions.push(format!("topic_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.user_id {
            conditions.push(format!("user_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.post_type {
            conditions.push(format!("post_type = ${}", param_idx));
            param_idx += 1;
        }

        if !self.include_deleted {
            conditions.push("deleted_at IS NULL".to_string());
        }

        if !self.include_hidden {
            conditions.push("hidden = FALSE".to_string());
        }

        if let Some(_) = self.before_post_number {
            conditions.push(format!("post_number < ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.after_post_number {
            conditions.push(format!("post_number > ${}", param_idx));
            param_idx += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            PostOrder::PostNumber => "post_number ASC",
            PostOrder::Created => "created_at DESC",
            PostOrder::Likes => "like_count DESC",
        };
        sql.push_str(&format!(" ORDER BY {}", order_clause));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, Post>(&sql);

        if let Some(topic_id) = self.topic_id {
            query = query.bind(topic_id);
        }
        if let Some(user_id) = self.user_id {
            query = query.bind(user_id);
        }
        if let Some(post_type) = self.post_type {
            query = query.bind(post_type);
        }
        if let Some(before) = self.before_post_number {
            query = query.bind(before);
        }
        if let Some(after) = self.after_post_number {
            query = query.bind(after);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
