//! Strict classification of properties and findings across 4 Epistemic Tiers.
//! Implements FR-003 and SC-001.

use crate::model::EpistemicTier;

/// Strict epistemic classifier enforcing theoretical and algorithmic boundaries.
pub struct EpistemicClassifier;

impl EpistemicClassifier {
    /// Classifies an analytical rule key or metric name into its corresponding EpistemicTier.
    pub fn classify_rule(rule_key: &str) -> EpistemicTier {
        match rule_key {
            // Tier A: Direct calculation, graph topology, cycle detection, token counts
            "graph_topology"
            | "token_count"
            | "heading_fragment"
            | "broken_reference"
            | "broken_fragment_anchor"
            | "tarjan_scc_cycle"
            | "reciprocal_link"
            | "unreachable_document"
            | "provenance_commit" => EpistemicTier::TierAFullyDeterministic,

            // Tier B: Algorithmic candidate generation with fixed seeds and thresholds
            "louvain_community"
            | "split_candidate"
            | "merge_candidate"
            | "bridge_candidate"
            | "vocabulary_typo_candidate"
            | "hub_candidate" => EpistemicTier::TierBCandidateGeneration,

            // Tier C: Multi-criteria decision support, AHP fit scores, utility valuation
            "ahp_fit_score"
            | "strategy_recommendation"
            | "composite_plan_value"
            | "trade_off_matrix"
            | "priority_ranking" => EpistemicTier::TierCDecisionSupport,

            // Tier D: Epistemological limits - cannot be determined purely deterministically
            "semantic_truth"
            | "author_intent"
            | "prose_elegance"
            | "factual_veracity" => EpistemicTier::TierDNotReliablyDeterminable,

            // Default fallback based on naming convention
            s if s.ends_with("_candidate") => EpistemicTier::TierBCandidateGeneration,
            s if s.ends_with("_score") || s.ends_with("_recommendation") => {
                EpistemicTier::TierCDecisionSupport
            }
            s if s.ends_with("_limit") || s.contains("intent") || s.contains("truth") => {
                EpistemicTier::TierDNotReliablyDeterminable
            }
            _ => EpistemicTier::TierAFullyDeterministic,
        }
    }

    /// Asserts whether a given operation requires external AI or can be performed deterministically.
    pub fn is_deterministic(tier: EpistemicTier) -> bool {
        matches!(
            tier,
            EpistemicTier::TierAFullyDeterministic
                | EpistemicTier::TierBCandidateGeneration
                | EpistemicTier::TierCDecisionSupport
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epistemic_classification() {
        assert_eq!(
            EpistemicClassifier::classify_rule("broken_reference"),
            EpistemicTier::TierAFullyDeterministic
        );
        assert_eq!(
            EpistemicClassifier::classify_rule("tarjan_scc_cycle"),
            EpistemicTier::TierAFullyDeterministic
        );
        assert_eq!(
            EpistemicClassifier::classify_rule("split_candidate"),
            EpistemicTier::TierBCandidateGeneration
        );
        assert_eq!(
            EpistemicClassifier::classify_rule("ahp_fit_score"),
            EpistemicTier::TierCDecisionSupport
        );
        assert_eq!(
            EpistemicClassifier::classify_rule("author_intent"),
            EpistemicTier::TierDNotReliablyDeterminable
        );
    }
}
