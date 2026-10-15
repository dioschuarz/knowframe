//! Unit test for pairwise strategy relationship roles.
//! Validates FR-034: PRIMARY, SUPPORTING, ALTERNATIVE, MUTUALLY_EXCLUSIVE.

use llm_wiki_core::model::{StrategyRole, StrategyType};
use llm_wiki_core::strategies::classifier::classify_pairwise_role;

#[test]
fn test_preserve_document_and_split_decompose_mutually_exclusive() {
    let role = classify_pairwise_role(
        StrategyType::PreserveDocument,
        StrategyType::SplitDecompose,
        "doc1.md",
        "doc1.md",
    );
    assert_eq!(role, StrategyRole::MutuallyExclusive);
}

#[test]
fn test_split_decompose_and_facet_metadata_supporting() {
    let role = classify_pairwise_role(
        StrategyType::SplitDecompose,
        StrategyType::ExtractFacetMetadata,
        "doc1.md",
        "doc1.md",
    );
    assert_eq!(role, StrategyRole::Supporting);
}

#[test]
fn test_alternative_strategies_same_scope() {
    let role = classify_pairwise_role(
        StrategyType::SplitDecompose,
        StrategyType::ConsolidateIndex,
        "doc1.md",
        "doc1.md",
    );
    assert_eq!(role, StrategyRole::Alternative);
}
