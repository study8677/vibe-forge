use crate::models::user::User;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct UserSearchQuery {
    term: Option<String>,
    group_id: Option<i64>,
    trust_level: Option<i16>,
    include_staged: bool,
    include_inactive: bool,
    admin_only: bool,
    moderator_only: bool,
    order: UserOrder,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum UserOrder {
    Username,
    Created,
    LastSeen,
    TrustLevel,
    PostCount,
    LikesReceived,
    DaysVisited,
}

impl Default for UserSearchQuery {
    fn default() -> Self {
        Self {
            term: None,
            group_id: None,
            trust_level: None,
            include_staged: false,
            include_inactive: false,
            admin_only: false,
            moderator_only: false,
            order: UserOrder::Username,
            limit: 50,
            offset: 0,
        }
    }
}

impl UserSearchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn term(mut self, term: &str) -> Self {
        self.term = Some(term.to_string());
        self
    }

    pub fn group(mut self, group_id: i64) -> Self {
        self.group_id = Some(group_id);
        self
    }

    pub fn trust_level(mut self, level: i16) -> Self {
        self.trust_level = Some(level);
        self
    }

    pub fn include_staged(mut self, val: bool) -> Self {
        self.include_staged = val;
        self
    }

    pub fn include_inactive(mut self, val: bool) -> Self {
        self.include_inactive = val;
        self
    }

    pub fn admin_only(mut self) -> Self {
        self.admin_only = true;
        self
    }

    pub fn moderator_only(mut self) -> Self {
        self.moderator_only = true;
        self
    }

    pub fn order_by(mut self, order: UserOrder) -> Self {
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let mut sql = String::from("SELECT u.* FROM users u");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if self.group_id.is_some() {
            sql.push_str(" INNER JOIN group_users gu ON gu.user_id = u.id");
            conditions.push(format!("gu.group_id = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(ref _term) = self.term {
            conditions.push(format!(
                "(u.username_lower LIKE ${param} OR u.name ILIKE ${param})",
                param = param_idx
            ));
            param_idx += 1;
        }

        if let Some(_) = self.trust_level {
            conditions.push(format!("u.trust_level = ${}", param_idx));
            param_idx += 1;
        }

        if !self.include_staged {
            conditions.push("u.staged = FALSE".to_string());
        }

        if !self.include_inactive {
            conditions.push("u.active = TRUE".to_string());
        }

        if self.admin_only {
            conditions.push("u.admin = TRUE".to_string());
        }

        if self.moderator_only {
            conditions.push("u.moderator = TRUE".to_string());
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        let order_clause = match self.order {
            UserOrder::Username => "u.username_lower ASC",
            UserOrder::Created => "u.created_at DESC",
            UserOrder::LastSeen => "u.last_seen_at DESC NULLS LAST",
            UserOrder::TrustLevel => "u.trust_level DESC",
            UserOrder::PostCount => "u.views DESC",
            UserOrder::LikesReceived => "u.views DESC",
            UserOrder::DaysVisited => "u.views DESC",
        };
        sql.push_str(&format!(" ORDER BY {}", order_clause));
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, User>(&sql);

        if let Some(group_id) = self.group_id {
            query = query.bind(group_id);
        }
        if let Some(ref term) = self.term {
            let pattern = format!("%{}%", term.to_lowercase());
            query = query.bind(pattern);
        }
        if let Some(trust_level) = self.trust_level {
            query = query.bind(trust_level);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
