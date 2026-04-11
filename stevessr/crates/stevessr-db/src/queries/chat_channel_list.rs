use crate::models::chat_channel::ChatChannel;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct ChatChannelListQuery {
    user_id: Option<i64>,
    status: Option<i32>,
    channel_type: Option<ChannelType>,
    filter: Option<String>,
    limit: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
pub enum ChannelType {
    Public,
    DirectMessage,
}

impl Default for ChatChannelListQuery {
    fn default() -> Self {
        Self {
            user_id: None,
            status: Some(0), // open
            channel_type: None,
            filter: None,
            limit: 50,
            offset: 0,
        }
    }
}

impl ChatChannelListQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_user(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    pub fn channel_type(mut self, channel_type: ChannelType) -> Self {
        self.channel_type = Some(channel_type);
        self
    }

    pub fn filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_string());
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

    pub async fn execute(self, pool: &PgPool) -> Result<Vec<ChatChannel>, sqlx::Error> {
        let mut sql = String::from("SELECT cc.* FROM chat_channels cc");
        let mut conditions: Vec<String> = Vec::new();
        let mut param_idx = 1u32;

        if self.user_id.is_some() {
            sql.push_str(" INNER JOIN chat_memberships cm ON cm.chat_channel_id = cc.id");
            conditions.push(format!("cm.user_id = ${}", param_idx));
            conditions.push("cm.following = TRUE".to_string());
            param_idx += 1;
        }

        if let Some(_) = self.status {
            conditions.push(format!("cc.status = ${}", param_idx));
            param_idx += 1;
        }

        if let Some(ref channel_type) = self.channel_type {
            let type_str = match channel_type {
                ChannelType::Public => "Category",
                ChannelType::DirectMessage => "DirectMessage",
            };
            conditions.push(format!("cc.chatable_type = ${}", param_idx));
            param_idx += 1;
            // type_str will be bound below
            let _ = type_str;
        }

        if let Some(ref _filter) = self.filter {
            conditions.push(format!("cc.name ILIKE ${}", param_idx));
            param_idx += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

        sql.push_str(" ORDER BY cc.updated_at DESC");
        sql.push_str(&format!(" LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        let mut query = sqlx::query_as::<_, ChatChannel>(&sql);

        if let Some(user_id) = self.user_id {
            query = query.bind(user_id);
        }
        if let Some(status) = self.status {
            query = query.bind(status);
        }
        if let Some(ref channel_type) = self.channel_type {
            let type_str = match channel_type {
                ChannelType::Public => "Category",
                ChannelType::DirectMessage => "DirectMessage",
            };
            query = query.bind(type_str);
        }
        if let Some(ref filter) = self.filter {
            let pattern = format!("%{}%", filter);
            query = query.bind(pattern);
        }
        query = query.bind(self.limit);
        query = query.bind(self.offset);

        query.fetch_all(pool).await
    }
}
