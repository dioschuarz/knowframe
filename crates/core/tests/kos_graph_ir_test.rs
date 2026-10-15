//! Unit tests for Information Architecture, BM25F Information Retrieval, and Network Analysis.
//! Validates FR-020, FR-021, FR-023, and SC-008.

use llm_wiki_core::methodologies::information_retrieval::{
    evaluate_bm25f, evaluate_information_architecture, DocumentFieldContent,
};
use llm_wiki_core::methodologies::network_analysis::{
    evaluate_network_analysis, NetworkGraph,
};

#[test]
fn test_information_architecture_navigation_depth_and_scent() {
    let mut edges = Vec::new();
    edges.push(("index.md".to_string(), "guide.md".to_string(), "Getting Started Guide".to_string()));
    edges.push(("guide.md".to_string(), "setup.md".to_string(), "Installation Setup".to_string()));

    let node_titles = vec![
        ("index.md".to_string(), "Documentation Index".to_string()),
        ("guide.md".to_string(), "User Guide".to_string()),
        ("setup.md".to_string(), "Installation and Setup".to_string()),
    ];

    let ia_eval = evaluate_information_architecture("index.md", &edges, &node_titles);
    assert_eq!(ia_eval.max_depth, 2);
    assert!(ia_eval.mean_scent_score > 0.30);
    assert!(ia_eval.dead_ends.contains(&"setup.md".to_string()));
}

#[test]
fn test_bm25f_multi_field_retrieval() {
    let docs = vec![
        DocumentFieldContent {
            id: "doc1.md".to_string(),
            title: "Deterministic Knowledge Engine".to_string(),
            headings: vec!["Epistemic Tiers".to_string(), "Architecture".to_string()],
            body: "A high-performance local Rust engine for repository organization.".to_string(),
        },
        DocumentFieldContent {
            id: "doc2.md".to_string(),
            title: "Recipe for Chocolate Cake".to_string(),
            headings: vec!["Ingredients".to_string(), "Baking Instructions".to_string()],
            body: "Flour, cocoa, sugar and eggs baked at 350 degrees.".to_string(),
        },
    ];

    let results = evaluate_bm25f("deterministic engine rust", &docs);
    assert!(!results.is_empty());
    assert_eq!(results[0].document_id, "doc1.md");
    assert!(results[0].score > 0.0);
}

#[test]
fn test_network_analysis_centrality_pagerank_and_bridges() {
    let mut graph = NetworkGraph::new();
    // Node A connects to B; B connects to C; B is the articulation bridge
    graph.add_edge("A", "B");
    graph.add_edge("B", "C");

    let eval = evaluate_network_analysis(&graph);
    assert!(eval.betweenness.get("B").copied().unwrap_or(0.0) > eval.betweenness.get("A").copied().unwrap_or(0.0));
    assert!(eval.pagerank.contains_key("B"));
    assert!(eval.bridges.contains(&("A".to_string(), "B".to_string())) || eval.bridges.contains(&("B".to_string(), "A".to_string())));
}
