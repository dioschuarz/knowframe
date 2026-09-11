//! Main entrypoint for llm-wiki-core CLI.

use clap::{Parser, Subcommand};
use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "llm-wiki-core")]
#[command(about = "Rust core engine for LLM Wiki knowledge graph and context budgeting", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Headless scan of workspace documentation, outputting topology JSON
    Scan {
        /// Workspace root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Override designated Level 0 Hub
        #[arg(long)]
        hub: Option<String>,

        /// Pretty-print JSON output
        #[arg(short, long)]
        pretty: bool,
    },

    /// Run as background WebSocket JSON-RPC daemon
    Daemon {
        /// Workspace root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Port to bind (0 for OS-assigned ephemeral port)
        #[arg(short, long, default_value_t = 0)]
        port: u16,
    },

    /// Process git commit delta notification (invoked by prek/post-commit)
    GitDelta {
        /// Workspace root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Changed files
        files: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { root, hub, pretty } => {
            let options = ScanOptions {
                workspace_root: root,
                hub,
                pretty,
            };
            let result = execute_scan(options)?;
            if pretty {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("{}", serde_json::to_string(&result)?);
            }
        }
        Commands::Daemon { root, port } => {
            println!("llm-wiki-core daemon starting on root {:?} port {}...", root, port);
            // Will be fully implemented in Phase 4 (US2)
        }
        Commands::GitDelta { root, files } => {
            println!("Processing git delta for {:?} files in {:?}", files.len(), root);
            // Will be fully implemented in Phase 6 (US4)
        }
    }

    Ok(())
}
