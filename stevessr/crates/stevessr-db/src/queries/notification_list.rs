use crate::models::notification::Notification;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct NotificationListQuery {
    user_id: i64,
    notification_type: Option<i32>,
    read: Option<bool>,
    high_priority_only: bool,
    limit: i64,
    offset: i64,
}

impl NotificationListQuery {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            notification_type: None,
            read: None,
            high_priority_only: false,
            limit: 60,
            offset: 0,
        }
    }

    pub fn notification_type(mut self, notification_type: i32) -> Self {
        self.notification_type = Some(notification_type);
        self
    }

    pub fn unread_only(mut self) -> Self {
        self.read = Some(false);
        self
    }

    pub fn read_only(mut self) -> Self {
        self.read = Some(true);
        self
    }

    pub fn high_priority_only(mut self) -> Self {
        self.high_priority_only = true;
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<Notification>, sqlx::Error> {
        let mut sql = String::from("SELECT * FROM notifications");
        let mut conditions: Vec<String> = vec![format!("user_id = $1")];
        let mut param_idx = 2u32;

        if let Some(_) = self.notification_type {
            conditions.push(format!("notification_type = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(_) = self.read {
            conditions.push(format!("read = ${}", param_idx));
            param_idx += 1;
        }

        if self.high_priority_only {
            conditions.push("high_priority = TRUE".to_string());
        }

        sql.push_str(" WHERE ");
        sql.push_str(&conditions.join(" AND "));
        sql.push_str(" ORDER BY high_priority DESC, created_at DESC");
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, Notification>(&sql);
        query = query.bind(self.user_id);

        if let Some(notification_type) = self.notification_type {
            query = query.bind(notification_type);
        }
        if let Some(read) = self.read {
            query = query.bind(read);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
