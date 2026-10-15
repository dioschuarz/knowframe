//! CLI workflow for AI documentation governance (plan, diff preview, apply, and sync).

use crate::evolution::transformation::{GovernancePlanGenerator, GovernancePlanStore};
use crate::harness::GovernanceStrategyType;
use std::path::Path;

/// Generates and previews a governance transformation plan with standard unified diff.
pub fn execute_governance_plan(
    root: &Path,
    strategy_str: &str,
    target: &str,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let strategy = GovernanceStrategyType::from_str(strategy_str).ok_or_else(|| {
        format!(
            "Unknown governance strategy '{}'. Available strategies: sync_harness_routes, eliminate_secondary_routing_layer, introduce_section_addressing, restructure_directory_domain",
            strategy_str
        )
    })?;

    let plan = GovernancePlanGenerator::generate_plan(root, strategy, target)?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&plan)?);
        return Ok(());
    }

    println!("==============================================================");
    println!("Governance Plan: {}", plan.title);
    println!("==============================================================");
    println!("Strategy:  {}", plan.strategy_type.as_str());
    println!("Target:    {}", plan.target_file_or_dir);
    println!("Plan ID:   {}", plan.plan_id);
    println!("Approval:  {}", plan.approval_token);
    println!();
    println!("Description:\n  {}", plan.description);
    println!();

    if !plan.expected_benefits.is_empty() {
        println!("Expected Benefits:");
        for b in &plan.expected_benefits {
            println!("  + {}", b);
        }
        println!();
    }

    if !plan.countervailing_trade_offs.is_empty() {
        println!("Countervailing Trade-Offs:");
        for t in &plan.countervailing_trade_offs {
            println!("  - {}", t);
        }
        println!();
    }

    println!("Projected Impact:");
    println!("  Token Delta: {:+} tokens", plan.projected_delta_tokens);
    println!("  Hop Delta:   {:+}", plan.projected_delta_hops);
    println!("  Actions:     {}", plan.actions.len());
    println!();

    println!("Unified Diff Preview:");
    println!("--------------------------------------------------------------");
    println!("{}", plan.unified_diff.trim_end());
    println!("--------------------------------------------------------------");

    if !plan.actions.is_empty() {
        println!();
        println!("To apply this plan after reviewing the diff, run:");
        println!(
            "  llm-wiki-core governance apply --strategy {} --target {} --approve",
            plan.strategy_type.as_str(),
            plan.target_file_or_dir
        );
    }

    Ok(())
}

/// Applies a governance transformation plan after explicit user approval (Invariant XVI).
pub fn execute_governance_apply(
    root: &Path,
    strategy_str: &str,
    target: &str,
    approve: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !approve {
        return Err(
            "Applying a governance plan requires explicit user approval: pass '--approve' (Constitutional Invariant XVI)".into()
        );
    }

    let strategy = GovernanceStrategyType::from_str(strategy_str).ok_or_else(|| {
        format!(
            "Unknown governance strategy '{}'. Available strategies: sync_harness_routes, eliminate_secondary_routing_layer, introduce_section_addressing, restructure_directory_domain",
            strategy_str
        )
    })?;

    let plan = GovernancePlanGenerator::generate_plan(root, strategy, target)?;

    if plan.actions.is_empty() {
        println!("No changes needed. Target '{}' is already in 100% parity/compliance.", target);
        return Ok(());
    }

    let store = GovernancePlanStore::default();
    store.save_plan(plan.clone());

    let result = store.apply_plan(root, &plan.plan_id, &plan.approval_token)?;

    if result.applied {
        println!("Successfully applied governance plan '{}'!", plan.plan_id);
        println!("Files modified:");
        for f in &result.files_modified {
            println!("  - {}", f);
        }
        println!("Re-Measurement Impact:");
        println!("  Tokens Delta: {:+} tokens", result.re_measurement.delta_tokens);
        println!("  Hops Delta:   {:+}", result.re_measurement.delta_hops);
        println!("  Gov Score:    {:+.1} pts", result.re_measurement.delta_governance_score);
    } else {
        println!("Plan application completed with 0 files modified.");
    }

    Ok(())
}

/// Convenient helper command to sync harness routes with preview or approval.
pub fn execute_sync_harnesses(
    root: &Path,
    target: Option<String>,
    approve: bool,
    json_output: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let target_file = target.unwrap_or_else(|| "AGENTS.md".to_string());
    if approve {
        execute_governance_apply(root, "sync_harness_routes", &target_file, true)
    } else {
        execute_governance_plan(root, "sync_harness_routes", &target_file, json_output)
    }
}
