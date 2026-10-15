use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

use llm_wiki_core::evolution::*;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::ipc::handlers::{handle_request, AppState};
use llm_wiki_core::ipc::protocol::JsonRpcRequest;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};

#[test]
fn test_manager_summary_empty_vault() {
    let topology = GraphTopology::new();
    let temp = tempdir().unwrap();
    let summary = compute_manager_summary(
        &topology,
        temp.path(),
        DocumentationOrientation::AiOriented,
        &[],
    );

    assert_eq!(summary.structure.total_documents, 0);
    assert_eq!(summary.structure.connected_documents, 0);
    assert_eq!(summary.structure.isolated_documents, 0);
    assert_eq!(summary.structure.total_references, 0);
    assert_eq!(summary.health.quality_score, 100);
    assert_eq!(summary.health.findings_count, 0);
    assert_eq!(summary.evolution.opportunity_count, 0);
    assert_eq!(summary.evolution.orientation, DocumentationOrientation::AiOriented);
}

#[test]
fn test_manager_summary_isolated_notes() {
    let mut topology = GraphTopology::new();
    for i in 0..5 {
        let node = NoteNode::new(
            format!("note_{}.md", i),
            format!("/vault/note_{}.md", i),
            format!("Note {}", i),
        );
        topology.add_node(node);
    }

    let temp = tempdir().unwrap();
    let summary = compute_manager_summary(
        &topology,
        temp.path(),
        DocumentationOrientation::Balanced,
        &[],
    );

    assert_eq!(summary.structure.total_documents, 5);
    assert_eq!(summary.structure.connected_documents, 0);
    assert_eq!(summary.structure.isolated_documents, 5);
    assert_eq!(summary.structure.total_references, 0);
    assert_eq!(summary.health.discoverability, 0);
}

#[test]
fn test_manager_summary_connected_graph() {
    let mut topology = GraphTopology::new();
    for i in 0..3 {
        let node = NoteNode::new(
            format!("doc_{}.md", i),
            format!("/vault/doc_{}.md", i),
            format!("Doc {}", i),
        );
        topology.add_node(node);
    }

    // Add valid reference doc_0 -> doc_1
    let mut edge1 = NoteEdge::new(
        "doc_0.md".to_string(),
        "doc_1.md".to_string(),
        LinkType::Wikilink,
        "wikilink".to_string(),
        None,
        Some(10),
    );
    edge1.is_broken = false;
    topology.add_edge(edge1);

    // Add broken reference doc_1 -> doc_missing
    let mut edge2 = NoteEdge::new(
        "doc_1.md".to_string(),
        "doc_missing.md".to_string(),
        LinkType::Wikilink,
        "wikilink".to_string(),
        None,
        Some(20),
    );
    edge2.is_broken = true;
    topology.add_edge(edge2);

    // Add invalid anchor doc_0 -> doc_2#invalid_sec
    let mut edge3 = NoteEdge::new(
        "doc_0.md".to_string(),
        "doc_2.md".to_string(),
        LinkType::Wikilink,
        "wikilink".to_string(),
        Some("invalid_sec".to_string()),
        Some(30),
    );
    edge3.is_broken = true;
    edge3.target_fragment = Some("invalid_sec".to_string());
    topology.add_edge(edge3);

    let temp = tempdir().unwrap();
    let summary = compute_manager_summary(
        &topology,
        temp.path(),
        DocumentationOrientation::HumanOriented,
        &[],
    );

    assert_eq!(summary.structure.total_documents, 3);
    assert_eq!(summary.structure.total_references, 1);
    assert_eq!(summary.health.broken_references, 1);
    assert_eq!(summary.health.invalid_anchors, 1);
    assert_eq!(summary.health.findings_count, 2);
}

#[tokio::test]
async fn test_ipc_manager_get_summary() {
    let temp = tempdir().unwrap();
    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::from(1),
        method: "manager/getSummary".to_string(),
        params: None,
    };

    let res = handle_request(&state, req).await;
    assert!(res.error.is_none(), "RPC should not return error");
    let result = res.result.expect("Expected result");
    assert!(result.get("health").is_some());
    assert!(result.get("structure").is_some());
    assert!(result.get("evolution").is_some());
    assert!(result.get("sync").is_some());
}

#[tokio::test]
async fn test_ipc_evolution_set_orientation() {
    let temp = tempdir().unwrap();
    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::from(2),
        method: "evolution/setOrientation".to_string(),
        params: Some(serde_json::json!({ "orientation": "human_oriented" })),
    };

    let res = handle_request(&state, req).await;
    assert!(res.error.is_none());
    let result = res.result.expect("Expected result");
    assert_eq!(result.get("success").unwrap(), true);
    let profile = result.get("profile").unwrap();
    assert_eq!(profile.get("orientation").unwrap(), "human_oriented");
}

#[tokio::test]
async fn test_ipc_evolution_update_custom_profile() {
    let temp = tempdir().unwrap();
    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::from(3),
        method: "evolution/updateCustomProfile".to_string(),
        params: Some(serde_json::json!({
            "tier1_preferences": {
                "ai_discoverability": "high",
                "context_efficiency": "medium",
                "human_readability": "high",
                "knowledge_reuse": "low",
                "narrative_coherence": "high",
                "maintenance_simplicity": "medium"
            },
            "hard_constraints": ["never_split_documents"]
        })),
    };

    let res = handle_request(&state, req).await;
    assert!(res.error.is_none());
    let result = res.result.expect("Expected result");
    assert_eq!(result.get("success").unwrap(), true);
    let profile = result.get("profile").unwrap();
    assert_eq!(profile.get("orientation").unwrap(), "custom");
}

#[tokio::test]
async fn test_ipc_evolution_opportunities_and_preview_workflow() {
    let temp = tempdir().unwrap();
    let doc_file = temp.path().join("router.md");
    std::fs::write(&doc_file, "# Router\n\nContent\n").unwrap();

    let state = Arc::new(RwLock::new(AppState::new(temp.path().to_path_buf())));
    {
        let mut st = state.write().await;
        let mut node = NoteNode::new(
            "router.md".to_string(),
            doc_file.to_string_lossy().to_string(),
            "Router",
        );
        node.tokens = 200;
        node.out_degree = 25;
        node.betweenness = 0.50;
        st.topology.add_node(node);
    }

    // 1. Fetch opportunities
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::from(4),
        method: "evolution/getOpportunities".to_string(),
        params: Some(serde_json::json!({ "limit": 10 })),
    };
    let res = handle_request(&state, req).await;
    assert!(res.error.is_none());
    let result = res.result.expect("Expected result");
    let opps = result.get("opportunities").and_then(|v| v.as_array()).unwrap();
    assert!(!opps.is_empty(), "Should detect weak discovery / implicit router opportunity");
    let opp_id = opps[0].get("id").and_then(|v| v.as_str()).unwrap();

    // 2. Preview transformation
    let req2 = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::from(5),
        method: "evolution/previewTransformation".to_string(),
        params: Some(serde_json::json!({
            "opportunityId": opp_id,
            "strategy": "lightweight_index",
        })),
    };
    let res2 = handle_request(&state, req2).await;
    assert!(res2.error.is_none());
    let result2 = res2.result.expect("Expected preview result");
    let plan_id = result2.get("plan_id").and_then(|v| v.as_str()).unwrap();
    assert!(!plan_id.is_empty());

    // 3. Apply transformation with approval
    let req3 = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::Value::from(6),
        method: "evolution/applyTransformation".to_string(),
        params: Some(serde_json::json!({
            "plan_id": plan_id,
            "requireApproval": true,
        })),
    };
    let res3 = handle_request(&state, req3).await;
    assert!(res3.error.is_none());
    let result3 = res3.result.expect("Expected apply result");
    assert_eq!(result3.get("success").unwrap(), true);
    assert!(result3.get("reMeasurement").is_some());
}

