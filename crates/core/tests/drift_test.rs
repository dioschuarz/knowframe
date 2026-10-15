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

    let reverse_index =
        build_reverse_impact_index(dir.path(), &["docs/architecture.md".to_string()]);

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
        code_dependencies: vec![],
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

    // Default config: block_on_technical_drift = false, block_on_unreviewed_docs = false -> should WARN, exit code 0
    let config_warn = AdmissionConfig {
        block_on_unreviewed_docs: false,
        ..AdmissionConfig::default()
    };
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

#[test]
fn test_frontmatter_code_dependencies_extraction_and_injection_t005() {
    use llm_wiki_core::fragment::frontmatter::{parse_code_dependencies, inject_code_dependencies};

    let doc = r#"---
title: System Overview
code_dependencies:
  - crates/core/src/model/mod.rs
  - extension/src/engine.ts
---
# System Overview
Documentation body.
"#;

    let deps = parse_code_dependencies(doc).expect("parse valid code_dependencies");
    assert_eq!(deps, vec!["crates/core/src/model/mod.rs", "extension/src/engine.ts"]);

    // Test non-destructive injection
    let injected = inject_code_dependencies(doc, &["crates/core/src/lib.rs"]).expect("inject code dependencies");
    let re_parsed = parse_code_dependencies(&injected).expect("parse injected dependencies");
    assert_eq!(re_parsed, vec!["crates/core/src/lib.rs"]);
    assert!(injected.contains("# System Overview"));
}

#[test]
fn test_path_normalization_and_containment_t005() {
    use llm_wiki_core::drift::extractor::{
        validate_and_normalize_code_path, validate_path_containment,
        PATH_TRAVERSAL_PROHIBITED,
    };

    // 1. POSIX normalization (Windows backslashes to forward slashes, strip ./)
    let norm1 = validate_and_normalize_code_path(None, r"crates\core\src\lib.rs").expect("normalize backslashes");
    assert_eq!(norm1, "crates/core/src/lib.rs");

    let norm2 = validate_and_normalize_code_path(None, "./extension/src/types/engine.ts").expect("strip ./");
    assert_eq!(norm2, "extension/src/types/engine.ts");

    // 2. Relative to referencing doc
    let norm3 = validate_and_normalize_code_path(Some("notes/architecture/engine.md"), "../code/main.rs")
        .expect("relative path from subfolder");
    assert_eq!(norm3, "notes/code/main.rs");

    // 3. Traversal escapes outside workspace root MUST be rejected with PATH_TRAVERSAL_PROHIBITED
    let err1 = validate_path_containment("../../etc/passwd");
    assert_eq!(err1, Err(PATH_TRAVERSAL_PROHIBITED));

    let err2 = validate_and_normalize_code_path(Some("notes/spec.md"), "../../outside.rs");
    assert_eq!(err2, Err(PATH_TRAVERSAL_PROHIBITED));
}

#[test]
fn test_reverse_impact_index_tracks_dependency_sources_t005() {
    use llm_wiki_core::drift::extractor::extract_all_code_dependencies;
    use llm_wiki_core::drift::ReverseImpactIndex;
    use llm_wiki_core::model::DependencySource;

    let content = r#"---
title: Traceability Test
code_dependencies:
  - crates/core/src/model/mod.rs
  - ../../etc/passwd
---
# Traceability Test
References inline: [Model](crates/core/src/model/mod.rs).
"#;

    let (deps, errors) = extract_all_code_dependencies("notes/traceability.md", content);
    assert!(!deps.is_empty());
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("PATH_TRAVERSAL_PROHIBITED")));

    let mut index = ReverseImpactIndex::new();
    for dep in &deps {
        index.insert_dependency("notes/traceability.md", dep);
    }

    let docs = index.get_docs_for_code("crates/core/src/model/mod.rs");
    assert!(docs.contains("notes/traceability.md"));

    let sources = index.get_sources_for_doc_and_code("notes/traceability.md", "crates/core/src/model/mod.rs");
    assert!(sources.contains(&DependencySource::Frontmatter));
    assert!(sources.contains(&DependencySource::InlineMarkdownLink));
}

#[test]
fn test_architectural_rule_evaluation_from_config_t023() {
    use llm_wiki_core::architecture::rules::{
        classify_change, evaluate_architectural_rules, evaluate_semantic_claim,
    };
    use llm_wiki_core::config::ContextVaultConfig;
    use llm_wiki_core::drift::ReverseImpactIndex;
    use llm_wiki_core::model::{
        ChangeEvaluationCategory, CodeDependency, DependencySource, DeterministicFindingSeverity,
        EpistemicTier,
    };
    use std::collections::HashMap;
    use tempfile::TempDir;

    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // 1. Verify deserialization from YAML
    let yaml_config = r#"
version: "2.0"
architecture:
  rules:
    - id: "NO_CORE_AST_IN_EXTENSION"
      rule_type: "prohibited_dependency"
      source_pattern: "extension/*"
      target_pattern: "crates/core/src/ast/*"
      severity: "error"
      explanation: "Extension must not depend directly on core AST internals"
    - id: "MANDATORY_API_DOCS"
      rule_type: "mandatory_documentation"
      source_pattern: "crates/core/src/api/*"
      required_doc: "docs/api.md"
      severity: "error"
      explanation: "All public API modules must be documented in docs/api.md"
    - id: "MANDATORY_ADR_EXISTS"
      rule_type: "path_exists"
      target_pattern: "docs/ADR-001.md"
      severity: "error"
      explanation: "ADR-001 must exist on disk"
"#;

    let config: ContextVaultConfig = serde_yaml::from_str(yaml_config).expect("parse vault config");
    let arch_config = config.architecture.expect("architecture config exists");
    assert_eq!(arch_config.rules.len(), 3);

    // 2. Setup mock dependencies
    let mut reverse_index = ReverseImpactIndex::new();
    let dep = CodeDependency {
        code_path: "crates/core/src/api/service.rs".to_string(),
        source: DependencySource::Frontmatter,
        line_number: None,
    };
    reverse_index.insert_dependency("docs/other.md", &dep);

    let mut doc_code_deps = HashMap::new();
    doc_code_deps.insert(
        "extension/src/helper.ts".to_string(),
        vec!["crates/core/src/ast/visitor.rs".to_string()],
    );

    // 3. Evaluate rules
    let changed_files = vec!["crates/core/src/api/service.rs".to_string()];
    let findings = evaluate_architectural_rules(
        root,
        &arch_config.rules,
        &changed_files,
        &reverse_index,
        &doc_code_deps,
    );

    // Should detect:
    // 1. Prohibited dependency (extension -> core/ast)
    // 2. Mandatory doc missing (service.rs is cited by docs/other.md, but required is docs/api.md)
    // 3. Path exists violation (docs/ADR-001.md does not exist in temp root)
    assert_eq!(findings.len(), 3);

    let prohibited = findings.iter().find(|f| f.rule_id == "NO_CORE_AST_IN_EXTENSION").unwrap();
    assert_eq!(prohibited.severity, DeterministicFindingSeverity::Error);
    assert_eq!(prohibited.subject, "extension/src/helper.ts");
    assert_eq!(prohibited.epistemic_tier, Some(EpistemicTier::TierAFullyDeterministic));

    let mandatory = findings.iter().find(|f| f.rule_id == "MANDATORY_API_DOCS").unwrap();
    assert_eq!(mandatory.severity, DeterministicFindingSeverity::Error);
    assert_eq!(mandatory.subject, "crates/core/src/api/service.rs");

    let path_missing = findings.iter().find(|f| f.rule_id == "MANDATORY_ADR_EXISTS").unwrap();
    assert_eq!(path_missing.severity, DeterministicFindingSeverity::Error);

    // 4. Test change classification logic
    assert_eq!(
        classify_change(true, false, true, false),
        ChangeEvaluationCategory::RuleViolation
    );
    assert_eq!(
        classify_change(false, true, true, false),
        ChangeEvaluationCategory::ReviewRequired
    );
    assert_eq!(
        classify_change(false, false, true, false),
        ChangeEvaluationCategory::ChangeDetected
    );
    assert_eq!(
        classify_change(false, false, false, true),
        ChangeEvaluationCategory::NotVerifiable
    );

    // 5. Test Epistemic Tier D enforcement for freeform prose
    let (cat, tier) = evaluate_semantic_claim("The system is highly performant and secure", false);
    assert_eq!(cat, ChangeEvaluationCategory::NotVerifiable);
    assert_eq!(tier, Some(EpistemicTier::TierDNotReliablyDeterminable));
}

