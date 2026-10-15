//! Transformation Plan Generator with cryptographic approval tokens.
//! Implements FR-036, FR-037, and Constitution Invariant XVI.

use crate::model::{StrategyType, TransformationAction, TransformationPlan};
use blake3::Hasher;

/// Generates a cryptographic approval token for a transformation plan.
pub fn generate_approval_token(plan_id: &str, actions: &[TransformationAction]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(b"approval_token_v1:");
    hasher.update(plan_id.as_bytes());
    hasher.update(b":");
    if let Ok(serialized) = serde_json::to_string(actions) {
        hasher.update(serialized.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

/// Verifies whether an approval token is valid for a given plan.
pub fn verify_approval_token(
    plan_id: &str,
    actions: &[TransformationAction],
    token: &str,
) -> bool {
    let expected = generate_approval_token(plan_id, actions);
    expected == token
}

#[derive(Debug, Clone, Default)]
pub struct TransformationPlanGenerator;

impl TransformationPlanGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn create_plan(
        &self,
        strategy_id: &str,
        _strategy_type: StrategyType,
        target_files: Vec<String>,
        actions: Vec<TransformationAction>,
        projected_score_delta: f64,
    ) -> TransformationPlan {
        let plan_id = format!("plan-{}", blake3::hash(strategy_id.as_bytes()).to_hex()[..12].to_string());
        let approval_token = generate_approval_token(&plan_id, &actions);

        let mut diffs = Vec::new();
        for action in &actions {
            match action {
                TransformationAction::MoveFile { source, destination } => {
                    diffs.push(format!("rename from {}\nrename to {}", source, destination));
                }
                TransformationAction::UpdateFrontmatter { file, key, value } => {
                    diffs.push(format!("--- a/{}\n+++ b/{}\n@@ -1,3 +1,4 @@\n+ {}: {}", file, file, key, value));
                }
                TransformationAction::RetargetLink { file, old_target, new_target, .. } => {
                    diffs.push(format!("--- a/{}\n+++ b/{}\n-[[{}]]\n+[[{}]]", file, file, old_target, new_target));
                }
                TransformationAction::ExtractSectionToFile { source_file, anchor, new_file } => {
                    diffs.push(format!("extract section #{} from {} into new file {}", anchor, source_file, new_file));
                }
            }
        }

        TransformationPlan {
            plan_id,
            approval_token,
            strategy_id: strategy_id.to_string(),
            target_files,
            unified_diff: diffs.join("\n\n"),
            projected_score_delta,
            actions,
        }
    }

    pub fn validate_approval(
        &self,
        plan: &TransformationPlan,
        provided_token: &str,
    ) -> Result<(), String> {
        if verify_approval_token(&plan.plan_id, &plan.actions, provided_token) {
            Ok(())
        } else {
            Err("Invalid or unauthorized approval token. Transformation rejected per Invariant XVI.".to_string())
        }
    }
}
