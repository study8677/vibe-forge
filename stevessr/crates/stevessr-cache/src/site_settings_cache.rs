use redis::aio::ConnectionManager;
use redis::AsyncCommands;

const CACHE_KEY: &str = "stevessr:site_settings";

pub struct SiteSettingsCache {
    redis: ConnectionManager,
}

impl SiteSettingsCache {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.hget(CACHE_KEY, key).await
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.hset(CACHE_KEY, key, value).await
    }

    pub async fn get_all(&self) -> Result<std::collections::HashMap<String, String>, redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.hgetall(CACHE_KEY).await
    }

    pub async fn invalidate(&self) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.del(CACHE_KEY).await
    }
}
