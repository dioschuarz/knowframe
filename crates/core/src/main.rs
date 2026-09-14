//! Main entrypoint for llm-wiki-core CLI.

use clap::{Parser, Subcommand};
use llm_wiki_core::cli::baseline_cmd::{execute_baseline_init, execute_baseline_reset};
use llm_wiki_core::cli::delta::execute_git_delta;
use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use llm_wiki_core::cli::issues_cmd::execute_issues;
use llm_wiki_core::cli::validate::execute_validate;
use llm_wiki_core::ipc::server::start_ipc_server;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "llm-wiki-core")]
#[command(
    about = "Rust core engine for LLM Wiki knowledge graph, admission control and context budgeting",
    long_about = None
)]
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

        /// Commit SHA of newly created commit (defaults to HEAD)
        #[arg(long)]
        commit_sha: Option<String>,

        /// Suppress standard output
        #[arg(short, long)]
        quiet: bool,

        /// Hard non-blocking timeout in milliseconds (default 500ms)
        #[arg(long, default_value_t = 500)]
        timeout_ms: u64,

        /// Changed files
        files: Vec<String>,
    },

    /// Evaluates staged changes against candidate graph (pre-commit hook)
    Validate {
        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Evaluate only files staged in Git index
        #[arg(long, default_value_t = true)]
        staged_only: bool,

        /// Suppress informational output
        #[arg(short, long)]
        quiet: bool,

        /// Emit CandidateValidationReport as machine-readable JSON
        #[arg(long)]
        json: bool,

        /// Hard timeout in milliseconds before non-blocking fail-open (default 500ms)
        #[arg(long, default_value_t = 500)]
        timeout_ms: u64,
    },

    /// Trace context discovery path from entrypoint to query topic
    Trace {
        /// Task query or topic to trace
        query: String,

        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Entrypoint note identifier (defaults to AGENTS.md or Hub)
        #[arg(short, long)]
        entrypoint: Option<String>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Compute 4-pillar network-aware knowledge quality score
    Quality {
        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// List detected knowledge issues from last sync, categorized and filterable by pillar
    Issues {
        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Filter issues by pillar (all, integrity, consistency, discoverability, freshness)
        #[arg(short, long, default_value = "all")]
        pillar: String,

        /// Limit maximum number of issues to display
        #[arg(short, long)]
        limit: Option<usize>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Materialize heading fragment metadata into YAML frontmatter
    Materialize {
        /// File or directory path to materialize
        #[arg(index = 1)]
        path: PathBuf,

        /// Automatically stage changes into Git index with git add
        #[arg(long, default_value_t = false)]
        stage: bool,
    },

    /// Manage knowledge debt baseline (.context-vault/baseline.json)
    Baseline {
        #[command(subcommand)]
        subcommand: BaselineCommands,
    },
}


#[derive(Subcommand)]
enum BaselineCommands {
    /// Initialize debt baseline from current repository state
    Init {
        /// Workspace root directory
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Force overwrite of existing baseline
        #[arg(short, long)]
        force: bool,
    },

    /// Reset debt baseline to current repository state
    Reset {
        /// Workspace root directory
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,
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
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
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
            let workspace_root = root
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
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
        Commands::GitDelta {
            root,
            commit_sha,
            quiet,
            timeout_ms,
            files,
        } => {
            let timeout_duration = std::time::Duration::from_millis(timeout_ms);
            let _ = execute_git_delta(
                &root,
                &files,
                commit_sha.as_deref(),
                quiet,
                timeout_duration,
            )
            .await;
        }
        Commands::Validate {
            root,
            path,
            positional_root,
            staged_only,
            quiet,
            json,
            timeout_ms,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            let exit_code =
                execute_validate(&workspace_root, staged_only, quiet, json, timeout_ms).await;
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Commands::Trace {
            query,
            root,
            path,
            entrypoint,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = llm_wiki_core::cli::trace_cmd::execute_trace(
                &workspace_root,
                &query,
                entrypoint.as_deref(),
                json,
            ) {
                eprintln!("Error executing trace: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Quality {
            root,
            path,
            positional_root,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = llm_wiki_core::cli::quality_cmd::execute_quality(&workspace_root, json) {
                eprintln!("Error computing quality score: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Issues {
            root,
            path,
            positional_root,
            pillar,
            limit,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = execute_issues(&workspace_root, &pillar, limit, json) {
                eprintln!("Error retrieving knowledge issues: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Materialize { path, stage } => {
            if let Err(e) = llm_wiki_core::cli::materialize_cmd::execute_materialize(&path, stage) {
                eprintln!("Error materializing frontmatter: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Baseline { subcommand } => match subcommand {
            BaselineCommands::Init {
                root,
                positional_root,
                force,
            } => {
                let workspace_root = root
                    .or(positional_root)
                    .unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = execute_baseline_init(&workspace_root, force) {
                    eprintln!("Error initializing baseline: {}", e);
                    std::process::exit(1);
                }
            }
            BaselineCommands::Reset {
                root,
                positional_root,
            } => {
                let workspace_root = root
                    .or(positional_root)
                    .unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = execute_baseline_reset(&workspace_root) {
                    eprintln!("Error resetting baseline: {}", e);
                    std::process::exit(1);
                }
            }
        },
    }

    Ok(())
}
