use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "media-station", version, about = "Lightweight LAN Media Station")]
pub struct Config {
    /// Media directories to scan (can specify multiple)
    #[arg(short = 'd', long = "dir", required = true)]
    pub media_dirs: Vec<PathBuf>,

    /// Data directory for database and thumbnails
    #[arg(long, default_value = "./ms-data")]
    pub data_dir: PathBuf,

    /// Web server bind address
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    /// Web server port
    #[arg(short, long, default_value_t = 9110)]
    pub port: u16,

    /// Number of thumbnail worker threads (0 = number of CPUs)
    #[arg(long, default_value_t = 4)]
    pub thumb_workers: usize,
}
