use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "stevessr=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("starting stevessr server");

    // Load configuration
    let config = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name(&format!("config/{}", std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into()))).required(false))
        .add_source(config::Environment::with_prefix("STEVESSR"))
        .build()?;

    let app_config: stevessr_core::config::AppConfig = config.try_deserialize()?;
    let bind_addr = app_config.server.bind.clone();

    // Create database pool
    let db_pool = stevessr_db::create_pool(
        &app_config.database.url,
        app_config.database.max_connections,
        app_config.database.min_connections,
        app_config.database.connect_timeout_secs,
        app_config.database.idle_timeout_secs,
    ).await?;

    tracing::info!("database pool created");

    // Run migrations
    sqlx::migrate!("../../migrations")
        .run(&db_pool)
        .await?;

    tracing::info!("database migrations applied");

    // Create Redis connection
    let redis = stevessr_cache::connection::create_redis_pool(&app_config.redis.url).await?;
    tracing::info!("redis connection established");

    // Load plugins
    let plugin_registry = stevessr_plugin_host::PluginRegistry::new();
    if app_config.plugins.enabled {
        let plugin_dir = std::path::Path::new(&app_config.plugins.directory);
        match plugin_registry.discover_and_load(plugin_dir).await {
            Ok(loaded) => tracing::info!("loaded {} plugins: {:?}", loaded.len(), loaded),
            Err(e) => tracing::error!("plugin loading error: {}", e),
        }
    }

    // Build application state
    let state = stevessr_api::state::AppState {
        db: db_pool.clone(),
        redis: redis.clone(),
        config: std::sync::Arc::new(app_config.clone()),
    };

    // Build router
    let app = stevessr_api::app::build_router(state);

    // Start background job runner
    let job_redis = redis.clone();
    tokio::spawn(async move {
        let runner = stevessr_jobs::JobRunner::new(job_redis);
        runner.run_loop().await;
    });

    // Start server
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!("listening on {}", bind_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("server stopped");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    tracing::info!("shutdown signal received");
}
