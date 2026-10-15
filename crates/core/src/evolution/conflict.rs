use super::compiler::{ObjectiveLevel, Tier1Objectives, Tier2Parameters};
use super::profile::{ConstraintConflict, ConstraintLevel, ConstraintRule};

pub const RULE_NEVER_SPLIT_CANONICAL_DOCS: &str = "never_split_canonical_docs";
pub const RULE_NEVER_AUTO_SPLIT_DOCUMENTS: &str = "never_auto_split_documents";
pub const RULE_NEVER_DELETE_CONTENT: &str = "never_delete_content";
pub const RULE_NEVER_SPLIT_DOCUMENTS: &str = "never_split_documents";
pub const RULE_FORCE_DECOMPOSITION: &str = "force_decomposition";
pub const RULE_ENFORCE_STRICT_SURFACE: &str = "enforce_strict_surface";

#[derive(Debug, Clone, Default)]
pub struct ConflictDetector;

impl ConflictDetector {
    pub fn new() -> Self {
        Self
    }

    /// Built-in constraint templates per FR-021.
    pub fn builtin_constraints() -> Vec<ConstraintRule> {
        vec![
            ConstraintRule {
                id: "builtin_never_split_canonical".to_string(),
                level: ConstraintLevel::HardConstraint,
                rule_key: RULE_NEVER_SPLIT_CANONICAL_DOCS.to_string(),
                description: "Strictly prevents splitting canonical architecture or ADR notes".to_string(),
            },
            ConstraintRule {
                id: "builtin_never_auto_split".to_string(),
                level: ConstraintLevel::HardConstraint,
                rule_key: RULE_NEVER_AUTO_SPLIT_DOCUMENTS.to_string(),
                description: "Prohibits unattended background/automatic document splitting while permitting explicit user-approved transformation plans".to_string(),
            },
            ConstraintRule {
                id: "builtin_never_delete_content".to_string(),
                level: ConstraintLevel::HardConstraint,
                rule_key: RULE_NEVER_DELETE_CONTENT.to_string(),
                description: "Strictly prevents automated deletions".to_string(),
            },
        ]
    }

    /// Evaluates constraint rules for mutual satisfiability per FR-022.
    pub fn validate(&self, constraints: &[ConstraintRule]) -> (bool, Vec<ConstraintConflict>) {
        self.validate_with_tier2(None, None, constraints)
    }

    /// Evaluates constraint rules combined with optional Tier 1/Tier 2 parameters.
    pub fn validate_with_tier2(
        &self,
        tier1: Option<&Tier1Objectives>,
        tier2: Option<&Tier2Parameters>,
        constraints: &[ConstraintRule],
    ) -> (bool, Vec<ConstraintConflict>) {
        let mut conflicts = Vec::new();

        let has_rule = |key: &str| constraints.iter().any(|c| c.rule_key == key);
        let has_no_split =
            has_rule(RULE_NEVER_SPLIT_CANONICAL_DOCS) || has_rule(RULE_NEVER_SPLIT_DOCUMENTS);

        // Rule 1: Contradictory splitting directives
        if has_no_split && has_rule(RULE_FORCE_DECOMPOSITION) {
            conflicts.push(ConstraintConflict {
                conflicting_rules: vec![
                    RULE_NEVER_SPLIT_DOCUMENTS.to_string(),
                    RULE_FORCE_DECOMPOSITION.to_string(),
                ],
                reason: "Contradictory directives: cannot simultaneously enforce 'never_split' and 'force_decomposition'.".to_string(),
                suggestion: "Remove 'force_decomposition' or relax the 'never_split' constraint.".to_string(),
            });
        }

        // Rule 2: Strict context surface budget vs never split documents (FR-022)
        if has_no_split {
            if let Some(t2) = tier2 {
                if t2.max_context_surface_tokens <= 5000 {
                    conflicts.push(ConstraintConflict {
                        conflicting_rules: vec![
                            "max_context_surface_tokens <= 5000".to_string(),
                            RULE_NEVER_SPLIT_DOCUMENTS.to_string(),
                        ],
                        reason: "Context budget (<= 5,000 tokens) cannot be satisfied without splitting oversized documents.".to_string(),
                        suggestion: "Permit section addressing or topic decomposition, or increase max context surface budget.".to_string(),
                    });
                }
            } else if has_rule(RULE_ENFORCE_STRICT_SURFACE) {
                conflicts.push(ConstraintConflict {
                    conflicting_rules: vec![
                        RULE_ENFORCE_STRICT_SURFACE.to_string(),
                        RULE_NEVER_SPLIT_DOCUMENTS.to_string(),
                    ],
                    reason: "Context budget cannot be satisfied without splitting oversized documents.".to_string(),
                    suggestion: "Permit section addressing or topic decomposition, or relax context surface requirements.".to_string(),
                });
            }
        }

        // Rule 3: High context efficiency vs High narrative coherence vs never split
        if let Some(t1) = tier1 {
            if t1.context_efficiency == ObjectiveLevel::High
                && t1.narrative_coherence == ObjectiveLevel::High
                && has_no_split
            {
                conflicts.push(ConstraintConflict {
                    conflicting_rules: vec![
                        "context_efficiency: high".to_string(),
                        "narrative_coherence: high".to_string(),
                        RULE_NEVER_SPLIT_DOCUMENTS.to_string(),
                    ],
                    reason: "High tension: demanding maximal context efficiency while strictly preserving monolithic narrative coherence without splitting creates irreconcilable constraints.".to_string(),
                    suggestion: "Downgrade one objective or permit section-level fragment addressing.".to_string(),
                });
            }
        }

        let is_valid = conflicts.is_empty();
        (is_valid, conflicts)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_builtin_hard_constraints_registration() {
        let builtins = ConflictDetector::builtin_constraints();
        assert_eq!(builtins.len(), 3);
        let keys: Vec<String> = builtins.into_iter().map(|c| c.rule_key).collect();
        assert!(keys.contains(&RULE_NEVER_SPLIT_CANONICAL_DOCS.to_string()));
        assert!(keys.contains(&RULE_NEVER_AUTO_SPLIT_DOCUMENTS.to_string()));
        assert!(keys.contains(&RULE_NEVER_DELETE_CONTENT.to_string()));
    }

    #[test]
    fn test_constraint_conflict_detector_latency() {
        let detector = ConflictDetector::new();
        let builtins = ConflictDetector::builtin_constraints();

        // Valid constraint set: builtins only
        let (is_valid, conflicts) = detector.validate(&builtins);
        assert!(is_valid);
        assert!(conflicts.is_empty());

        // Conflicting constraint set
        let mut contradictory = builtins.clone();
        contradictory.push(ConstraintRule {
            id: "contradict_decomp".to_string(),
            level: ConstraintLevel::HardConstraint,
            rule_key: RULE_FORCE_DECOMPOSITION.to_string(),
            description: "Force decomposition of all documents".to_string(),
        });

        let start = std::time::Instant::now();
        let (is_valid_c, conflicts_c) = detector.validate(&contradictory);
        let elapsed = start.elapsed();

        assert!(!is_valid_c);
        assert!(!conflicts_c.is_empty());
        assert!(
            elapsed <= std::time::Duration::from_millis(10),
            "Conflict detection must execute within 10ms (NFR-002, SC-002), took {:?}",
            elapsed
        );
    }
}
