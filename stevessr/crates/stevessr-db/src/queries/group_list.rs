use crate::models::group::Group;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct GroupListQuery {
    visibility_level: Option<i32>,
    include_automatic: bool,
    user_id: Option<i64>,
    term: Option<String>,
    order: GroupOrder,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum GroupOrder {
    Name,
    UserCount,
    Created,
}

impl Default for GroupListQuery {
    fn default() -> Self {
        Self {
            visibility_level: None,
            include_automatic: true,
            user_id: None,
            term: None,
            order: GroupOrder::Name,
            limit: 36,
            offset: 0,
        }
    }
}

impl GroupListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn visibility(mut self, level: i32) -> Self {
        self.visibility_level = Some(level);
        self
    }

    pub fn exclude_automatic(mut self) -> Self {
        self.include_automatic = false;
        self
    }

    pub fn for_user(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn search(mut self, term: &str) -> Self {
        self.term = Some(term.to_string());
        self
    }

    pub fn order_by(mut self, order: GroupOrder) -> Self {
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Group>, sqlx::Error> {
        let mut sql = String::from("SELECT g.* FROM groups g");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if self.user_id.is_some() {
            sql.push_str(" INNER JOIN group_users gu ON gu.group_id = g.id");
            conditions.push(format!("gu.user_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.visibility_level {
            conditions.push(format!("g.visibility_level <= ${}", param_idx));
            param_idx += 1;
        }

        if !self.include_automatic {
            conditions.push("g.automatic = FALSE".to_string());
        }

        if let Some(ref _term) = self.term {
            conditions.push(format!("(g.name ILIKE ${param} OR g.full_name ILIKE ${param})", param = param_idx));
            param_idx += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            GroupOrder::Name => "g.name ASC",
            GroupOrder::UserCount => "g.user_count DESC",
            GroupOrder::Created => "g.created_at DESC",
        };
        sql.push_str(&format!(" ORDER BY {}", order_clause));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, Group>(&sql);

        if let Some(user_id) = self.user_id {
            query = query.bind(user_id);
        }
        if let Some(visibility_level) = self.visibility_level {
            query = query.bind(visibility_level);
        }
        if let Some(ref term) = self.term {
            let pattern = format!("%{}%", term);
            query = query.bind(pattern);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
