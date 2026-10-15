//! 64-bit raw integer numeric model.
//! Implements FR-030, NFR-007, and Constitution Invariants IX & X:
//! Raw numeric integrity requires exact 64-bit integer values across all storage,
//! metric evaluation, and RPC interfaces, deferring compact formatting strictly to UI presentation.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tiktoken_rs::{cl100k_base, CoreBPE};

static BPE_TOKENIZER: OnceLock<CoreBPE> = OnceLock::new();

fn get_bpe_tokenizer() -> &'static CoreBPE {
    BPE_TOKENIZER.get_or_init(|| {
        cl100k_base().expect("Failed to initialize cl100k_base BPE tokenizer")
    })
}

/// Computes exact cl100k_base BPE token count for text.
pub fn count_exact_tokens(text: &str) -> u64 {
    let bpe = get_bpe_tokenizer();
    bpe.encode_with_special_tokens(text).len() as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RawTokenMetrics {
    pub total_tokens: u64,
    pub prose_tokens: u64,
    pub code_tokens: u64,
    pub heading_tokens: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RawGraphMetrics {
    pub total_documents: u64,
    pub total_references: u64,
    pub resolved_references: u64,
    pub broken_references: u64,
    pub orphan_documents: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RawDebtMetrics {
    pub legacy_debt_count: u64,
    pub new_regressions_count: u64,
    pub resolved_debt_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RawNumericMetrics {
    pub tokens: RawTokenMetrics,
    pub graph: RawGraphMetrics,
    pub debt: RawDebtMetrics,
}
