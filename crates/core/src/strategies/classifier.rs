//! Pairwise strategy relationship role classifier.
//! Implements FR-034: PRIMARY, SUPPORTING, ALTERNATIVE, and MUTUALLY_EXCLUSIVE.

use crate::model::{StrategyRole, StrategyType};

/// Classifies the pairwise role relationship between two strategy options.
pub fn classify_pairwise_role(
    strat_a: StrategyType,
    strat_b: StrategyType,
    target_a: &str,
    target_b: &str,
) -> StrategyRole {
    // If scopes differ, they can be complementary/supporting across the repo
    if target_a != target_b {
        return StrategyRole::Supporting;
    }

    // Same document scope:
    match (strat_a, strat_b) {
        // Mutually exclusive: you cannot preserve a document while splitting or merging it
        (StrategyType::PreserveDocument, StrategyType::SplitDecompose)
        | (StrategyType::SplitDecompose, StrategyType::PreserveDocument)
        | (StrategyType::PreserveDocument, StrategyType::MergeCandidates)
        | (StrategyType::MergeCandidates, StrategyType::PreserveDocument) => {
            StrategyRole::MutuallyExclusive
        }

        // Supporting / complementary: can be applied together on the same document
        (StrategyType::SplitDecompose, StrategyType::ExtractFacetMetadata)
        | (StrategyType::ExtractFacetMetadata, StrategyType::SplitDecompose)
        | (StrategyType::SplitDecompose, StrategyType::StandardizeTerminology)
        | (StrategyType::StandardizeTerminology, StrategyType::SplitDecompose)
        | (StrategyType::ExtractFacetMetadata, StrategyType::StandardizeTerminology)
        | (StrategyType::StandardizeTerminology, StrategyType::ExtractFacetMetadata) => {
            StrategyRole::Supporting
        }

        // Alternatives: competing transformation pathways for the same target
        _ => StrategyRole::Alternative,
    }
}
