//! CLI analyze command emitting bit-exact JSON with analysis_id and graph topology.
//! Implements FR-002, FR-046, and SC-002.

use anyhow::Result;
use llm_wiki_core::model::{EvolutionMode, OptimizationProfile};
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use serde_json::json;
use std::path::Path;

/// Executes the analyze command on a repository.
pub fn execute_analyze(
    root: &Path,
    mode: Option<EvolutionMode>,
    commit: Option<&str>,
    pretty: bool,
) -> Result<()> {
    let profile = OptimizationProfile::for_mode(mode.unwrap_or(EvolutionMode::AiOriented));
    let commit_hash = commit.unwrap_or("HEAD");

    let result = scan_repository(root, &ScanOptions::default(), &profile, commit_hash)?;

    let output = json!({
        "analysis_id": result.analysis_id,
        "epistemic_tier": "tier_a_fully_deterministic",
        "total_documents": result.total_documents,
        "total_tokens": result.total_tokens,
        "topology": result.topology,
    });

    if pretty {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("{}", serde_json::to_string(&output)?);
    }

    Ok(())
}
