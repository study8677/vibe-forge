use crate::models::directory_item::DirectoryItem;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct DirectoryQuery {
    period_type: i32,
    order: DirectoryOrder,
    asc: bool,
    name_filter: Option<String>,
    group_id: Option<i64>,
    exclude_usernames: Vec<String>,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum DirectoryOrder {
    LikesReceived,
    LikesGiven,
    TopicCount,
    PostCount,
    DaysVisited,
    TopicsEntered,
    PostsRead,
}

impl Default for DirectoryQuery {
    fn default() -> Self {
        Self {
            period_type: 1, // weekly
            order: DirectoryOrder::LikesReceived,
            asc: false,
            name_filter: None,
            group_id: None,
            exclude_usernames: Vec::new(),
            limit: 50,
            offset: 0,
        }
    }
}

impl DirectoryQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn period(mut self, period_type: i32) -> Self {
        self.period_type = period_type;
        self
    }

    pub fn order_by(mut self, order: DirectoryOrder) -> Self {
        self.order = order;
        self
    }

    pub fn ascending(mut self) -> Self {
        self.asc = true;
        self
    }

    pub fn name_filter(mut self, filter: &str) -> Self {
        self.name_filter = Some(filter.to_string());
        self
    }

    pub fn group(mut self, group_id: i64) -> Self {
        self.group_id = Some(group_id);
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<DirectoryItem>, sqlx::Error> {
        let mut sql = String::from("SELECT di.* FROM directory_items di INNER JOIN users u ON u.id = di.user_id");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        conditions.push(format!("di.period_type = ${}", param_idx));
        param_idx += 1;

        conditions.push("u.active = TRUE".to_string());
        conditions.push("u.staged = FALSE".to_string());

        if self.group_id.is_some() {
            sql.push_str(" INNER JOIN group_users gu ON gu.user_id = di.user_id");
            conditions.push(format!("gu.group_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(ref _filter) = self.name_filter {
            conditions.push(format!("(u.username_lower LIKE ${param} OR u.name ILIKE ${param})", param = param_idx));
            param_idx += 1;
        }

        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));

        let order_col = match self.order {
            DirectoryOrder::LikesReceived => "di.likes_received",
            DirectoryOrder::LikesGiven => "di.likes_given",
            DirectoryOrder::TopicCount => "di.topic_count",
            DirectoryOrder::PostCount => "di.post_count",
            DirectoryOrder::DaysVisited => "di.days_visited",
            DirectoryOrder::TopicsEntered => "di.topics_entered",
            DirectoryOrder::PostsRead => "di.posts_read",
        };
        let direction = if self.asc { "ASC" } else { "DESC" };
        sql.push_str(&format!(" ORDER BY {} {}", order_col, direction));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, DirectoryItem>(&sql);

        query = query.bind(self.period_type);
        if let Some(group_id) = self.group_id {
            query = query.bind(group_id);
        }
        if let Some(ref filter) = self.name_filter {
            let pattern = format!("%{}%", filter.to_lowercase());
            query = query.bind(pattern);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
