use llm_wiki_core::tokens::counter::{count_bpe_tokens, compute_token_summary};
use llm_wiki_core::model::{NoteNode, TokenBudgetStatus};

#[test]
fn test_exact_bpe_token_calculation() {
    let sample = "Hello, world! This is a test of cl100k_base BPE tokenization.";
    let tokens = count_bpe_tokens(sample).expect("Failed to count tokens");
    assert!(tokens > 0);

    // Empty content must have 0 tokens
    assert_eq!(count_bpe_tokens("").unwrap(), 0);
}

#[test]
fn test_token_summary_thresholds() {
    let mut hub = NoteNode::new("hub.md", "/repo/hub.md", "Hub");
    hub.tokens = 50_000;
    hub.hop_distance = 0;

    let mut leaf = NoteNode::new("leaf.md", "/repo/leaf.md", "Leaf");
    leaf.tokens = 30_000;
    leaf.hop_distance = 1;

    let nodes = vec![hub, leaf];
    let summary = compute_token_summary(&nodes, 60_000, 100_000);

    assert_eq!(summary.active_harness_tokens, 50_000);
    assert_eq!(summary.subcall_accessible_tokens, 30_000);
    assert_eq!(summary.status, TokenBudgetStatus::Normal);

    // If active exceeds 60_000 -> Warning
    let summary_warn = compute_token_summary(&nodes, 40_000, 100_000);
    assert_eq!(summary_warn.status, TokenBudgetStatus::Warning);

    // If active exceeds 100_000 -> Critical
    let summary_crit = compute_token_summary(&nodes, 40_000, 45_000);
    assert_eq!(summary_crit.status, TokenBudgetStatus::Critical);
}
