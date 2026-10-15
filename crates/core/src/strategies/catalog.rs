//! Candidate evaluators for the 6 transformation strategies.
//! Implements FR-032.

use crate::model::StrategyType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateStrategyEvaluation {
    pub strategy: StrategyType,
    pub target_document: String,
    pub raw_fit_score: f64,
    pub estimated_line_churn: usize,
    pub target_files_count: usize,
}

/// Evaluates applicability of PreserveDocument strategy.
pub fn evaluate_preserve_document(
    doc_id: &str,
    cohesion: f64,
    tokens: u64,
) -> CandidateStrategyEvaluation {
    let fit = if tokens > 3000 && cohesion >= 0.70 {
        0.85
    } else {
        0.40
    };
    CandidateStrategyEvaluation {
        strategy: StrategyType::PreserveDocument,
        target_document: doc_id.to_string(),
        raw_fit_score: fit,
        estimated_line_churn: 0,
        target_files_count: 1,
    }
}

/// Evaluates applicability of SplitDecompose strategy.
pub fn evaluate_split_decompose(
    doc_id: &str,
    tokens: u64,
    semantic_diversity: f64,
) -> CandidateStrategyEvaluation {
    let fit = if tokens > 5000 && semantic_diversity >= 0.60 {
        0.90
    } else if tokens > 3000 {
        0.65
    } else {
        0.30
    };
    CandidateStrategyEvaluation {
        strategy: StrategyType::SplitDecompose,
        target_document: doc_id.to_string(),
        raw_fit_score: fit,
        estimated_line_churn: 120,
        target_files_count: 3,
    }
}

/// Evaluates applicability of ExtractFacetMetadata strategy.
pub fn evaluate_extract_facet_metadata(
    doc_id: &str,
    facet_orthogonality: f64,
) -> CandidateStrategyEvaluation {
    let fit = if facet_orthogonality >= 0.70 {
        0.80
    } else {
        0.45
    };
    CandidateStrategyEvaluation {
        strategy: StrategyType::ExtractFacetMetadata,
        target_document: doc_id.to_string(),
        raw_fit_score: fit,
        estimated_line_churn: 25,
        target_files_count: 1,
    }
}

/// Evaluates applicability of ConsolidateIndex strategy.
pub fn evaluate_consolidate_index(
    doc_id: &str,
    hop_depth: usize,
) -> CandidateStrategyEvaluation {
    let fit = if hop_depth > 3 { 0.85 } else { 0.40 };
    CandidateStrategyEvaluation {
        strategy: StrategyType::ConsolidateIndex,
        target_document: doc_id.to_string(),
        raw_fit_score: fit,
        estimated_line_churn: 40,
        target_files_count: 2,
    }
}

/// Evaluates applicability of MergeCandidates strategy.
pub fn evaluate_merge_candidates(
    doc_a: &str,
    _doc_b: &str,
    similarity: f64,
) -> CandidateStrategyEvaluation {
    let fit = if similarity >= 0.80 { 0.85 } else { 0.35 };
    CandidateStrategyEvaluation {
        strategy: StrategyType::MergeCandidates,
        target_document: doc_a.to_string(),
        raw_fit_score: fit,
        estimated_line_churn: 60,
        target_files_count: 2,
    }
}

/// Evaluates applicability of StandardizeTerminology strategy.
pub fn evaluate_standardize_terminology(
    doc_id: &str,
    typo_count: usize,
) -> CandidateStrategyEvaluation {
    let fit = if typo_count > 0 { 0.85 } else { 0.20 };
    CandidateStrategyEvaluation {
        strategy: StrategyType::StandardizeTerminology,
        target_document: doc_id.to_string(),
        raw_fit_score: fit,
        estimated_line_churn: typo_count * 5,
        target_files_count: 1,
    }
}
