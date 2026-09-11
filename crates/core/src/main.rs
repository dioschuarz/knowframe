//! Main entrypoint for llm-wiki-core CLI.

use clap::{Parser, Subcommand};
use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use llm_wiki_core::ipc::server::start_ipc_server;
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
        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Override designated Level 0 Hub
        #[arg(long)]
        hub: Option<String>,

        /// Pretty-print JSON output
        #[arg(short, long)]
        pretty: bool,
    },

    /// Run as background WebSocket JSON-RPC daemon
    Daemon {
        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

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
        Commands::Scan {
            root,
            path,
            positional_root,
            hub,
            pretty,
        } => {
            let workspace_root = path.or(root).or(positional_root).unwrap_or_else(|| PathBuf::from("."));
            let options = ScanOptions {
                workspace_root,
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
        Commands::Daemon {
            root,
            positional_root,
            port,
        } => {
            let workspace_root = root.or(positional_root).unwrap_or_else(|| PathBuf::from("."));
            let auth_token = std::env::var("LLM_WIKI_AUTH_TOKEN").ok();
            let (addr, _shutdown_tx) = start_ipc_server(port, workspace_root, auth_token).await?;
            println!(
                "{}",
                serde_json::json!({
                    "event": "daemon_ready",
                    "port": addr.port(),
                    "address": addr.to_string()
                })
            );
            use std::io::Write;
            let _ = std::io::stdout().flush();
            // Wait for termination signal
            tokio::signal::ctrl_c().await?;
        }
        Commands::GitDelta { root, files } => {
            let timeout_duration = std::time::Duration::from_millis(500);
            let _ = llm_wiki_core::cli::delta::execute_git_delta(&root, &files, timeout_duration).await;
        }
    }

    Ok(())
}
