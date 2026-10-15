//! Unit test verifying that all recommendations itemize expected benefits and countervailing trade-offs.
//! Validates FR-031, FR-035, and Constitution Invariant XV.

use llm_wiki_core::model::StrategyType;
use llm_wiki_core::strategies::tradeoffs::generate_strategy_tradeoffs;

#[test]
fn test_all_strategies_itemize_benefits_and_tradeoffs() {
    let strategies = vec![
        StrategyType::PreserveDocument,
        StrategyType::SplitDecompose,
        StrategyType::ExtractFacetMetadata,
        StrategyType::ConsolidateIndex,
        StrategyType::MergeCandidates,
        StrategyType::StandardizeTerminology,
    ];

    for strat in strategies {
        let (benefits, tradeoffs) = generate_strategy_tradeoffs(strat, "test.md", 5000);
        assert!(
            !benefits.is_empty(),
            "Strategy {:?} must explicitly output expected benefits",
            strat
        );
        assert!(
            !tradeoffs.is_empty(),
            "Strategy {:?} must explicitly output countervailing trade-offs per Invariant XV",
            strat
        );
    }
}
