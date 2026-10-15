//! Unit tests for Controlled Vocabulary, Thesaurus, and Semantic Networks KOS methodologies.
//! Validates FR-013, FR-014, FR-015, and SC-008.

use llm_wiki_core::methodologies::semantic_networks::{
    evaluate_semantic_network, SemanticNetworkGraph,
};
use llm_wiki_core::methodologies::thesaurus::{
    evaluate_controlled_vocabulary, evaluate_thesaurus_reciprocity, ThesaurusGraph,
    ThesaurusRelation,
};

#[test]
fn test_controlled_vocabulary_near_duplicate_detection() {
    let authorized = vec!["architecture".to_string(), "performance".to_string(), "database".to_string()];
    let observed = vec![
        "architecture".to_string(),
        "performace".to_string(), // Levenshtein typo (distance 1)
        "datbase".to_string(),    // Levenshtein typo (distance 1)
        "unrelated_tag".to_string(),
    ];

    let result = evaluate_controlled_vocabulary(&authorized, &observed);
    assert_eq!(result.near_duplicates.len(), 2);
    assert!(result.near_duplicates.iter().any(|d| d.observed == "performace" && d.suggested == "performance"));
    assert!(result.near_duplicates.iter().any(|d| d.observed == "datbase" && d.suggested == "database"));
    assert_eq!(result.uncontrolled_count, 3);
}

#[test]
fn test_thesaurus_reciprocal_relationship_verification() {
    let mut thesaurus = ThesaurusGraph::new();
    // Valid reciprocal pair: concept A has broader concept B, and concept B has narrower concept A
    thesaurus.add_relation("ConceptA", "ConceptB", ThesaurusRelation::BroaderTerm);
    thesaurus.add_relation("ConceptB", "ConceptA", ThesaurusRelation::NarrowerTerm);

    // Broken reciprocal pair: concept C has broader concept D, but concept D lacks narrower concept C
    thesaurus.add_relation("ConceptC", "ConceptD", ThesaurusRelation::BroaderTerm);

    let report = evaluate_thesaurus_reciprocity(&thesaurus);
    assert_eq!(report.reciprocal_violations.len(), 1);
    assert_eq!(report.reciprocal_violations[0].source, "ConceptC");
    assert_eq!(report.reciprocal_violations[0].target, "ConceptD");
}

#[test]
fn test_semantic_network_shannon_diversity_and_density() {
    let mut network = SemanticNetworkGraph::new();
    network.add_typed_edge("A", "B", "depends_on");
    network.add_typed_edge("B", "C", "implements");
    network.add_typed_edge("C", "A", "extends");
    network.add_typed_edge("A", "C", "references");

    let eval = evaluate_semantic_network(&network);
    assert!(eval.shannon_diversity > 1.5, "Balanced relation types yield high Shannon diversity");
    assert!(eval.relational_density > 0.0);
}
