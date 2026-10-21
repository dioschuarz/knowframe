use llm_wiki_core::drift::extractor::CodeCitation;
use llm_wiki_core::drift::freshness::{
    compute_freshness_from_signals, detect_deleted_symbols, evaluate_document_freshness,
    verify_code_citations_disk, FreshnessTier,
};
use llm_wiki_core::drift::{DriftConfidence, DriftType, TechnicalDriftFinding};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_signal_1_code_citation_disk_verification() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Create an existing code file
    fs::create_dir_all(root.join("src/auth")).unwrap();
    fs::write(root.join("src/auth/service.rs"), "pub fn login() {}").unwrap();

    let citations = vec![
        CodeCitation {
            code_path: "src/auth/service.rs".to_string(),
            symbol: None,
            line_number: 10,
        },
        CodeCitation {
            code_path: "src/auth/missing_provider.rs".to_string(),
            symbol: None,
            line_number: 20,
        },
    ];

    let missing = verify_code_citations_disk(root, &citations);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0], "src/auth/missing_provider.rs");

    // Compute evidence with 1 missing citation (-15 pts) -> 85.0 (Fresh)
    let evidence = compute_freshness_from_signals(missing, vec![], 0, vec![], None);
    assert_eq!(evidence.score, 85.0);
    assert_eq!(evidence.tier, FreshnessTier::Fresh);
    assert_eq!(evidence.missing_citations.len(), 1);
    assert!(evidence.reasons[0].contains("missing_provider.rs"));
}

#[test]
fn test_signal_2_git_commit_chronology_capped_deduction() {
    // 1 commit = -10 pts -> score 90.0
    let ev1 =
        compute_freshness_from_signals(vec![], vec!["src/lib.rs".to_string()], 1, vec![], None);
    assert_eq!(ev1.score, 90.0);
    assert_eq!(ev1.tier, FreshnessTier::Fresh);

    // 2 commits = -20 pts -> score 80.0
    let ev2 =
        compute_freshness_from_signals(vec![], vec!["src/lib.rs".to_string()], 2, vec![], None);
    assert_eq!(ev2.score, 80.0);
    assert_eq!(ev2.tier, FreshnessTier::Fresh);

    // 3 commits = -30 pts -> score 70.0 (PotentiallyStale)
    let ev3 =
        compute_freshness_from_signals(vec![], vec!["src/lib.rs".to_string()], 3, vec![], None);
    assert_eq!(ev3.score, 70.0);
    assert_eq!(ev3.tier, FreshnessTier::PotentiallyStale);

    // 10 commits -> capped at -30 pts max -> score 70.0 (PotentiallyStale)
    let ev10 =
        compute_freshness_from_signals(vec![], vec!["src/lib.rs".to_string()], 10, vec![], None);
    assert_eq!(ev10.score, 70.0);
    assert_eq!(ev10.tier, FreshnessTier::PotentiallyStale);
}

#[test]
fn test_signal_3_deleted_symbol_impact() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("src")).unwrap();
    // File exists but only has new_func(), not old_func()
    fs::write(root.join("src/service.rs"), "pub fn new_func() {}").unwrap();

    let citations = vec![
        CodeCitation {
            code_path: "src/service.rs".to_string(),
            symbol: Some("old_func".to_string()),
            line_number: 5,
        },
        CodeCitation {
            code_path: "src/service.rs".to_string(),
            symbol: Some("new_func".to_string()),
            line_number: 6,
        },
    ];

    let valid_paths = vec!["src/service.rs".to_string()];
    let deleted = detect_deleted_symbols(root, &citations, &valid_paths, &[]);

    assert_eq!(deleted.len(), 1);
    assert_eq!(deleted[0], "old_func");

    // Also verify when drift finding reports deleted symbol
    let drifts = vec![TechnicalDriftFinding {
        doc_id: "docs/auth.md".to_string(),
        code_target: "src/legacy.rs".to_string(),
        symbol_name: Some("LegacyAuth".to_string()),
        citation_line: 12,
        drift_type: DriftType::DeletedSymbol,
        confidence: DriftConfidence::High,
        explanation: "LegacyAuth removed".to_string(),
    }];

    let deleted_with_drifts = detect_deleted_symbols(root, &citations, &valid_paths, &drifts);
    assert_eq!(deleted_with_drifts.len(), 2);
    assert!(deleted_with_drifts.contains(&"old_func".to_string()));
    assert!(deleted_with_drifts.contains(&"LegacyAuth".to_string()));

    // Deleted symbol deduction: -20 pts each -> -40 pts -> 60.0 (PotentiallyStale)
    let ev = compute_freshness_from_signals(vec![], vec![], 0, deleted_with_drifts, None);
    assert_eq!(ev.score, 60.0);
    assert_eq!(ev.tier, FreshnessTier::PotentiallyStale);
}

#[test]
fn test_signal_4_canonical_contradiction() {
    // Canonical contradiction deduction: -25 pts -> 75.0 (PotentiallyStale)
    let ev = compute_freshness_from_signals(
        vec![],
        vec![],
        0,
        vec![],
        Some("docs/architecture/canonical_auth.md".to_string()),
    );
    assert_eq!(ev.score, 75.0);
    assert_eq!(ev.tier, FreshnessTier::PotentiallyStale);
    assert_eq!(
        ev.canonical_contradiction,
        Some("docs/architecture/canonical_auth.md".to_string())
    );
}

#[test]
fn test_composite_freshness_evaluation_and_stale_tier() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Document citing non-existent code files
    let content = r#"# Architecture
Refer to `src/nonexistent/module.rs` and `src/nonexistent/provider.rs`.
"#;
    let ev = evaluate_document_freshness(
        root,
        "docs/arch.md",
        content,
        Some("docs/canonical.md".to_string()),
        &[],
    );

    // Missing citations: 2 * -15 = -30 pts
    // Canonical contradiction: -25 pts
    // Score: 100 - 55 = 45.0 (< 60.0) -> Stale
    assert_eq!(ev.score, 45.0);
    assert_eq!(ev.tier, FreshnessTier::Stale);
    assert_eq!(ev.missing_citations.len(), 2);
    assert!(ev.canonical_contradiction.is_some());
    assert!(!ev.reasons.is_empty());
}

#[test]
fn test_implementation_changes_emit_review_required_never_rule_violation_t027() {
    use llm_wiki_core::architecture::rules::{can_auto_update_architecture, classify_change, evaluate_architectural_rules};
    use llm_wiki_core::delta::updater::{FileDeltaItem, FileDeltaStatus};
    use llm_wiki_core::drift::review::calculate_affected_documents;
    use llm_wiki_core::drift::ReverseImpactIndex;
    use llm_wiki_core::model::{ChangeEvaluationCategory, CodeDependency, DependencySource, ReviewStatus};
    use std::collections::HashMap;

    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // 1. Create architectural documentation file on disk
    let doc_path = root.join("docs/auth_architecture.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    let initial_doc_content = r#"---
title: Auth Architecture
code_dependencies:
  - src/auth.rs
---
# Auth Architecture
The authentication service handles tokens.
"#;
    fs::write(&doc_path, initial_doc_content).unwrap();

    // 2. Setup code change
    let changed_code = "src/auth.rs".to_string();
    let mut reverse_index = ReverseImpactIndex::new();
    let dep = CodeDependency {
        code_path: changed_code.clone(),
        source: DependencySource::Frontmatter,
        line_number: None,
    };
    reverse_index.insert_dependency("docs/auth_architecture.md", &dep);

    // 3. Evaluate architectural rules (no rule violations configured)
    let findings = evaluate_architectural_rules(
        root,
        &[], // No architectural constraints violated
        &[changed_code.clone()],
        &reverse_index,
        &HashMap::new(),
    );
    assert!(findings.is_empty(), "Expected 0 rule violations for normal code diff");

    // 4. Classify change category
    let category = classify_change(
        !findings.is_empty(), // has_rule_violation = false
        true,                 // has_citing_docs = true
        true,                 // is_code_modified = true
        false,                // is_freeform_claim = false
    );
    // MUST be ReviewRequired, NEVER RuleViolation! (FR-012, SC-003)
    assert_eq!(category, ChangeEvaluationCategory::ReviewRequired);
    assert_ne!(category, ChangeEvaluationCategory::RuleViolation);

    // 5. Calculate affected documents review obligations
    let delta_items = vec![FileDeltaItem {
        path: changed_code,
        status: FileDeltaStatus::Modified,
    }];
    let non_blocking_config = llm_wiki_core::model::ReviewGatingConfig {
        enforce_doc_review: false,
        block_on_legacy_debt: false,
        block_on_missing_fragments: false,
    };
    let affected_pending = calculate_affected_documents(
        root,
        &delta_items,
        &reverse_index,
        Some(&non_blocking_config),
        None,
        None,
    );
    assert_eq!(affected_pending.obligations.len(), 1);
    assert_eq!(affected_pending.obligations[0].doc_id, "docs/auth_architecture.md");
    assert_eq!(affected_pending.obligations[0].status, ReviewStatus::Pending);

    // When gating config enforces review, obligation transitions to Blocked
    let blocking_config = llm_wiki_core::model::ReviewGatingConfig {
        enforce_doc_review: true,
        block_on_legacy_debt: false,
        block_on_missing_fragments: true,
    };
    let affected_blocked = calculate_affected_documents(
        root,
        &delta_items,
        &reverse_index,
        Some(&blocking_config),
        None,
        None,
    );
    assert_eq!(affected_blocked.obligations[0].status, ReviewStatus::Blocked);

    // 6. Negative assertion: Ordinary code diffs NEVER mutate architectural documentation notes
    // on disk without explicit user consent (FR-014, Invariant XVI).
    let doc_content_after = fs::read_to_string(&doc_path).unwrap();
    assert_eq!(
        initial_doc_content, doc_content_after,
        "Documentation note on disk was unexpectedly mutated by code diff analysis!"
    );
    assert!(!can_auto_update_architecture(false));
}
