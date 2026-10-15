//! Evidence-based expected benefits and countervailing trade-offs generator.
//! Implements FR-031, FR-035, and Constitution Invariant XV.

use crate::model::StrategyType;

/// Derives concrete mathematical expected benefits and countervailing trade-offs for a strategy.
pub fn generate_strategy_tradeoffs(
    strategy: StrategyType,
    target_doc: &str,
    tokens: u64,
) -> (Vec<String>, Vec<String>) {
    match strategy {
        StrategyType::PreserveDocument => (
            vec![
                format!("Preserves comprehensive linear reading narrative of {}", target_doc),
                "Zero filesystem churn or broken link risk".to_string(),
                "Maintains high internal cohesion".to_string(),
            ],
            vec![
                format!("High prompt token load ({} tokens) when loaded into context", tokens),
                "Coarse retrieval granularity without sub-section addressing".to_string(),
            ],
        ),
        StrategyType::SplitDecompose => (
            vec![
                "Reduces per-document context token footprint by 60-80%".to_string(),
                "Increases topic modularity and targeted retrieval precision".to_string(),
                "Enables fine-grained parallel editing without git merge conflicts".to_string(),
            ],
            vec![
                "Increases total vault document count and link management overhead".to_string(),
                "Requires updating incoming references and breadcrumb navigation".to_string(),
            ],
        ),
        StrategyType::ExtractFacetMetadata => (
            vec![
                "Enables multidimensional faceted filtering and querying".to_string(),
                "Standardizes frontmatter attributes across document clusters".to_string(),
            ],
            vec![
                "Slightly increases frontmatter schema maintenance overhead".to_string(),
                "Requires consistent tag taxonomy enforcement across authors".to_string(),
            ],
        ),
        StrategyType::ConsolidateIndex => (
            vec![
                "Eliminates intermediate navigation hops to reach destination notes".to_string(),
                "Reduces Dijkstra click-depth budget to <= 2 hops".to_string(),
            ],
            vec![
                "Increases root harness index document size and link density".to_string(),
                "Requires ongoing synchronization as new target notes are added".to_string(),
            ],
        ),
        StrategyType::MergeCandidates => (
            vec![
                "Eliminates redundant near-duplicate documents and semantic fragmentation".to_string(),
                "Establishes a single authoritative canonical source of truth".to_string(),
            ],
            vec![
                "Deletes secondary note paths, requiring link redirects".to_string(),
                "Requires reconciling overlapping headings and duplicate sections".to_string(),
            ],
        ),
        StrategyType::StandardizeTerminology => (
            vec![
                "Reconciles variant terms and typos with the controlled glossary".to_string(),
                "Improves BM25 lexical recall and search consistency".to_string(),
            ],
            vec![
                "Mutates existing document prose lines across the repository".to_string(),
                "May affect historical quotes or external code citations if unverified".to_string(),
            ],
        ),
    }
}
