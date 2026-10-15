//! Determinism integration test verifying bit-exact BLAKE3 analysis_id reproducibility.
//! Validates FR-002, NFR-002, and SC-002.

use llm_wiki_core::epistemic::{compute_analysis_id, METHODOLOGY_VERSION};
use llm_wiki_core::model::OptimizationProfile;

#[test]
fn test_bit_exact_analysis_id_determinism() {
    let commit_hash = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
    let config_bytes = b"scan:\n  exclude:\n    - target/**\n";
    let profile = OptimizationProfile::ai_oriented();
    let profile_bytes = serde_json::to_vec(&profile).expect("Failed to serialize profile");

    let run1 = compute_analysis_id(commit_hash, config_bytes, &profile_bytes, METHODOLOGY_VERSION);
    let run2 = compute_analysis_id(commit_hash, config_bytes, &profile_bytes, METHODOLOGY_VERSION);
    let run3 = compute_analysis_id(commit_hash, config_bytes, &profile_bytes, METHODOLOGY_VERSION);

    assert_eq!(run1, run2, "Analysis IDs must be bit-exact across executions");
    assert_eq!(run2, run3, "Analysis IDs must be bit-exact across executions");
    assert_eq!(run1.len(), 64, "BLAKE3 hex digest must be 64 characters");
}

#[test]
fn test_analysis_id_sensitivity_to_profile_change() {
    let commit_hash = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
    let config_bytes = b"scan:\n  exclude:\n    - target/**\n";

    let ai_profile = OptimizationProfile::ai_oriented();
    let human_profile = OptimizationProfile::human_oriented();

    let ai_bytes = serde_json::to_vec(&ai_profile).unwrap();
    let human_bytes = serde_json::to_vec(&human_profile).unwrap();

    let ai_id = compute_analysis_id(commit_hash, config_bytes, &ai_bytes, METHODOLOGY_VERSION);
    let human_id = compute_analysis_id(commit_hash, config_bytes, &human_bytes, METHODOLOGY_VERSION);

    assert_ne!(
        ai_id, human_id,
        "Switching optimization profile must produce a distinct analysis_id"
    );
}
