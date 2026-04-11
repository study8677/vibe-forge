use redis::aio::ConnectionManager;
use redis::AsyncCommands;

/// Cache for anonymous (logged-out) page views.
/// Equivalent to Discourse's anonymous cache which serves pre-rendered pages
/// to non-authenticated users.
pub struct AnonymousCache {
    redis: ConnectionManager,
    ttl_secs: u64,
}

impl AnonymousCache {
    pub fn new(redis: ConnectionManager, ttl_secs: u64) -> Self {
        Self { redis, ttl_secs }
    }

    fn key(path: &str) -> String {
        format!("stevessr:anon_cache:{}", path)
    }

    pub async fn get(&self, path: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.get(Self::key(path)).await
    }

    pub async fn set(&self, path: &str, body: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.set_ex(Self::key(path), body, self.ttl_secs).await
    }

    pub async fn invalidate(&self, path: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        conn.del(Self::key(path)).await
    }
}
