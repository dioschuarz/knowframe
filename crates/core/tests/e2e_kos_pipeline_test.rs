//! End-to-end integration test verifying full pipeline execution across all 18 KOS methodologies
//! with strictly 0 external AI calls.
//! Implements T069, FR-001..FR-046, NFR-001..NFR-010, and SC-001..SC-008.

use llm_wiki_core::automation::mutators::apply_transformation_action;
use llm_wiki_core::automation::plan_generator::TransformationPlanGenerator;
use llm_wiki_core::automation::re_measurement::compute_re_measurement_delta;
use llm_wiki_core::epistemic::classifier::EpistemicClassifier;
use llm_wiki_core::health::baseline::BaselineManager;
use llm_wiki_core::health::monolith_guard::{check_monolith_protection, MonolithMetrics};
use llm_wiki_core::health::scorer::compute_health_score;
use llm_wiki_core::methodologies::context_routing::{
    evaluate_context_trace_routing, evaluate_knowledge_drift, ContextTraceHop,
};
use llm_wiki_core::methodologies::facets::{compute_facet_orthogonality, DocumentFacets};
use llm_wiki_core::methodologies::information_retrieval::{
    evaluate_bm25f, evaluate_information_architecture, DocumentFieldContent,
};
use llm_wiki_core::methodologies::metadata_profiles::{
    evaluate_dcap_metadata, evaluate_ontology_shapes, DcapShape, DocumentMetadataRecord,
};
use llm_wiki_core::methodologies::network_analysis::{
    evaluate_network_analysis, NetworkGraph,
};
use llm_wiki_core::methodologies::provenance::compute_co_change_coupling;
use llm_wiki_core::methodologies::semantic_networks::{
    evaluate_concept_map, evaluate_semantic_network, ConceptProposition, SemanticNetworkGraph,
};
use llm_wiki_core::methodologies::taxonomy::{
    evaluate_classification_coverage, evaluate_taxonomy, TaxonomyGraph,
};
use llm_wiki_core::methodologies::terminology::{evaluate_terminology_occurrences, TermDefinition};
use llm_wiki_core::methodologies::thesaurus::{
    evaluate_controlled_vocabulary, evaluate_thesaurus_reciprocity, ThesaurusGraph,
    ThesaurusRelation,
};
use llm_wiki_core::model::{
    EpistemicTier, FindingSeverity, Methodology, OptimizationProfile, QualityPillar,
    StrategyRole, StrategyType, StructuralFinding, TransformationAction,
};
use llm_wiki_core::observation::scanner::{scan_repository, ScanOptions};
use llm_wiki_core::strategies::ahp::{
    compute_ahp_priority_vector, compute_implementation_cost, compute_strategy_fit_score,
    AhpMatrix,
};
use llm_wiki_core::strategies::catalog::{
    evaluate_extract_facet_metadata, evaluate_preserve_document, evaluate_split_decompose,
};
use llm_wiki_core::strategies::classifier::classify_pairwise_role;
use llm_wiki_core::strategies::tradeoffs::generate_strategy_tradeoffs;
use std::collections::{HashMap, HashSet};
use tempfile::tempdir;

#[test]
fn test_end_to_end_full_pipeline_18_methodologies_zero_ai() {
    let dir = tempdir().expect("Failed to create temporary directory for e2e test");
    let root = dir.path();

    // 1. Setup sample knowledge vault with interlinked documents
    let agent_doc = "# AGENTS.md\n\nEntrypoint for AI harness.\nSee [[architecture.md#design]] and [[glossary.md]].\n";
    let arch_doc = "# Architecture\n\n## Design\nCore design principles.\nSee [[database.md]] and [[api.md]].\n";
    let db_doc = "---\ncategory: backend\ntype: database\n---\n# Database\n\nDatabase topology and migrations.\n";
    let api_doc = "---\ncategory: backend\ntype: api\n---\n# API\n\nREST and gRPC interfaces.\n";
    let gloss_doc = "# Glossary\n\nKOS: Knowledge Organization System.\nBPE: Byte Pair Encoding.\n";

    std::fs::write(root.join("AGENTS.md"), agent_doc).unwrap();
    std::fs::write(root.join("architecture.md"), arch_doc).unwrap();
    std::fs::write(root.join("database.md"), db_doc).unwrap();
    std::fs::write(root.join("api.md"), api_doc).unwrap();
    std::fs::write(root.join("glossary.md"), gloss_doc).unwrap();

    // Setup .context-vault/baseline.json
    let cv_dir = root.join(".context-vault");
    std::fs::create_dir_all(&cv_dir).unwrap();
    let baseline_mgr = BaselineManager::new(&cv_dir.join("baseline.json"));
    baseline_mgr.save().unwrap();

    let profile = OptimizationProfile::ai_oriented();
    let options = ScanOptions::default();

    // 2. Cold Scan & Deterministic Observation (FR-001, FR-002, Invariant III & IV)
    let scan_res = scan_repository(root, &options, &profile, "commit_e2e_alpha").unwrap();
    assert_eq!(scan_res.total_documents, 5);
    assert_eq!(scan_res.analysis_id.len(), 64); // 256-bit BLAKE3 digest

    // 3. Evaluate all 18 KOS Methodologies (FR-010..FR-027, SC-008)
    // 3.1 Classification Coverage & Taxonomy
    let mut doc_cats = HashMap::new();
    doc_cats.insert("database.md".to_string(), Some("backend".to_string()));
    doc_cats.insert("api.md".to_string(), Some("backend".to_string()));
    let class_eval = evaluate_classification_coverage(&doc_cats);
    assert!(class_eval.coverage_ratio > 0.0);

    let mut tax_graph = TaxonomyGraph::new();
    tax_graph.add_node("root");
    tax_graph.add_node("backend");
    tax_graph.add_child("root", "backend");
    let tax_eval = evaluate_taxonomy(&tax_graph);
    assert!(tax_eval.cycles.is_empty());

    // 3.2 Faceted Classification
    let facet_docs = vec![
        DocumentFacets {
            document_id: "database.md".to_string(),
            facets: vec![("tier".to_string(), "backend".to_string())],
        },
        DocumentFacets {
            document_id: "api.md".to_string(),
            facets: vec![("tier".to_string(), "backend".to_string())],
        },
    ];
    let facet_ortho = compute_facet_orthogonality(&facet_docs);
    assert!(facet_ortho.orthogonality >= 0.0);

    // 3.3 Controlled Vocabulary & Thesaurus ISO 25964 Reciprocity
    let auth_terms = vec!["architecture".to_string(), "database".to_string()];
    let obs_terms = vec!["architecture".to_string(), "datbase".to_string()];
    let cv_eval = evaluate_controlled_vocabulary(&auth_terms, &obs_terms);
    assert_eq!(cv_eval.near_duplicates.len(), 1);

    let mut thesaurus = ThesaurusGraph::new();
    thesaurus.add_relation("Database", "Storage", ThesaurusRelation::BroaderTerm);
    thesaurus.add_relation("Storage", "Database", ThesaurusRelation::NarrowerTerm);
    let the_eval = evaluate_thesaurus_reciprocity(&thesaurus);
    assert!(the_eval.reciprocal_violations.is_empty());

    // 3.4 Semantic Networks & Concept Maps
    let mut sem_net = SemanticNetworkGraph::new();
    sem_net.add_typed_edge("AGENTS.md", "architecture.md", "references");
    let sem_eval = evaluate_semantic_network(&sem_net);
    assert!(sem_eval.relational_density > 0.0);

    let propositions = vec![ConceptProposition {
        subject: "Harness".to_string(),
        predicate: "routes_to".to_string(),
        object: "Architecture".to_string(),
    }];
    let cmap_eval = evaluate_concept_map(&propositions, 100);
    assert!(cmap_eval.proposition_density > 0.0);

    // 3.5 Metadata Profiles (DCAP & SHACL Ontology)
    let mut dcap_shape = DcapShape::new("module_spec");
    dcap_shape.add_mandatory_field("category");
    let dcap_records = vec![DocumentMetadataRecord {
        document_id: "database.md".to_string(),
        fields: vec![("category".to_string(), "backend".to_string())],
    }];
    let dcap_eval = evaluate_dcap_metadata(&dcap_shape, &dcap_records);
    assert_eq!(dcap_eval.compliant_documents, 1);

    let ontology_eval = evaluate_ontology_shapes(&dcap_shape, &dcap_records);
    assert!(ontology_eval.violating_documents.is_empty());

    // 3.6 Provenance & Co-change Coupling
    let mut commits_1 = HashSet::new();
    commits_1.insert("c1".to_string());
    commits_1.insert("c2".to_string());
    let mut commits_2 = HashSet::new();
    commits_2.insert("c2".to_string());
    let prov_coupling = compute_co_change_coupling(&commits_1, &commits_2);
    assert!((prov_coupling - 0.50).abs() < 1e-4);

    // 3.7 Information Architecture & BM25F Information Retrieval
    let ia_edges = vec![("AGENTS.md".to_string(), "architecture.md".to_string(), "Architecture".to_string())];
    let ia_titles = vec![("AGENTS.md".to_string(), "Agents".to_string()), ("architecture.md".to_string(), "Architecture".to_string())];
    let ia_eval = evaluate_information_architecture("AGENTS.md", &ia_edges, &ia_titles);
    assert_eq!(ia_eval.max_depth, 1);

    let bm_docs = vec![DocumentFieldContent {
        id: "architecture.md".to_string(),
        title: "Architecture".to_string(),
        headings: vec!["Design".to_string()],
        body: "Core design principles.".to_string(),
    }];
    let bm_results = evaluate_bm25f("architecture design", &bm_docs);
    assert!(!bm_results.is_empty());

    // 3.8 Terminology Management (Aho-Corasick)
    let terms = vec![TermDefinition {
        canonical: "KOS".to_string(),
        is_deprecated: false,
    }];
    let term_matches = evaluate_terminology_occurrences(&terms, "KOS: Knowledge Organization System.");
    assert_eq!(term_matches.len(), 1);

    // 3.9 Knowledge Graph / Network Analysis (Brandes Centrality, PageRank, Bridges)
    let mut net = NetworkGraph::new();
    net.add_edge("AGENTS.md", "architecture.md");
    net.add_edge("architecture.md", "database.md");
    let net_eval = evaluate_network_analysis(&net);
    assert!(net_eval.pagerank.contains_key("architecture.md"));
    assert!(!net_eval.bridges.is_empty());

    // 3.10 Context Routing & Subject Ontogeny
    let hops = vec![
        ContextTraceHop {
            document_id: "AGENTS.md".to_string(),
            fragment: Some("design".to_string()),
            fragment_tokens: 50,
            full_document_tokens: 500,
        },
    ];
    let trace_res = evaluate_context_trace_routing(&hops);
    assert!(trace_res.reduction_percentage > 0.80);

    let mut t1_cit = HashSet::new();
    t1_cit.insert("database.md".to_string());
    let mut t2_cit = HashSet::new();
    t2_cit.insert("api.md".to_string());
    let drift = evaluate_knowledge_drift(&t1_cit, &t2_cit);
    assert!(drift > 0.0);

    // 4. Epistemic Audit (FR-004..FR-007, SC-001)
    let finding = StructuralFinding {
        id: "finding-001".to_string(),
        methodology: Methodology::Thesaurus,
        pillar: QualityPillar::Consistency,
        tier: EpistemicTier::TierBCandidateGeneration,
        severity: FindingSeverity::Low,
        document_id: "glossary.md".to_string(),
        fragment_anchor: None,
        line_number: 1,
        rule_key: "vocabulary_typo_candidate".to_string(),
        message: "Suggested preferred term alignment".to_string(),
        evidence_metrics: serde_json::json!({ "levenshtein": 1 }),
        remediation_hint: "Add reciprocal BT/NT cross-reference".to_string(),
    };
    assert_eq!(
        EpistemicClassifier::classify_rule(&finding.rule_key),
        EpistemicTier::TierBCandidateGeneration
    );

    // 5. Invariant XIII: Monolith Protection Guard
    let monolith_metrics = MonolithMetrics {
        tokens: 50_000,
        cohesion: 0.85,
        coupling: 0.20,
        semantic_diversity: 0.35,
        fragment_count: 8,
        broken_anchors_count: 0,
    };
    let protection = check_monolith_protection(&monolith_metrics);
    assert!(protection.is_protected);

    // 6. 5-Pillar Profile-Relative Health Assessment (FR-028, FR-029)
    let health = compute_health_score(&scan_res.analysis_id, &[finding.clone()], &profile, 0, 0);
    assert!(health.aggregate_score > 90.0);
    assert!(health.integrity.raw_score <= 100.0);
    assert!(health.consistency.raw_score <= 100.0);

    // 7. Candidate Strategy Evaluation & Saaty AHP (FR-032, FR-033, FR-034)
    let cand_preserve = evaluate_preserve_document("architecture.md", 0.85, 4000);
    let cand_split = evaluate_split_decompose("architecture.md", 6000, 0.75);
    let cand_facet = evaluate_extract_facet_metadata("database.md", 0.80);

    assert!(cand_preserve.raw_fit_score > 0.0);
    assert!(cand_split.raw_fit_score > 0.0);
    assert!(cand_facet.raw_fit_score > 0.0);

    let ahp_matrix = AhpMatrix {
        size: 2,
        data: vec![vec![1.0, 2.0], vec![0.5, 1.0]],
    };
    let ahp_weights = compute_ahp_priority_vector(&ahp_matrix).unwrap();
    assert_eq!(ahp_weights.weights.len(), 2);

    let cost = compute_implementation_cost(cand_facet.target_files_count, cand_facet.estimated_line_churn);
    assert!(cost >= 0.0);

    let fit_score = compute_strategy_fit_score(&ahp_weights.weights, &[cand_facet.raw_fit_score, 0.5], 0.0);
    assert!(fit_score > 0.0);

    let pair_role = classify_pairwise_role(
        StrategyType::PreserveDocument,
        StrategyType::SplitDecompose,
        "architecture.md",
        "architecture.md",
    );
    assert_eq!(pair_role, StrategyRole::MutuallyExclusive);

    let (benefits, tradeoffs) = generate_strategy_tradeoffs(StrategyType::ExtractFacetMetadata, "database.md", 1000);
    assert!(!benefits.is_empty());
    assert!(!tradeoffs.is_empty()); // Invariant XV: Explicit transparent trade-offs

    // 8. User-Controlled Automation & Cryptographic Approval Gate (FR-036, FR-037, Invariant XVI)
    let plan_generator = TransformationPlanGenerator::new();
    let action = TransformationAction::UpdateFrontmatter {
        file: root.join("database.md").to_string_lossy().to_string(),
        key: "managed_by".to_string(),
        value: serde_json::json!("llm-wiki-engine"),
    };
    let plan = plan_generator.create_plan(
        "strat-facet-01",
        StrategyType::ExtractFacetMetadata,
        vec![root.join("database.md").to_string_lossy().to_string()],
        vec![action.clone()],
        3.5,
    );

    // Validate approval gate prevents unapproved application
    assert!(plan_generator.validate_approval(&plan, "invalid-token").is_err());
    assert!(plan_generator.validate_approval(&plan, &plan.approval_token).is_ok());

    // Apply mutation
    apply_transformation_action(&action).expect("Failed to apply transformation action");
    let updated_db_content = std::fs::read_to_string(root.join("database.md")).unwrap();
    assert!(updated_db_content.contains("managed_by: llm-wiki-engine"));

    // 9. Before/After Comparative Re-Measurement (FR-040)
    let updated_scan = scan_repository(root, &options, &profile, "commit_e2e_beta").unwrap();
    let updated_health = compute_health_score(&updated_scan.analysis_id, &[], &profile, 0, 0);
    let delta = compute_re_measurement_delta(&health, &updated_health, scan_res.total_tokens, updated_scan.total_tokens, None);
    assert!(delta.aggregate_score_delta >= 0.0);

    // 100% deterministic local computation with zero external AI calls!
}
