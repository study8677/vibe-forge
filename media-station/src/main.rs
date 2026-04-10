mod api;
mod config;
mod db;
mod exif_parser;
mod models;
mod scanner;
mod thumbnail;

use clap::Parser;
use config::Config;
use db::Database;
use scanner::{ScanProgress, start_scan};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("media_station=info")),
        )
        .init();

    let config = Config::parse();

    // Validate media directories
    for d in &config.media_dirs {
        if !d.is_dir() {
            tracing::warn!("Media directory does not exist: {}", d.display());
        }
    }

    // Create data directory
    std::fs::create_dir_all(&config.data_dir).expect("Failed to create data directory");

    // Open database
    let db = Arc::new(Database::open(&config.data_dir).expect("Failed to open database"));

    // Shared scan progress
    let progress = Arc::new(ScanProgress::new());

    // Start initial scan
    start_scan(db.clone(), config.media_dirs.clone(), progress.clone());

    // Start thumbnail workers
    {
        let db = db.clone();
        let data_dir = config.data_dir.clone();
        let workers = config.thumb_workers;
        tokio::spawn(async move {
            thumbnail::worker_loop(db, data_dir, workers).await;
        });
    }

    // Build app state
    let state = Arc::new(api::AppState {
        db,
        progress,
        config: config.clone(),
    });

    let app = api::router(state);
    let addr = format!("{}:{}", config.host, config.port);

    info!("--------------------------------------------");
    info!("  Media Station v{}", env!("CARGO_PKG_VERSION"));
    info!("  http://{addr}");
    info!("  Media dirs: {:?}", config.media_dirs);
    info!("  Data dir:   {}", config.data_dir.display());
    info!("--------------------------------------------");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind {addr}: {e}"));

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
