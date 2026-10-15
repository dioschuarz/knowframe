//! Architectural rule evaluation, epistemic classification, and semantic drift separation.
//! Enforces FR-012, FR-013, FR-014, and SC-003.

use crate::config::{ArchitecturalRule, ArchitecturalRuleType};
use crate::drift::index::ReverseImpactIndex;
use crate::model::{
    ChangeEvaluationCategory, DeterministicFinding, EpistemicTier, FindingEvidence,
};
use std::collections::HashMap;
use std::path::Path;

/// Evaluates explicit architectural rules from `.context-vault.yaml` against repository code and documents.
pub fn evaluate_architectural_rules(
    workspace_root: &Path,
    rules: &[ArchitecturalRule],
    changed_files: &[String],
    reverse_index: &ReverseImpactIndex,
    code_dependencies_map: &HashMap<String, Vec<String>>,
) -> Vec<DeterministicFinding> {
    let mut findings = Vec::new();

    for rule in rules {
        match rule.rule_type {
            ArchitecturalRuleType::ProhibitedDependency => {
                let source_pat = rule.source_pattern.as_deref().unwrap_or("");
                let target_pat = rule.target_pattern.as_deref().unwrap_or("");

                for (doc_id, deps) in code_dependencies_map {
                    if matches_pattern(doc_id, source_pat) {
                        for dep in deps {
                            if matches_pattern(dep, target_pat) {
                                findings.push(DeterministicFinding {
                                    finding_id: format!("FINDING-RULE-{}", rule.id),
                                    rule_id: rule.id.clone(),
                                    severity: rule.severity,
                                    subject: doc_id.clone(),
                                    evidence: FindingEvidence {
                                        changed_paths: vec![dep.clone()],
                                        dependency_sources: vec!["architectural_rule".to_string()],
                                        rule_description: rule.explanation.clone(),
                                        line_number: None,
                                        anchor: None,
                                        target: Some(dep.clone()),
                                    },
                                    required_action: format!(
                                        "Remove prohibited dependency from '{}' to '{}' per rule '{}'",
                                        doc_id, dep, rule.id
                                    ),
                                    epistemic_tier: Some(EpistemicTier::TierAFullyDeterministic),
                                });
                            }
                        }
                    }
                }
            }
            ArchitecturalRuleType::MandatoryDocumentation => {
                let source_pat = rule.source_pattern.as_deref().unwrap_or("");
                for changed_file in changed_files {
                    if matches_pattern(changed_file, source_pat) {
                        let citing_docs = reverse_index.get_docs_for_code(changed_file);
                        let is_documented = if let Some(ref req_doc) = rule.required_doc {
                            citing_docs.contains(req_doc)
                        } else {
                            !citing_docs.is_empty()
                        };

                        if !is_documented {
                            findings.push(DeterministicFinding {
                                finding_id: format!("FINDING-MANDATORY-DOC-{}", rule.id),
                                rule_id: rule.id.clone(),
                                severity: rule.severity,
                                subject: changed_file.clone(),
                                evidence: FindingEvidence {
                                    changed_paths: vec![changed_file.clone()],
                                    dependency_sources: vec!["architectural_rule".to_string()],
                                    rule_description: rule.explanation.clone(),
                                    line_number: None,
                                    anchor: None,
                                    target: rule.required_doc.clone(),
                                },
                                required_action: format!(
                                    "Provide required architectural documentation for '{}' in '{}'",
                                    changed_file,
                                    rule.required_doc.as_deref().unwrap_or("an architectural note")
                                ),
                                epistemic_tier: Some(EpistemicTier::TierAFullyDeterministic),
                            });
                        }
                    }
                }
            }
            ArchitecturalRuleType::PathExists => {
                let target_path_str = rule
                    .target_pattern
                    .as_deref()
                    .or(rule.source_pattern.as_deref())
                    .unwrap_or("");
                if !target_path_str.is_empty() {
                    let target_abs = workspace_root.join(target_path_str);
                    if !target_abs.exists() {
                        findings.push(DeterministicFinding {
                            finding_id: format!("FINDING-PATH-EXISTS-{}", rule.id),
                            rule_id: rule.id.clone(),
                            severity: rule.severity,
                            subject: target_path_str.to_string(),
                            evidence: FindingEvidence {
                                changed_paths: vec![target_path_str.to_string()],
                                dependency_sources: vec!["path_exists".to_string()],
                                rule_description: rule.explanation.clone(),
                                line_number: None,
                                anchor: None,
                                target: Some(target_path_str.to_string()),
                            },
                            required_action: format!(
                                "Create or restore required path '{}' per rule '{}'",
                                target_path_str, rule.id
                            ),
                            epistemic_tier: Some(EpistemicTier::TierAFullyDeterministic),
                        });
                    }
                }
            }
        }
    }

    findings
}

/// Evaluates a code or doc change and classifies it into one of the four categories:
/// 1. `rule_violation`: Explicit constraint broken.
/// 2. `review_required`: Code changed and documentation explicitly declares dependency.
/// 3. `change_detected`: Code changed without declaring documentation.
/// 4. `not_verifiable`: Epistemic Tier D freeform semantic prose unverified by deterministic facts.
pub fn classify_change(
    has_rule_violation: bool,
    has_citing_docs: bool,
    is_code_modified: bool,
    is_freeform_claim: bool,
) -> ChangeEvaluationCategory {
    if has_rule_violation {
        ChangeEvaluationCategory::RuleViolation
    } else if is_freeform_claim {
        ChangeEvaluationCategory::NotVerifiable
    } else if has_citing_docs && is_code_modified {
        ChangeEvaluationCategory::ReviewRequired
    } else if is_code_modified {
        ChangeEvaluationCategory::ChangeDetected
    } else {
        ChangeEvaluationCategory::NotVerifiable
    }
}

/// Evaluates a semantic prose claim against deterministic evidence.
/// When no explicit AST code citation or architectural rule covers the claim,
/// enforces Epistemic Tier D (`not_verifiable`).
pub fn evaluate_semantic_claim(
    _claim_text: &str,
    has_explicit_evidence: bool,
) -> (ChangeEvaluationCategory, Option<EpistemicTier>) {
    if !has_explicit_evidence {
        (
            ChangeEvaluationCategory::NotVerifiable,
            Some(EpistemicTier::TierDNotReliablyDeterminable),
        )
    } else {
        (
            ChangeEvaluationCategory::ChangeDetected,
            Some(EpistemicTier::TierAFullyDeterministic),
        )
    }
}

/// Asserts that code modifications must never auto-mutate architectural documentation notes
/// without explicit user consent (FR-014, Invariant XVI).
pub fn can_auto_update_architecture(user_approved: bool) -> bool {
    user_approved
}

/// Returns true if a path matches a wildcard pattern or prefix.
pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    if pattern.is_empty() || pattern == "*" {
        return true;
    }
    let norm_path = path.replace('\\', "/");
    let norm_pattern = pattern.replace('\\', "/");

    if norm_pattern.ends_with('*') {
        let prefix = norm_pattern.trim_end_matches('*');
        norm_path.starts_with(prefix)
    } else if norm_pattern.starts_with('*') {
        let suffix = norm_pattern.trim_start_matches('*');
        norm_path.ends_with(suffix)
    } else {
        norm_path == norm_pattern || norm_path.starts_with(&format!("{}/", norm_pattern))
    }
}
