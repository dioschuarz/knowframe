//! CLI entrypoint for knowframe-cli.

use clap::{Parser, Subcommand};
use llm_wiki_cli::commands::{
    analyze::execute_analyze,
    baseline::execute_baseline,
    plan::{execute_plan_apply, execute_plan_generate},
    sync::execute_sync,
    validate::execute_validate,
};
use llm_wiki_core::model::EvolutionMode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "knowframe-cli")]
#[command(about = "CLI driver for KnowFrame deterministic knowledge engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze repository documents and emit deterministic JSON topology
    Analyze {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        #[arg(short, long)]
        mode: Option<String>,
        #[arg(short, long)]
        commit: Option<String>,
        #[arg(short, long)]
        pretty: bool,
    },
    /// Baseline manager command
    Baseline {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
    /// Pre-commit validation command with <=500ms fail-open guarantee
    Validate {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
    /// Post-commit sync command with <=50ms latency budget
    Sync {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
    /// Transformation plan commands
    Plan {
        #[command(subcommand)]
        action: PlanCommands,
    },
}

#[derive(Subcommand)]
enum PlanCommands {
    Generate {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        #[arg(short, long)]
        strategy_id: String,
        #[arg(short, long)]
        target: String,
    },
    Apply {
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
        #[arg(long)]
        plan_id: String,
        #[arg(long)]
        approval_token: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze {
            root,
            mode,
            commit,
            pretty,
        } => {
            let evolution_mode = mode.as_deref().and_then(|m| match m {
                "ai_oriented" => Some(EvolutionMode::AiOriented),
                "human_oriented" => Some(EvolutionMode::HumanOriented),
                "balanced" => Some(EvolutionMode::Balanced),
                "custom" => Some(EvolutionMode::Custom),
                _ => None,
            });
            execute_analyze(&root, evolution_mode, commit.as_deref(), pretty)?;
        }
        Commands::Baseline { root } => {
            execute_baseline(&root)?;
        }
        Commands::Validate { root } => {
            execute_validate(&root)?;
        }
        Commands::Sync { root } => {
            execute_sync(&root)?;
        }
        Commands::Plan { action } => match action {
            PlanCommands::Generate {
                root,
                strategy_id,
                target,
            } => {
                execute_plan_generate(&root, &strategy_id, &target)?;
            }
            PlanCommands::Apply {
                root,
                plan_id,
                approval_token,
            } => {
                execute_plan_apply(&root, &plan_id, &approval_token, &[])?;
            }
        },
    }

    Ok(())
}
