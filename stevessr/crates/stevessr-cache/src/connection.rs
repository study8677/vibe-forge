use redis::aio::ConnectionManager;

pub type RedisPool = ConnectionManager;

pub async fn create_redis_pool(url: &str) -> Result<RedisPool, redis::RedisError> {
    let client = redis::Client::open(url)?;
    client.get_connection_manager().await
}
