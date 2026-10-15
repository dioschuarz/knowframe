//! Unit test for brownfield baseline isolation (.context-vault/baseline.json).
//! Validates FR-028, SC-003, and Constitution Invariant XI.

use llm_wiki_core::health::baseline::{BaselineManager, DebtItem};
use tempfile::tempdir;

#[test]
fn test_brownfield_baseline_isolation() {
    let dir = tempdir().unwrap();
    let baseline_path = dir.path().join("baseline.json");

    let mut manager = BaselineManager::new(&baseline_path);

    // Add legacy finding to baseline
    manager.add_item(DebtItem {
        rule_key: "broken_link".to_string(),
        target_file: "legacy.md".to_string(),
        line_number: 15,
        fingerprint: "hash_legacy_1".to_string(),
    });
    manager.save().unwrap();

    // Reload baseline
    let loaded = BaselineManager::load(&baseline_path).unwrap();
    assert_eq!(loaded.items().len(), 1);

    // Verify classification: legacy vs net-new
    assert!(loaded.is_baselined("hash_legacy_1"));
    assert!(!loaded.is_baselined("hash_new_regression"));
}
