use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::RwLock;

use llm_wiki_core::collision::detect_collisions_in_graph;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::graph::centrality::compute_brandes_betweenness;
use llm_wiki_core::ipc::handlers::{handle_request, AppState};
use llm_wiki_core::ipc::protocol::JsonRpcRequest;
use llm_wiki_core::model::NoteNode;

#[test]
fn test_compact_graph_collision_detection_preserves_duplicates_and_excludes_unrelated() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    let auth_a = "# OAuth Token Service\n\nHandles JWT refresh tokens, session revocation, and OAuth2 bearer validation.\n\n## Token Rotation\nTokens rotate automatically on every refresh.\n";
    let auth_b = "# OAuth Token Service\n\nHandles JWT refresh tokens, session revocation, and OAuth2 bearer validation.\n\n## Token Rotation\nTokens rotate automatically on every refresh.\n";
    let sql_doc = "# PostgreSQL Indexing Strategy\n\nCovers B-tree composite indexes, GIN full-text search, and VACUUM tuning.\n";

    fs::write(root.join("auth_a.md"), auth_a).unwrap();
    fs::write(root.join("auth_b.md"), auth_b).unwrap();
    fs::write(root.join("sql.md"), sql_doc).unwrap();

    let mut topology = GraphTopology::new();
    topology.add_node(NoteNode::new("auth_a.md", "auth_a.md", "OAuth Token Service"));
    topology.add_node(NoteNode::new("auth_b.md", "auth_b.md", "OAuth Token Service"));
    topology.add_node(NoteNode::new("sql.md", "sql.md", "PostgreSQL Indexing Strategy"));

    let collisions = detect_collisions_in_graph(&topology, root, 0.70);
    assert_eq!(collisions.len(), 1, "Expected exactly 1 duplicate pair");
    assert_eq!(collisions[0].source_id, "auth_a.md");
    assert_eq!(collisions[0].target_id, "auth_b.md");
    assert!(collisions[0].composite_similarity >= 0.85);
    assert!(!collisions[0].shared_key_terms.is_empty());
}

#[test]
fn test_brandes_betweenness_enterprise_k_pivot_scale_and_hub_ranking() {
    // Construct a 2,000-node hub-and-spoke graph (> 1,500 exact threshold -> k-pivot Brandes)
    let n = 2_000usize;
    let node_ids: Vec<String> = (0..n).map(|i| format!("doc_{}.md", i)).collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    // Hub doc_0 bridges group A (1..1000) -> doc_0 -> group B (1000..2000)
    let mut hub_out = Vec::with_capacity(1000);
    for i in 1..1000 {
        adj.insert(format!("doc_{}.md", i), vec!["doc_0.md".to_string()]);
    }
    for i in 1000..n {
        hub_out.push(format!("doc_{}.md", i));
    }
    adj.insert("doc_0.md".to_string(), hub_out);

    let start = std::time::Instant::now();
    let scores = compute_brandes_betweenness(&node_ids, &adj);
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_secs_f64() < 2.0,
        "2,000-node k-pivot Brandes must finish well under 2s (took {:?})",
        elapsed
    );
    let hub_score = *scores.get("doc_0.md").unwrap();
    let leaf_score = *scores.get("doc_1.md").unwrap();
    assert!(
        hub_score > leaf_score && hub_score > 0.0,
        "Central bridge hub must have highest betweenness (hub={}, leaf={})",
        hub_score,
        leaf_score
    );
}

#[tokio::test]
async fn test_ipc_scan_strips_fragments_and_get_issues_uses_cache_with_default_cap() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Create 600 tiny orphan/non-clickable docs to verify default limit=500 cap & exact totals
    fs::write(root.join("AGENTS.md"), "# Hub\n\n[[doc_0|Zero]]\n").unwrap();
    for i in 0..600 {
        fs::write(
            root.join(format!("doc_{}.md", i)),
            format!("# Document {}\n\n## Details\nBody text for doc {}.\n", i, i),
        )
        .unwrap();
    }

    let state = Arc::new(RwLock::new(AppState::new(root.to_path_buf())));

    // 1. Scan with include_fragments = false must strip wire fragments while daemon retains them
    let scan_resp = handle_request(
        &state,
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            method: "workspace/scan".to_string(),
            params: Some(serde_json::json!({ "include_fragments": false })),
        },
    )
    .await;
    let scan_val = scan_resp.result.expect("scan must succeed");
    let wire_nodes = scan_val["nodes"].as_array().unwrap();
    assert_eq!(wire_nodes.len(), 601);
    assert!(
        wire_nodes.iter().all(|n| n["fragments"].as_array().unwrap().is_empty()),
        "Wire nodes must have fragments stripped when include_fragments=false"
    );
    {
        let st = state.read().await;
        assert!(
            !st.topology.nodes.get("doc_0.md").unwrap().fragments.is_empty(),
            "Daemon topology must retain full fragments for on-demand inspection"
        );
    }

    // 2. workspace/getQualityScore populates cached_analysis once
    let q_resp = handle_request(
        &state,
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(2),
            method: "workspace/getQualityScore".to_string(),
            params: Some(serde_json::json!({})),
        },
    )
    .await;
    assert!(q_resp.result.is_some());
    {
        let st = state.read().await;
        assert!(st.cached_analysis.is_some(), "Analysis must be cached after getQualityScore");
    }

    // 3. workspace/getIssues hits cache and caps default response at 500 while preserving exact totals
    let issues_resp = handle_request(
        &state,
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(3),
            method: "workspace/getIssues".to_string(),
            params: Some(serde_json::json!({})),
        },
    )
    .await;
    let issues_val = issues_resp.result.expect("getIssues must succeed");
    let total_issues = issues_val["total_issues"].as_u64().unwrap();
    let returned = issues_val["issues"].as_array().unwrap().len();
    assert!(total_issues > 500, "Expected > 500 total issues, got {}", total_issues);
    assert_eq!(returned, 500, "Default RPC issues payload must be capped at 500");
    assert_eq!(issues_val["truncated"].as_bool().unwrap(), true);
}
