use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::time::Duration;

/// A simple distributed mutex using Redis SET NX EX.
pub struct DistributedMutex {
    redis: ConnectionManager,
}

impl DistributedMutex {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Try to acquire a lock. Returns true if the lock was acquired.
    pub async fn try_lock(&self, name: &str, ttl: Duration) -> Result<bool, redis::RedisError> {
        let key = format!("stevessr:lock:{}", name);
        let mut conn = self.redis.clone();
        let result: Option<String> = redis::cmd("SET")
            .arg(&key)
            .arg("1")
            .arg("NX")
            .arg("EX")
            .arg(ttl.as_secs())
            .query_async(&mut conn)
            .await?;
        Ok(result.is_some())
    }

    /// Release a lock.
    pub async fn unlock(&self, name: &str) -> Result<(), redis::RedisError> {
        let key = format!("stevessr:lock:{}", name);
        let mut conn = self.redis.clone();
        conn.del(key).await
    }
}
