use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "stevessr-cli", about = "Stevessr administration CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run database migrations
    Migrate,
    /// Create a database backup
    Backup {
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Restore from a backup
    Restore {
        #[arg(short, long)]
        input: String,
    },
    /// Reindex search
    Reindex,
    /// Create an admin user
    CreateAdmin {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    let config = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build()?;
    let app_config: stevessr_core::config::AppConfig = config.try_deserialize()?;

    let pool = stevessr_db::create_pool(
        &app_config.database.url,
        app_config.database.max_connections,
        app_config.database.min_connections,
        app_config.database.connect_timeout_secs,
        app_config.database.idle_timeout_secs,
    ).await?;

    match cli.command {
        Commands::Migrate => {
            sqlx::migrate!("../../migrations").run(&pool).await?;
            println!("migrations applied successfully");
        }
        Commands::Backup { output } => {
            let filename = output.unwrap_or_else(|| format!("backup_{}.sql", chrono::Utc::now().format("%Y%m%d_%H%M%S")));
            println!("creating backup: {}", filename);
            // TODO: pg_dump equivalent
        }
        Commands::Restore { input } => {
            println!("restoring from: {}", input);
            // TODO: pg_restore equivalent
        }
        Commands::Reindex => {
            println!("reindexing search...");
            // TODO: rebuild tantivy index
        }
        Commands::CreateAdmin { username, email, password } => {
            println!("creating admin user: {}", username);
            let user = stevessr_db::models::user::User::create(&pool, &username, None, true, 4, None).await?;
            stevessr_db::models::user_email::UserEmail::create(&pool, user.id, &email, true).await?;
            stevessr_db::models::user::User::set_admin(&pool, user.id, true).await?;
            // TODO: hash and store password
            let _ = password;
            println!("admin user created: id={}", user.id);
        }
    }

    Ok(())
}
