//! Unit tests for Terminology, Provenance, Knowledge Drift, and Context Trace routing.
//! Validates FR-018, FR-022, FR-025, FR-026, FR-027, SC-004, and SC-008.

use llm_wiki_core::methodologies::context_routing::{
    evaluate_context_trace_routing, evaluate_knowledge_drift, ContextTraceHop,
};
use llm_wiki_core::methodologies::provenance::compute_co_change_coupling;
use llm_wiki_core::methodologies::terminology::{evaluate_terminology_occurrences, TermDefinition};
use std::collections::HashSet;

#[test]
fn test_terminology_aho_corasick_matching() {
    let terms = vec![
        TermDefinition {
            canonical: "Deterministic Knowledge Engine".to_string(),
            is_deprecated: false,
        },
        TermDefinition {
            canonical: "OldWikiEngine".to_string(),
            is_deprecated: true,
        },
    ];

    let document_text = "We replaced OldWikiEngine with the new Deterministic Knowledge Engine.";
    let matches = evaluate_terminology_occurrences(&terms, document_text);
    assert_eq!(matches.len(), 2);
    assert!(matches.iter().any(|m| m.term == "OldWikiEngine" && m.is_deprecated));
    assert!(matches.iter().any(|m| m.term == "Deterministic Knowledge Engine" && !m.is_deprecated));
}

#[test]
fn test_provenance_commit_co_change_coupling() {
    let mut commits_a = HashSet::new();
    commits_a.insert("c1".to_string());
    commits_a.insert("c2".to_string());
    commits_a.insert("c3".to_string());

    let mut commits_b = HashSet::new();
    commits_b.insert("c2".to_string());
    commits_b.insert("c3".to_string());
    commits_b.insert("c4".to_string());

    let coupling = compute_co_change_coupling(&commits_a, &commits_b);
    // Intersection = {c2, c3} (2), Union = {c1, c2, c3, c4} (4) -> 2/4 = 0.50
    assert!((coupling - 0.50).abs() < 1e-4);
}

#[test]
fn test_knowledge_drift_temporal_citation_turnover() {
    let mut t1_citations = HashSet::new();
    t1_citations.insert("spec.md".to_string());
    t1_citations.insert("rfc_old.md".to_string());

    let mut t2_citations = HashSet::new();
    t2_citations.insert("spec.md".to_string());
    t2_citations.insert("rfc_new.md".to_string());

    let drift = evaluate_knowledge_drift(&t1_citations, &t2_citations);
    // Intersection = {spec.md} (1), Union = {spec.md, rfc_old.md, rfc_new.md} (3)
    // Drift = 1 - 1/3 = 2/3 = 0.6667
    assert!((drift - (2.0 / 3.0)).abs() < 1e-3);
}

#[test]
fn test_context_trace_section_level_reduction() {
    let hops = vec![
        ContextTraceHop {
            document_id: "AGENTS.md".to_string(),
            fragment: Some("Section 3".to_string()),
            fragment_tokens: 300,
            full_document_tokens: 3000,
        },
        ContextTraceHop {
            document_id: "spec.md".to_string(),
            fragment: Some("FR-027".to_string()),
            fragment_tokens: 200,
            full_document_tokens: 4000,
        },
    ];

    let trace_result = evaluate_context_trace_routing(&hops);
    assert_eq!(trace_result.total_trace_tokens, 500);
    assert_eq!(trace_result.total_full_tokens, 7000);
    // Reduction = 1 - (500 / 7000) = ~92.8%
    assert!(trace_result.reduction_percentage > 0.90);
}
