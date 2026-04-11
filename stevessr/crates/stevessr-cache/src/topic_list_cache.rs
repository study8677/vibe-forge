use redis::aio::ConnectionManager;
use redis::AsyncCommands;

pub struct TopicListCache {
    redis: ConnectionManager,
    ttl_secs: u64,
}

impl TopicListCache {
    pub fn new(redis: ConnectionManager, ttl_secs: u64) -> Self {
        Self { redis, ttl_secs }
    }

    fn key(category_id: Option<i64>, filter: &str, page: u32) -> String {
        match category_id {
            Some(id) => format!("stevessr:topic_list:{}:{}:{}", id, filter, page),
            None => format!("stevessr:topic_list:all:{}:{}", filter, page),
        }
    }

    pub async fn get(&self, category_id: Option<i64>, filter: &str, page: u32) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.get(Self::key(category_id, filter, page)).await
    }

    pub async fn set(&self, category_id: Option<i64>, filter: &str, page: u32, json: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.set_ex(Self::key(category_id, filter, page), json, self.ttl_secs).await
    }

    pub async fn invalidate_all(&self) -> Result<(), redis::RedisError> {
        // In production, use SCAN + DEL for the prefix pattern
        // For now, this is a placeholder
        let _ = &self.redis;
        Ok(())
    }
}
