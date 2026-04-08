use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use line_history_core::{resolve_history_step, HistoryStep, ResolveRequest};

#[derive(Debug, Parser)]
#[command(name = "zed-lines-history-cli")]
#[command(about = "Prototype CLI for recursive Git line history traversal")]
struct Args {
    #[arg(long)]
    repo: PathBuf,
    #[arg(long, default_value = "HEAD")]
    rev: String,
    #[arg(long)]
    path: PathBuf,
    #[arg(long)]
    line: u32,
    #[arg(long, default_value_t = 2)]
    context_before: u32,
    #[arg(long, default_value_t = 2)]
    context_after: u32,
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() {
    let args = Args::parse();
    let request = ResolveRequest {
        repo_root: args.repo,
        rev: args.rev,
        path: args.path,
        line: args.line,
        context_before: args.context_before,
        context_after: args.context_after,
    };

    match resolve_history_step(&request) {
        Ok(step) => match args.format {
            OutputFormat::Text => print!("{}", render_text(&step)),
            OutputFormat::Json => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&step).expect("serialize history step")
                );
            }
        },
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}

fn render_text(step: &HistoryStep) -> String {
    let mut output = String::new();
    output.push_str("Current\n");
    output.push_str(&format!(
        "  commit: {}\n  path: {}\n  line: {}\n",
        step.current.commit_id,
        step.current.file_path.display(),
        step.current.line_range.start
    ));
    output.push_str("  snippet:\n");
    for line in &step.current.snippet {
        output.push_str(&format!("    {:>4} | {}\n", line.number, line.text));
    }

    output.push_str("Previous\n");
    output.push_str(&format!(
        "  commit: {}\n  path: {}\n  line: {}\n",
        step.previous.commit_id,
        step.previous.file_path.display(),
        step.previous.line_range.start
    ));
    output.push_str("  snippet:\n");
    for line in &step.previous.snippet {
        output.push_str(&format!("    {:>4} | {}\n", line.number, line.text));
    }

    output.push_str(&format!(
        "Mapping confidence: {:?}\n",
        step.mapping_confidence
    ));
    output
}
