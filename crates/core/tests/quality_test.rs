use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

use llm_wiki_core::cli::quality_cmd::execute_quality;
use llm_wiki_core::collision::{CollisionClassification, SemanticCollision};
use llm_wiki_core::debt::baseline::capture_initial_baseline;
use llm_wiki_core::drift::{DriftConfidence, DriftType, TechnicalDriftFinding};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::ipc::handlers::{handle_request, AppState};
use llm_wiki_core::ipc::protocol::JsonRpcRequest;
use llm_wiki_core::model::{LinkResolutionStatus, LinkType, NodeStatus, NoteEdge, NoteNode};
use llm_wiki_core::quality::scorer::{calculate_quality_score, calculate_topological_weight};

fn make_test_node(id: &str, betweenness: f64) -> NoteNode {
    let mut node = NoteNode::new(id, format!("{}.md", id), id);
    node.betweenness = betweenness;
    node.status = NodeStatus::Active;
    node.in_degree = 1;
    node.out_degree = 1;
    node.tokens = 100;
    node.hop_distance = 1;
    node
}

fn make_test_edge(source: &str, target: &str, is_broken: bool) -> NoteEdge {
    let mut edge = NoteEdge::new(
        source,
        target,
        LinkType::Wikilink,
        format!("[[{}]]", target),
        None,
        Some(1),
    );
    edge.is_broken = is_broken;
    edge.resolution_status = if is_broken {
        Some(LinkResolutionStatus::BrokenWikilink)
    } else {
        Some(LinkResolutionStatus::Resolved)
    };
    edge
}

#[test]
fn test_quality_score_pillar_weights() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_test_node("A.md", 0.0));
    topology.add_node(make_test_node("B.md", 0.0));
    topology.add_edge(make_test_edge("A.md", "B.md", false));

    // Perfect graph: all 5 pillars 100.0, overall 100.0
    let perfect = calculate_quality_score(&topology, None, &[], &[]);
    assert_eq!(perfect.overall_score, 100.0);
    assert_eq!(perfect.integrity_score, 100.0);
    assert_eq!(perfect.structure_score, 100.0);
    assert_eq!(perfect.consistency_score, 100.0);
    assert_eq!(perfect.discoverability_score, 100.0);
    assert_eq!(perfect.freshness_score, 100.0);

    // Add broken link on A.md: integrity drops
    topology.add_edge(make_test_edge("A.md", "missing.md", true));
    let with_broken = calculate_quality_score(&topology, None, &[], &[]);
    // Integrity deduction = 5.0 * 1.0 = 5.0 -> integrity_score = 95.0
    // 5-pillar overall = 0.25 * 95.0 + 0.20 * 100 + 0.20 * 100 + 0.20 * 100 + 0.15 * 100 = 23.75 + 75.0 = 98.75 -> 98.8
    assert_eq!(with_broken.integrity_score, 95.0);
    assert_eq!(with_broken.overall_score, 98.8);
}

#[test]
fn test_topological_betweenness_defect_weighting() {
    // SC-007 Verification:
    // Hub node (B >= 0.5) vs Leaf node (B <= 0.05)
    let hub_betweenness = 0.85; // High hub
    let leaf_betweenness = 0.00; // Peripheral leaf

    let hub_weight = calculate_topological_weight(hub_betweenness);
    let leaf_weight = calculate_topological_weight(leaf_betweenness);

    // Multiplier ratio: (1.0 + 3.0 * 0.85) / (1.0 + 3.0 * 0.0) = 3.55 / 1.0 = 3.55 >= 2.5
    assert!(hub_weight / leaf_weight >= 2.5);

    // Verify through calculate_quality_score
    let mut top_leaf = GraphTopology::new();
    top_leaf.add_node(make_test_node("leaf.md", leaf_betweenness));
    top_leaf.add_edge(make_test_edge("leaf.md", "missing.md", true));
    let score_leaf = calculate_quality_score(&top_leaf, None, &[], &[]);

    let mut top_hub = GraphTopology::new();
    top_hub.add_node(make_test_node("hub.md", hub_betweenness));
    top_hub.add_edge(make_test_edge("hub.md", "missing.md", true));
    let score_hub = calculate_quality_score(&top_hub, None, &[], &[]);

    let leaf_drop = 100.0 - score_leaf.overall_score;
    let hub_drop = 100.0 - score_hub.overall_score;

    assert!(
        hub_drop / leaf_drop >= 2.5,
        "Hub drop ({:.2}) must be >= 2.5x leaf drop ({:.2}), got {:.2}x",
        hub_drop,
        leaf_drop,
        hub_drop / leaf_drop
    );
}

#[test]
fn test_quality_score_debt_breakdown() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_test_node("base.md", 0.0));
    topology.add_edge(make_test_edge("base.md", "missing1.md", true));

    // Initial baseline with 1 broken link
    let baseline = capture_initial_baseline(&topology, "commit_1", vec![], vec![]);
    assert_eq!(baseline.total_debt, 1);

    // Current state has the 1 legacy broken link + 1 new broken link
    topology.add_edge(make_test_edge("base.md", "missing2.md", true));

    let score = calculate_quality_score(&topology, Some(&baseline), &[], &[]);
    assert_eq!(score.legacy_debt_count, 1);
    assert_eq!(score.new_regressions_count, 1);
    assert_eq!(score.resolved_debt_count, 0);
}

#[test]
fn test_quality_top_degradation_factors() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_test_node("hub.md", 0.9));
    topology.add_node(make_test_node("mid.md", 0.3));
    topology.add_node(make_test_node("leaf.md", 0.0));

    topology.add_edge(make_test_edge("hub.md", "dead_hub_link.md", true));
    topology.add_edge(make_test_edge("leaf.md", "dead_leaf_link.md", true));

    let collision = SemanticCollision {
        source_id: "hub.md".to_string(),
        target_id: "mid.md".to_string(),
        composite_similarity: 0.92,
        title_similarity: 0.95,
        heading_similarity: 0.90,
        bm25_similarity: 0.88,
        ngram_similarity: 0.85,
        tag_similarity: 1.0,
        classification: CollisionClassification::PotentialDuplicate,
        shared_key_terms: vec!["architecture".to_string()],
        canonical_candidate: Some("hub.md".to_string()),
        canonical_confidence: 0.9,
        canonical_evidence: vec![],
    };

    let drift = TechnicalDriftFinding {
        doc_id: "hub.md".to_string(),
        code_target: "src/auth.rs".to_string(),
        symbol_name: Some("authenticate".to_string()),
        citation_line: 42,
        drift_type: DriftType::DeletedCodeFile,
        confidence: DriftConfidence::High,
        explanation: "File deleted".to_string(),
    };

    let score = calculate_quality_score(&topology, None, &[collision], &[drift]);

    // Top factors should have highest deduction points first
    assert!(!score.top_degradation_factors.is_empty());
    assert!(score.top_degradation_factors.len() <= 5);

    for i in 1..score.top_degradation_factors.len() {
        assert!(
            score.top_degradation_factors[i - 1].deduction_points
                >= score.top_degradation_factors[i].deduction_points
        );
    }
}

#[test]
fn test_cli_quality_command() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("AGENTS.md"),
        "# AGENTS\n\nEntrypoint referencing [[doc1]].\n",
    )
    .unwrap();
    fs::write(
        root.join("doc1.md"),
        "# Doc 1\n\nReferencing [[doc2]].\n",
    )
    .unwrap();
    fs::write(
        root.join("doc2.md"),
        "# Doc 2\n\nTerminal documentation note.\n",
    )
    .unwrap();

    // Text execution
    let res_text = execute_quality(root, false);
    assert!(res_text.is_ok());

    // JSON execution
    let res_json = execute_quality(root, true);
    assert!(res_json.is_ok());
}

#[tokio::test]
async fn test_ipc_get_quality_score_handler() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("AGENTS.md"),
        "# AGENTS\n\nEntrypoint referencing [[doc1]].\n",
    )
    .unwrap();
    fs::write(
        root.join("doc1.md"),
        "# Doc 1\n\nReferencing [[doc2]].\n",
    )
    .unwrap();

    let state = Arc::new(RwLock::new(AppState::new(root.to_path_buf())));
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: "workspace/getQualityScore".to_string(),
        params: None,
    };

    let resp = handle_request(&state, req).await;
    assert!(resp.error.is_none());
    assert!(resp.result.is_some());

    let result = resp.result.unwrap();
    assert!(result.get("overall_score").is_some());
    assert!(result.get("integrity_score").is_some());
    assert!(result.get("consistency_score").is_some());
    assert!(result.get("discoverability_score").is_some());
    assert!(result.get("freshness_score").is_some());
    assert!(result.get("top_degradation_factors").is_some());
}
