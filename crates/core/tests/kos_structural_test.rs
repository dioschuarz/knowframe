//! Unit tests for Classification, Taxonomy, and Faceted Classification KOS methodologies.
//! Validates FR-010, FR-011, FR-012, and SC-008.

use llm_wiki_core::methodologies::facets::{compute_facet_orthogonality, DocumentFacets};
use llm_wiki_core::methodologies::taxonomy::{
    evaluate_classification_coverage, evaluate_taxonomy, TaxonomyGraph,
};
use std::collections::HashMap;

#[test]
fn test_classification_coverage_and_unclassified_documents() {
    let mut doc_categories = HashMap::new();
    doc_categories.insert("doc1.md".to_string(), Some("architecture".to_string()));
    doc_categories.insert("doc2.md".to_string(), Some("database".to_string()));
    doc_categories.insert("doc3.md".to_string(), None); // unclassified
    doc_categories.insert("doc4.md".to_string(), Some("security".to_string()));

    let res = evaluate_classification_coverage(&doc_categories);
    assert_eq!(res.total_documents, 4);
    assert_eq!(res.classified_documents, 3);
    assert!((res.coverage_ratio - 0.75).abs() < 1e-4);
    assert_eq!(res.unclassified_documents, vec!["doc3.md"]);
}

#[test]
fn test_taxonomy_tarjan_cycle_detection_and_imbalance() {
    let mut tax = TaxonomyGraph::new();
    tax.add_node("root");
    tax.add_node("backend");
    tax.add_node("frontend");
    tax.add_node("database");

    tax.add_child("root", "backend");
    tax.add_child("backend", "database");
    tax.add_child("root", "frontend");

    let eval = evaluate_taxonomy(&tax);
    assert!(eval.cycles.is_empty(), "Valid DAG should have zero cycles");
    assert!(eval.imbalance_ratio >= 0.0);

    // Introduce cycle: database -> root
    tax.add_child("database", "root");
    let cyclic_eval = evaluate_taxonomy(&tax);
    assert!(!cyclic_eval.cycles.is_empty(), "Tarjan SCC must detect taxonomic cycle");
}

#[test]
fn test_faceted_classification_orthogonality() {
    let docs = vec![
        DocumentFacets {
            document_id: "doc1.md".to_string(),
            facets: vec![("domain".to_string(), "cloud".to_string()), ("tier".to_string(), "api".to_string())],
        },
        DocumentFacets {
            document_id: "doc2.md".to_string(),
            facets: vec![("domain".to_string(), "edge".to_string()), ("tier".to_string(), "db".to_string())],
        },
        DocumentFacets {
            document_id: "doc3.md".to_string(),
            facets: vec![("domain".to_string(), "cloud".to_string()), ("tier".to_string(), "db".to_string())],
        },
        DocumentFacets {
            document_id: "doc4.md".to_string(),
            facets: vec![("domain".to_string(), "edge".to_string()), ("tier".to_string(), "api".to_string())],
        },
    ];

    let result = compute_facet_orthogonality(&docs);
    assert!(
        result.orthogonality >= 0.80,
        "Independent orthogonal facets must have high orthogonality score (>= 0.80)"
    );
}
