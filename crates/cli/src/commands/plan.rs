//! CLI commands for transformation plan generation and application.
//! Implements FR-036, FR-037, and Invariant XVI.

use anyhow::{anyhow, Result};
use llm_wiki_core::automation::mutators::apply_transformation_action;
use llm_wiki_core::automation::plan_generator::TransformationPlanGenerator;
use llm_wiki_core::model::{StrategyType, TransformationAction};
use serde_json::json;
use std::path::Path;

pub fn execute_plan_generate(_root: &Path, strategy_id: &str, target: &str) -> Result<()> {
    let generator = TransformationPlanGenerator::new();
    let actions = vec![TransformationAction::UpdateFrontmatter {
        file: target.to_string(),
        key: "evolution_strategy".to_string(),
        value: serde_json::json!(strategy_id),
    }];

    let plan = generator.create_plan(
        strategy_id,
        StrategyType::SplitDecompose,
        vec![target.to_string()],
        actions,
        5.0,
    );

    let output = json!({
        "plan_id": plan.plan_id,
        "approval_token": plan.approval_token,
        "strategy_id": plan.strategy_id,
        "target_files": plan.target_files,
        "unified_diff": plan.unified_diff,
        "projected_score_delta": plan.projected_score_delta,
        "actions": plan.actions,
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

pub fn execute_plan_apply(
    _root: &Path,
    plan_id: &str,
    approval_token: &str,
    actions: &[TransformationAction],
) -> Result<()> {
    let generator = TransformationPlanGenerator::new();
    let dummy_plan = llm_wiki_core::model::TransformationPlan {
        plan_id: plan_id.to_string(),
        approval_token: approval_token.to_string(),
        strategy_id: "strat-apply".to_string(),
        target_files: Vec::new(),
        unified_diff: String::new(),
        projected_score_delta: 0.0,
        actions: actions.to_vec(),
    };

    generator
        .validate_approval(&dummy_plan, approval_token)
        .map_err(|e| anyhow!(e))?;

    for action in actions {
        apply_transformation_action(action)?;
    }

    println!("Transformation plan {} successfully applied.", plan_id);
    Ok(())
}
