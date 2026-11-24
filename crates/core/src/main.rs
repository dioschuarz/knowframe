//! Main entrypoint for knowframe-core CLI.

use clap::{Parser, Subcommand};
use llm_wiki_core::cli::baseline_cmd::{execute_baseline_init, execute_baseline_reset};
use llm_wiki_core::cli::delta::execute_git_delta;
use llm_wiki_core::cli::governance_cmd::{
    execute_governance_apply, execute_governance_plan, execute_sync_harnesses,
};
use llm_wiki_core::cli::method_evolution::{
    execute_apply, execute_plan, execute_profile, execute_suggestions,
};
use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use llm_wiki_core::cli::validate::execute_validate_with_paths;
use llm_wiki_core::ipc::server::start_ipc_server;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "knowframe-core")]
#[command(
    about = "Rust core engine for KnowFrame knowledge graph, admission control and context budgeting",
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

        /// Specific files to evaluate instead of git index
        #[arg(long, value_delimiter = ' ')]
        paths: Option<Vec<String>>,
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

        /// Show raw historical audit score instead of effective score
        #[arg(long, alias = "audit-all")]
        raw: bool,

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

        /// Filter issues by pillar (all, integrity, structure, consistency, discoverability, freshness)
        #[arg(short, long, default_value = "all")]
        pillar: String,

        /// Include acknowledged legacy debt from baseline
        #[arg(short = 'd', long = "include-debt", alias = "all")]
        include_debt: bool,

        /// Show compact summary grouped by directory
        #[arg(short = 's', long)]
        summary: bool,

        /// Filter issues by file path prefix (e.g. --filter-path specs/)
        #[arg(long = "filter-path", alias = "prefix")]
        filter_path: Option<String>,

        /// Check excluded paths also (normally ignored)
        #[arg(long = "include-excluded")]
        include_excluded: bool,

        /// Limit maximum number of issues to display
        #[arg(short, long)]
        limit: Option<usize>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Configure profiles, review suggestions, generate plans, and apply method evolution
    #[command(name = "method-evolution")]
    MethodEvolution {
        #[command(subcommand)]
        subcommand: MethodEvolutionCommands,
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

    /// Review governance plans, diff previews, apply transformations, and synchronize AI harnesses
    Governance {
        #[command(subcommand)]
        subcommand: GovernanceCommands,
    },

    /// Trace code-to-doc reverse impact for changed files or Git diff range
    #[command(name = "trace-impact")]
    TraceImpact {
        /// Explicit code file paths to evaluate
        #[arg(long, value_delimiter = ' ')]
        paths: Option<Vec<String>>,

        /// Git diff range (e.g. A..B or base...head)
        #[arg(long)]
        range: Option<String>,

        /// Evaluate files staged in Git index
        #[arg(long)]
        staged: bool,

        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Emit AffectedDocumentsReport as machine-readable JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage documentation review receipts (.context-vault/reviews.json)
    Review {
        #[command(subcommand)]
        subcommand: ReviewCommands,
    },

    /// Convert valid non-clickable links (wikilinks, missing .md, non-relative paths, unslugified anchors)
    /// into universally clickable standard Markdown relative links `[Label](relative/path.md#slug)`
    #[command(name = "fix-links")]
    FixLinks {
        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,

        /// Restrict normalization to a single file path (relative to root or absolute)
        #[arg(short, long)]
        file: Option<String>,

        /// Apply rewrites to disk (defaults to dry-run preview with unified diff)
        #[arg(long, default_value_t = false)]
        apply: bool,

        /// Emit machine-readable JSON report
        #[arg(long, default_value_t = false)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ReviewCommands {
    /// Resolve a review obligation by recording an approval receipt
    Resolve {
        /// Document ID to mark as reviewed unchanged
        #[arg(index = 1)]
        doc_id: String,

        /// Optional Git commit SHA or branch ref (defaults to HEAD)
        #[arg(long)]
        commit_ref: Option<String>,

        /// Workspace root directory (flag)
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Workspace root directory (path flag alias)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Positional workspace root directory
        #[arg(index = 2)]
        positional_root: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum GovernanceCommands {
    /// Generate and preview a governance transformation plan with standard unified diff
    Plan {
        /// Governance strategy (e.g. sync_harness_routes, eliminate_secondary_routing_layer)
        #[arg(short, long)]
        strategy: String,

        /// Target file or directory path
        #[arg(short, long)]
        target: String,

        /// Workspace root directory
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Apply a governance transformation plan after explicit user approval (Invariant XVI)
    Apply {
        /// Governance strategy (e.g. sync_harness_routes, eliminate_secondary_routing_layer)
        #[arg(short, long)]
        strategy: String,

        /// Target file or directory path
        #[arg(short, long)]
        target: String,

        /// Workspace root directory
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Explicit approval to mutate files on disk (Constitutional Invariant XVI)
        #[arg(long)]
        approve: bool,
    },
    /// Synchronize AI harness routes within peer directory scope
    SyncHarnesses {
        /// Target harness file (defaults to AGENTS.md)
        #[arg(index = 1)]
        target: Option<String>,

        /// Workspace root directory
        #[arg(short, long)]
        root: Option<PathBuf>,

        /// Apply changes immediately after review (without this, only diff preview is shown)
        #[arg(long)]
        approve: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum MethodEvolutionCommands {
    /// Show or save one of the four profiles: ai-oriented, human-oriented, balanced, custom
    Profile {
        #[arg(short, long)]
        root: Option<PathBuf>,
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,
        #[arg(long)]
        preset: Option<String>,
        #[arg(long = "set")]
        settings: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    /// List ranked evolution method suggestions
    Suggestions {
        #[arg(short, long)]
        root: Option<PathBuf>,
        #[arg(index = 1)]
        positional_root: Option<PathBuf>,
        #[arg(short, long)]
        limit: Option<usize>,
        #[arg(long)]
        json: bool,
    },
    /// Generate a transformation plan from a recommendation ID
    Plan {
        recommendation_id: String,
        #[arg(short, long)]
        root: Option<PathBuf>,
        #[arg(index = 2)]
        positional_root: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Apply a recommendation after explicit approval
    Apply {
        recommendation_id: String,
        #[arg(short, long)]
        root: Option<PathBuf>,
        #[arg(index = 2)]
        positional_root: Option<PathBuf>,
        #[arg(long)]
        approve: bool,
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
            paths,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            let exit_code =
                execute_validate_with_paths(&workspace_root, paths, staged_only, quiet, json, timeout_ms).await;
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
            let workspace_root = path.or(root).unwrap_or_else(|| PathBuf::from("."));
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
            raw,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = llm_wiki_core::cli::quality_cmd::execute_quality_with_options(
                &workspace_root,
                raw,
                json,
            ) {
                eprintln!("Error computing quality score: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Issues {
            root,
            path,
            positional_root,
            pillar,
            include_debt,
            summary,
            filter_path,
            include_excluded,
            limit,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            let options = llm_wiki_core::cli::issues_cmd::IssuesOptions {
                pillar,
                limit,
                json,
                include_debt,
                summary,
                filter_path,
                include_excluded,
            };
            if let Err(e) =
                llm_wiki_core::cli::issues_cmd::execute_issues_with_options(&workspace_root, options)
            {
                eprintln!("Error retrieving knowledge issues: {}", e);
                std::process::exit(1);
            }
        }
        Commands::MethodEvolution { subcommand } => {
            let result = match subcommand {
                MethodEvolutionCommands::Profile {
                    root,
                    positional_root,
                    preset,
                    settings,
                    json,
                } => execute_profile(
                    &root
                        .or(positional_root)
                        .unwrap_or_else(|| PathBuf::from(".")),
                    preset.as_deref(),
                    &settings,
                    json,
                ),
                MethodEvolutionCommands::Suggestions {
                    root,
                    positional_root,
                    limit,
                    json,
                } => execute_suggestions(
                    &root
                        .or(positional_root)
                        .unwrap_or_else(|| PathBuf::from(".")),
                    limit,
                    json,
                ),
                MethodEvolutionCommands::Plan {
                    recommendation_id,
                    root,
                    positional_root,
                    json,
                } => execute_plan(
                    &root
                        .or(positional_root)
                        .unwrap_or_else(|| PathBuf::from(".")),
                    &recommendation_id,
                    json,
                ),
                MethodEvolutionCommands::Apply {
                    recommendation_id,
                    root,
                    positional_root,
                    approve,
                } => execute_apply(
                    &root
                        .or(positional_root)
                        .unwrap_or_else(|| PathBuf::from(".")),
                    &recommendation_id,
                    approve,
                ),
            };
            if let Err(e) = result {
                eprintln!("method-evolution error: {}", e);
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
        Commands::Governance { subcommand } => match subcommand {
            GovernanceCommands::Plan {
                strategy,
                target,
                root,
                json,
            } => {
                let ws_root = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                if let Err(e) = execute_governance_plan(&ws_root, &strategy, &target, json) {
                    eprintln!("governance plan error: {}", e);
                    std::process::exit(1);
                }
            }
            GovernanceCommands::Apply {
                strategy,
                target,
                root,
                approve,
            } => {
                let ws_root = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                if let Err(e) = execute_governance_apply(&ws_root, &strategy, &target, approve) {
                    eprintln!("governance apply error: {}", e);
                    std::process::exit(1);
                }
            }
            GovernanceCommands::SyncHarnesses {
                target,
                root,
                approve,
                json,
            } => {
                let ws_root = root.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                if let Err(e) = execute_sync_harnesses(&ws_root, target, approve, json) {
                    eprintln!("governance sync error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::TraceImpact {
            paths,
            range,
            staged,
            root,
            path,
            positional_root,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            if let Err(e) = llm_wiki_core::cli::trace_impact::execute_trace_impact(
                &workspace_root,
                paths,
                range,
                staged,
                json,
            ) {
                eprintln!("trace-impact error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Review { subcommand } => match subcommand {
            ReviewCommands::Resolve {
                doc_id,
                commit_ref,
                root,
                path,
                positional_root,
            } => {
                let workspace_root = path
                    .or(root)
                    .or(positional_root)
                    .unwrap_or_else(|| PathBuf::from("."));
                if let Err(e) = llm_wiki_core::cli::review_cmd::execute_review_resolve(
                    &workspace_root,
                    &doc_id,
                    commit_ref.as_deref(),
                ) {
                    eprintln!("review resolve error: {}", e);
                    std::process::exit(1);
                }
            }
        },
        Commands::FixLinks {
            root,
            path,
            positional_root,
            file,
            apply,
            json,
        } => {
            let workspace_root = path
                .or(root)
                .or(positional_root)
                .unwrap_or_else(|| PathBuf::from("."));
            match llm_wiki_core::automation::link_normalizer::normalize_workspace_links(
                &workspace_root,
                file.as_deref(),
                apply,
            ) {
                Ok(report) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&report)?);
                    } else {
                        let mode = if report.applied { "APPLIED" } else { "DRY-RUN" };
                        println!(
                            "[{}] Scanned {} file(s) | {} link(s) {} across {} file(s) | {} already clickable | {} broken skipped",
                            mode,
                            report.files_scanned,
                            report.total_rewrites,
                            if report.applied { "rewritten" } else { "to rewrite" },
                            report.documents.len(),
                            report.total_already_clickable,
                            report.total_broken_skipped
                        );
                        for doc in &report.documents {
                            println!("\n--- {} ({} rewrites) ---", doc.file_path, doc.rewrites.len());
                            if !doc.unified_diff.is_empty() {
                                println!("{}", doc.unified_diff);
                            }
                        }
                        if !report.applied && report.total_rewrites > 0 {
                            println!("Run with --apply to write changes to disk.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("fix-links error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
