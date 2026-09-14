//! Tests for Phase 6: Reverse Impact Index & Technical Drift Engine (Tasks T031–T036).

use std::fs;
use tempfile::TempDir;

use llm_wiki_core::candidate::evaluator::{evaluate_candidate, AdmissionConfig};
use llm_wiki_core::candidate::overlay::CandidateOverlay;
use llm_wiki_core::candidate::{AdmissionDecision, RegressionCategory};
use llm_wiki_core::debt::KnowledgeDebtBaseline;
use llm_wiki_core::delta::updater::{FileDeltaItem, FileDeltaStatus};
use llm_wiki_core::delta::GitDelta;
use llm_wiki_core::drift::detector::{build_reverse_impact_index, detect_technical_drift};
use llm_wiki_core::drift::extractor::{extract_code_citations, is_code_path, normalize_code_path};
use llm_wiki_core::drift::ReverseImpactIndex;
use llm_wiki_core::drift::{DriftConfidence, DriftType};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{NodeStatus, NoteNode};

#[test]
fn test_code_citation_extraction_from_ast() {
    let content = r#"# Architecture Spec
See [Auth Service](src/auth/service.rs) and the token handler at `crates/auth/tokens.ts`.
Also references `AuthService.login()` and [database config](../config/database.json).
"#;

    let citations = extract_code_citations("docs/spec.md", content);
    assert!(!citations.is_empty());

    let paths: Vec<&str> = citations
        .iter()
        .map(|c| c.code_path.as_str())
        .filter(|p| !p.is_empty())
        .collect();

    assert!(paths.contains(&"docs/src/auth/service.rs") || paths.contains(&"src/auth/service.rs"));
    assert!(paths.contains(&"crates/auth/tokens.ts"));
    assert!(paths.contains(&"config/database.json"));

    let symbols: Vec<&str> = citations
        .iter()
        .filter_map(|c| c.symbol.as_deref())
        .collect();
    assert!(symbols.contains(&"AuthService.login"));
}

#[test]
fn test_reverse_impact_index_lookups() {
    let mut index = ReverseImpactIndex::new();

    let cit1 = llm_wiki_core::drift::extractor::CodeCitation {
        code_path: "src/auth/tokens.rs".to_string(),
        symbol: Some("TokenManager.rotate".to_string()),
        line_number: 14,
    };
    let cit2 = llm_wiki_core::drift::extractor::CodeCitation {
        code_path: "src/auth/tokens.rs".to_string(),
        symbol: None,
        line_number: 45,
    };

    index.insert_citation("docs/security.md", &cit1);
    index.insert_citation("docs/architecture.md", &cit2);

    let docs = index.get_docs_for_code("src/auth/tokens.rs");
    assert_eq!(docs.len(), 2);
    assert!(docs.contains("docs/security.md"));
    assert!(docs.contains("docs/architecture.md"));

    let sym_docs = index.get_docs_for_symbol("TokenManager.rotate");
    assert_eq!(sym_docs.len(), 1);
    assert!(sym_docs.contains("docs/security.md"));

    let code_paths = index.get_code_for_doc("docs/security.md");
    assert!(code_paths.contains("src/auth/tokens.rs"));
}

#[test]
fn test_staged_code_change_detection() {
    assert!(is_code_path("src/auth/tokens.rs"));
    assert!(is_code_path("extension/src/extension.ts"));
    assert!(is_code_path("backend/main.py"));
    assert!(!is_code_path("docs/spec.md"));
    assert!(!is_code_path("image.png"));

    let norm = normalize_code_path("docs/architecture/auth.md", "../database/schema.sql");
    assert_eq!(norm, "docs/database/schema.sql");
}

#[test]
fn test_technical_drift_on_deleted_service() {
    let dir = TempDir::new().unwrap();

    let doc_path = dir.path().join("docs").join("architecture.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    fs::write(
        &doc_path,
        "# Architecture\nUses legacy service `src/services/billing.rs` for payments.\n",
    )
    .unwrap();

    let reverse_index = build_reverse_impact_index(
        dir.path(),
        &["docs/architecture.md".to_string()],
    );

    // Staged deletion of src/services/billing.rs without modifying docs/architecture.md
    let staged_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "src/services/billing.rs".to_string(),
            status: FileDeltaStatus::Deleted,
        }],
    };

    let (findings, sigs) = detect_technical_drift(dir.path(), &staged_delta, &reverse_index);

    assert_eq!(findings.len(), 1);
    assert_eq!(sigs.len(), 1);
    let f = &findings[0];
    assert_eq!(f.doc_id, "docs/architecture.md");
    assert_eq!(f.code_target, "src/services/billing.rs");
    assert_eq!(f.drift_type, DriftType::DeletedCodeFile);
    assert_eq!(f.confidence, DriftConfidence::High);
    assert!(f.explanation.contains("deleted source file"));
}

#[test]
fn test_drift_provenance_diagnostics() {
    let dir = TempDir::new().unwrap();

    let doc_path = dir.path().join("api.md");
    fs::write(
        &doc_path,
        "# API Spec\nLine 2 info.\nLine 3 cites `src/handler.rs`.\n",
    )
    .unwrap();

    let reverse_index = build_reverse_impact_index(dir.path(), &["api.md".to_string()]);

    let staged_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "src/handler.rs".to_string(),
            status: FileDeltaStatus::Deleted,
        }],
    };

    let (findings, _) = detect_technical_drift(dir.path(), &staged_delta, &reverse_index);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].citation_line, 3);
    assert!(findings[0].explanation.contains("line 3"));
}

#[test]
fn test_candidate_technical_drift_warning() {
    let dir = TempDir::new().unwrap();

    let doc_path = dir.path().join("doc.md");
    fs::write(
        &doc_path,
        "# System Doc\nReferences `src/legacy_engine.rs`.\n",
    )
    .unwrap();

    let mut base_graph = GraphTopology::new();
    base_graph.add_node(NoteNode {
        id: "doc.md".to_string(),
        file_path: doc_path.to_string_lossy().to_string(),
        label: "System Doc".to_string(),
        tags: vec![],
        tokens: 30,
        in_degree: 1,
        out_degree: 0,
        betweenness: 0.0,
        hop_distance: 1,
        status: NodeStatus::Active,
        color: "#64748B".to_string(),
        broken_links: 0,
        fragments: vec![],
        freshness: None,
    });

    let baseline = KnowledgeDebtBaseline::default();

    // Stage code file deletion without touching doc.md
    let staged_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "src/legacy_engine.rs".to_string(),
            status: FileDeltaStatus::Deleted,
        }],
    };

    let overlay = CandidateOverlay::new(&base_graph, dir.path(), &staged_delta);

    // Default config: block_on_technical_drift = false -> should WARN, exit code 0
    let config_warn = AdmissionConfig::default();
    let report_warn = evaluate_candidate(&overlay, &baseline, vec![], vec![], config_warn);

    assert_eq!(report_warn.decision, AdmissionDecision::Warn);
    assert_eq!(report_warn.exit_code, 0);
    assert!(!report_warn.warnings.is_empty());
    assert!(report_warn
        .warnings
        .iter()
        .any(|w| w.message.contains("Technical drift")));

    // Strict config: block_on_technical_drift = true -> should BLOCK, exit code 1
    let config_block = AdmissionConfig {
        block_on_technical_drift: true,
        ..AdmissionConfig::default()
    };
    let report_block = evaluate_candidate(&overlay, &baseline, vec![], vec![], config_block);

    assert_eq!(report_block.decision, AdmissionDecision::Block);
    assert_eq!(report_block.exit_code, 1);
    assert!(report_block
        .regressions
        .iter()
        .any(|r| r.category == RegressionCategory::TechnicalDriftContradiction));
}
