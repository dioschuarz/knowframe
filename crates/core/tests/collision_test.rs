use std::fs;
use tempfile::TempDir;

use llm_wiki_core::candidate::evaluator::{evaluate_candidate, AdmissionConfig};
use llm_wiki_core::candidate::overlay::CandidateOverlay;
use llm_wiki_core::candidate::{AdmissionDecision, RegressionCategory, RegressionSeverity};
use llm_wiki_core::collision::canonical::{evaluate_canonical_candidate, DocumentMetadataContext};
use llm_wiki_core::collision::fingerprint::{extract_fingerprint, InvertedIndex};
use llm_wiki_core::collision::scorer::score_collision;
use llm_wiki_core::collision::CollisionClassification;
use llm_wiki_core::debt::KnowledgeDebtBaseline;
use llm_wiki_core::delta::updater::{FileDeltaItem, FileDeltaStatus};
use llm_wiki_core::delta::GitDelta;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{NodeStatus, NoteNode};

#[test]
fn test_multi_signal_fingerprint_extraction() {
    let doc_id = "notes/auth_service.md";
    let content = "# Authentication Service Architecture\n\nThis module handles OAuth2 tokens and session validation.\n\n## Refresh Token Rotation\nTokens are automatically rotated upon expiry.\n";
    let headings = vec![
        "Authentication Service Architecture".to_string(),
        "Refresh Token Rotation".to_string(),
    ];
    let tags = vec!["#security".to_string(), "#auth".to_string()];

    let fp = extract_fingerprint(doc_id, content, &headings, &tags);

    assert_eq!(fp.doc_id, "notes/auth_service.md");
    assert!(fp.title_tokens.contains("auth"));
    assert!(fp.title_tokens.contains("service"));
    assert!(fp.heading_tokens.contains("authentication"));
    assert!(fp.heading_tokens.contains("refresh"));
    assert!(fp.heading_tokens.contains("rotation"));
    assert!(fp.tags.contains("security"));
    assert!(fp.tags.contains("auth"));
    assert!(!fp.term_frequencies.is_empty());
    assert!(!fp.ngrams.is_empty());
}

#[test]
fn test_inverted_index_top_k_retrieval() {
    let mut index = InvertedIndex::new();

    let fp1 = extract_fingerprint(
        "notes/auth_oauth.md",
        "OAuth2 token exchange service and authentication flow.",
        &["OAuth2 Service".to_string()],
        &["auth".to_string()],
    );
    let fp2 = extract_fingerprint(
        "notes/database_sql.md",
        "PostgreSQL connection pool and relational schema migration.",
        &["Database Config".to_string()],
        &["database".to_string()],
    );
    let fp3 = extract_fingerprint(
        "notes/auth_login.md",
        "Login controller handling OAuth2 token validation and credentials.",
        &["Login Service".to_string()],
        &["auth".to_string()],
    );

    index.insert(fp1.clone());
    index.insert(fp2.clone());
    index.insert(fp3.clone());

    // Query with a new authentication document
    let query_fp = extract_fingerprint(
        "notes/new_oauth_doc.md",
        "OAuth2 token rotation and login authentication controller.",
        &["OAuth Authentication".to_string()],
        &["auth".to_string()],
    );

    let candidates = index.query_candidates(&query_fp, 2);
    assert_eq!(candidates.len(), 2);
    // Candidates should be auth_oauth.md and auth_login.md, NOT database_sql.md
    assert!(candidates.contains(&"notes/auth_oauth.md".to_string()));
    assert!(candidates.contains(&"notes/auth_login.md".to_string()));
    assert!(!candidates.contains(&"notes/database_sql.md".to_string()));
}

#[test]
fn test_composite_similarity_calculation() {
    let content_a = "# Token Authentication\n\nHandles JWT access tokens, token revocation, and user session management.";
    let content_b = "# Token Authentication\n\nHandles JWT access tokens, token revocation, and user session management.";

    let fp_a = extract_fingerprint(
        "notes/token_auth.md",
        content_a,
        &["Token Authentication".to_string()],
        &["auth".to_string()],
    );
    let fp_b = extract_fingerprint(
        "notes/token_auth_duplicate.md",
        content_b,
        &["Token Authentication".to_string()],
        &["auth".to_string()],
    );

    let collision = score_collision(&fp_a, &fp_b, 0.75).expect("Identical content should collide");
    assert!(
        collision.composite_similarity >= 0.85,
        "Identical documents should have similarity >= 0.85 (got {})",
        collision.composite_similarity
    );
    assert_eq!(
        collision.classification,
        CollisionClassification::PotentialDuplicate
    );
    assert!(!collision.shared_key_terms.is_empty());
}

#[test]
fn test_collision_classification_heuristics() {
    // 1. Potential Summary: short condensed version of a long document
    let long_content = "# Architecture Overview\n\n\
        The system implements an event-driven architecture using distributed messaging queues.\n\n\
        ## Microservices Topology\n\
        Services communicate asynchronously via Kafka topics with schema validation.\n\n\
        ## Caching Layer\n\
        Redis clusters provide sub-millisecond retrieval for frequently accessed sessions.\n\n\
        ## Persistence Store\n\
        PostgreSQL instances operate in primary-replica configurations with automated failover.\n\n\
        ## Security Controls\n\
        TLS encryption protects all transit data while AES-256 secures data at rest.\n";
    let short_content = "# Architecture Overview\n\nSummary of system architecture covering microservices, caching, persistence, and security controls.\n";

    let fp_long = extract_fingerprint(
        "notes/arch.md",
        long_content,
        &[
            "Architecture Overview".to_string(),
            "Microservices Topology".to_string(),
            "Caching Layer".to_string(),
            "Persistence Store".to_string(),
            "Security Controls".to_string(),
        ],
        &[],
    );
    let fp_short = extract_fingerprint(
        "notes/arch_summary.md",
        short_content,
        &["Architecture Overview".to_string()],
        &[],
    );

    let col_summary = score_collision(&fp_long, &fp_short, 0.25).expect("Should detect overlap");
    assert_eq!(
        col_summary.classification,
        CollisionClassification::PotentialSummary
    );

    // 2. Potential Specialization: deeper heading hierarchy with high term overlap
    let general_content = "# Logging\nGeneral logging patterns for our services.";
    let deep_content = "# Logging Framework\nGeneral logging patterns.\n## Structured JSON Log Format\n## Retention Policy\n## Log Shippers\n## Alerting Rules\n";

    let fp_gen = extract_fingerprint(
        "notes/logging.md",
        general_content,
        &["Logging".to_string()],
        &[],
    );
    let fp_deep = extract_fingerprint(
        "notes/logging_deep.md",
        deep_content,
        &[
            "Logging Framework".to_string(),
            "Structured JSON Log Format".to_string(),
            "Retention Policy".to_string(),
            "Log Shippers".to_string(),
            "Alerting Rules".to_string(),
        ],
        &[],
    );

    let col_spec = score_collision(&fp_gen, &fp_deep, 0.25).expect("Should detect overlap");
    assert_eq!(
        col_spec.classification,
        CollisionClassification::PotentialSpecialization
    );
}

#[test]
fn test_canonical_candidate_proposal() {
    let ctx_hub = DocumentMetadataContext {
        doc_id: "notes/auth.md",
        is_entrypoint_referenced: true,
        degree: 8,
        betweenness: 0.45,
        heading_count: 5,
        token_count: 1200,
        commit_timestamp: Some(1000),
    };

    let ctx_duplicate = DocumentMetadataContext {
        doc_id: "notes/auth_draft.md",
        is_entrypoint_referenced: false,
        degree: 1,
        betweenness: 0.0,
        heading_count: 2,
        token_count: 400,
        commit_timestamp: Some(2000),
    };

    let result = evaluate_canonical_candidate(&ctx_hub, &ctx_duplicate);
    assert_eq!(
        result.candidate_id, "notes/auth.md",
        "Hub note with entrypoint reference and higher centrality must be designated canonical"
    );
    assert!(result.confidence >= 0.80);
    assert!(result
        .evidence
        .iter()
        .any(|e| e.contains("AGENTS.md / README.md")));
}

#[test]
fn test_candidate_collision_blocking() {
    let dir = TempDir::new().unwrap();

    // Base graph with canonical note
    let mut base_graph = GraphTopology::new();
    let auth_path = dir.path().join("auth.md");
    fs::write(
        &auth_path,
        "# Authentication Service\nJWT session token rotation and security credentials verification.\n",
    )
    .unwrap();

    base_graph.add_node(NoteNode {
        id: "auth.md".to_string(),
        file_path: auth_path.to_string_lossy().to_string(),
        label: "Authentication Service".to_string(),
        tags: Vec::new(),
        tokens: 50,
        in_degree: 1,
        out_degree: 0,
        betweenness: 0.1,
        hop_distance: 1,
        status: NodeStatus::Active,
        color: "#64748B".to_string(),
        broken_links: 0,
        fragments: Vec::new(),
        freshness: None,
    });
    base_graph.set_headings(
        "auth.md".to_string(),
        vec!["Authentication Service".to_string()],
    );

    // Initial baseline without collisions
    let baseline = KnowledgeDebtBaseline::default();

    // Stage a near-identical duplicate
    let dup_path = dir.path().join("auth_copy.md");
    fs::write(
        &dup_path,
        "# Authentication Service\nJWT session token rotation and security credentials verification.\n",
    )
    .unwrap();

    let staged_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "auth_copy.md".to_string(),
            status: FileDeltaStatus::Added,
        }],
    };

    let overlay = CandidateOverlay::new(&base_graph, dir.path(), &staged_delta);
    let report = evaluate_candidate(
        &overlay,
        &baseline,
        vec![],
        vec![],
        AdmissionConfig::default(),
    );

    // High similarity (100% duplicate) MUST trigger BLOCK (exit code 1)
    assert_eq!(report.decision, AdmissionDecision::Block);
    assert_eq!(report.exit_code, 1);
    assert!(
        report
            .regressions
            .iter()
            .any(|r| r.category == RegressionCategory::SemanticCollision
                && r.severity == RegressionSeverity::Blocker)
    );
}
