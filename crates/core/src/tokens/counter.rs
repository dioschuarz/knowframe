//! Exact BPE token counter using tiktoken-rs (cl100k_base) and token summary calculator.

use crate::model::{NoteNode, TokenSummary};
use tiktoken_rs::cl100k_base;

/// Computes the exact number of BPE tokens in a markdown string using cl100k_base.
pub fn count_bpe_tokens(content: &str) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    if content.is_empty() {
        return Ok(0);
    }
    let bpe = cl100k_base()?;
    let tokens = bpe.encode_with_special_tokens(content);
    Ok(tokens.len() as u32)
}

/// Computes Active Prompt Load vs Accessible Subcall Context tokens and evaluates alert status.
pub fn compute_token_summary(
    nodes: &[NoteNode],
    warning_threshold: u64,
    critical_threshold: u64,
) -> TokenSummary {
    let mut active_tokens: u64 = 0;
    let mut subcall_tokens: u64 = 0;

    for node in nodes {
        if node.hop_distance == 0 {
            active_tokens += node.tokens as u64;
        } else if node.hop_distance >= 1 {
            subcall_tokens += node.tokens as u64;
        }
    }

    TokenSummary::new(active_tokens, subcall_tokens, warning_threshold, critical_threshold)
}
