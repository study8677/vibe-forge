use redis::aio::ConnectionManager;
use redis::AsyncCommands;

pub struct UserCache {
    redis: ConnectionManager,
    ttl_secs: u64,
}

impl UserCache {
    pub fn new(redis: ConnectionManager, ttl_secs: u64) -> Self {
        Self { redis, ttl_secs }
    }

    fn key(user_id: i64) -> String {
        format!("stevessr:user:{}", user_id)
    }

    pub async fn get(&self, user_id: i64) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.get(Self::key(user_id)).await
    }

    pub async fn set(&self, user_id: i64, json: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.set_ex(Self::key(user_id), json, self.ttl_secs).await
    }

    pub async fn invalidate(&self, user_id: i64) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.del(Self::key(user_id)).await
    }
}
