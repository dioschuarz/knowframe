use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::harness::matrix::AIOrientedCapabilityMatrixEvaluator;
use llm_wiki_core::harness::{
    EmbeddedIndexMetadata, HarnessEntryPoint, HarnessQualityRatings, HarnessType,
    KnowledgeIndexRoute, RouteResolutionStatus, SkillAsset, SkillAssetType, SkillDefinition,
    SkillValidationStatus,
};
use llm_wiki_core::model::{LinkResolutionStatus, LinkType, NodeStatus, NoteEdge, NoteNode};
use llm_wiki_core::trace::traversal::{trace_from_harness, HarnessTraceResult};
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

fn make_node(id: &str, label: &str, tags: Vec<String>, tokens: u32) -> NoteNode {
    let mut node = NoteNode::new(id, format!("{}.md", id), label);
    node.tags = tags;
    node.tokens = tokens;
    node.status = NodeStatus::Active;
    node
}

fn make_edge(source: &str, target: &str) -> NoteEdge {
    let mut edge = NoteEdge::new(
        source,
        target,
        LinkType::Wikilink,
        format!("[[{}]]", target),
        None,
        Some(1),
    );
    edge.is_broken = false;
    edge.resolution_status = Some(LinkResolutionStatus::Resolved);
    edge
}

#[test]
fn test_harness_anchored_context_trace_traversal_t036() {
    let mut topology = GraphTopology::new();

    // Create graph: AGENTS.md -> docs/architecture.md -> docs/database.md
    topology.add_node(make_node("AGENTS.md", "AI Agent Harness", vec!["#harness".to_string()], 1500));
    topology.add_node(make_node("docs/architecture.md", "System Architecture", vec!["#arch".to_string()], 2000));
    topology.add_node(make_node("docs/database.md", "Database Schema & Migration", vec!["#db".to_string()], 1200));

    topology.add_edge(make_edge("AGENTS.md", "docs/architecture.md"));
    topology.add_edge(make_edge("docs/architecture.md", "docs/database.md"));

    // Traversal querying "Database Migration" from AGENTS.md
    let result = trace_from_harness(&topology, "Database Migration", "AGENTS.md", 3);

    assert_eq!(result.harness_file, "AGENTS.md");
    assert_eq!(result.target_fragment, "docs/database.md");
    assert_eq!(result.total_hops, 2);
    assert_eq!(result.path, vec!["AGENTS.md", "docs/architecture.md", "docs/database.md"]);
    assert_eq!(result.total_tokens_traversed, 1500 + 2000 + 1200);
    assert_eq!(result.relevance_progression.len(), 3);
    assert!(result.relevance_progression.last().unwrap() > &0.0);
}

#[test]
fn test_4_capability_matrix_computation_t037() {
    let harness = HarnessEntryPoint {
        file_path: "AGENTS.md".to_string(),
        harness_type: HarnessType::AgentsMd,
        is_primary: true,
        total_tokens: 3000,
        heading_count: 10,
        embedded_index: Some(EmbeddedIndexMetadata {
            heading_anchor: "#knowledge-index".to_string(),
            start_line: 20,
            end_line: 50,
            routes: vec![KnowledgeIndexRoute {
                domain: "Core".to_string(),
                label: "Architecture".to_string(),
                target_file: "docs/arch.md".to_string(),
                target_anchor: None,
                resolution_status: RouteResolutionStatus::Resolved,
                hop_distance_from_entry: 1,
                target_tokens: 500,
                line_number: 25,
            }],
            domain_coverage_ratio: 0.90,
        }),
        secondary_layers_detected: Vec::new(),
        quality_ratings: HarnessQualityRatings {
            integrity_score: 95.0,
            structure_score: 90.0,
            consistency_score: 92.0,
            discoverability_score: 94.0,
            freshness_score: 100.0,
            context_efficiency: 85.0,
        },
        last_modified: None,
    };

    let skills = vec![SkillDefinition {
        skill_id: "deploy-service".to_string(),
        dir_path: ".agents/skills/deploy-service".to_string(),
        manifest_path: ".agents/skills/deploy-service/SKILL.md".to_string(),
        name: "deploy-service".to_string(),
        description: "Deploy service".to_string(),
        assets: vec![SkillAsset {
            asset_type: SkillAssetType::Script,
            file_path: "scripts/deploy.sh".to_string(),
            exists_on_disk: true,
            byte_size: 200,
        }],
        status: SkillValidationStatus::Valid,
        unreferenced: false,
        overlap_ids: Vec::new(),
    }];

    let traces = vec![HarnessTraceResult {
        harness_file: "AGENTS.md".to_string(),
        total_hops: 1,
        total_tokens_traversed: 2000,
        target_fragment: "docs/arch.md".to_string(),
        path: vec!["AGENTS.md".to_string(), "docs/arch.md".to_string()],
        relevance_progression: vec![0.2, 0.9],
    }];

    let matrix = AIOrientedCapabilityMatrixEvaluator::evaluate(
        Some(&harness),
        &skills,
        &[],
        &traces,
    );

    assert!(matrix.entry_score >= 80.0, "Entry score must reflect valid harness");
    assert!(matrix.discovery_score >= 80.0, "Discovery score must reflect 90% domain coverage");
    assert!(matrix.retrieval_score >= 80.0, "Retrieval score must reflect 1-hop trace");
    assert!(matrix.reuse_score >= 80.0, "Reuse score must reflect 100% valid skills");
    assert!(matrix.overall_governance_score >= 80.0);
}

#[tokio::test]
async fn test_trace_from_harness_rpc_t038() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let agents = root.join("AGENTS.md");
    std::fs::write(&agents, "# AGENTS.md\n## Knowledge Index\n- [Arch](docs/arch.md)\n").unwrap();

    let docs = root.join("docs");
    std::fs::create_dir_all(&docs).unwrap();
    std::fs::write(docs.join("arch.md"), "# Architecture\nSystem design.\n").unwrap();

    let state = Arc::new(RwLock::new(llm_wiki_core::ipc::handlers::AppState::new(root.to_path_buf())));

    let req = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(42),
        method: "governance/traceFromHarness".to_string(),
        params: Some(serde_json::json!({
            "query": "Architecture",
            "harness_file": "AGENTS.md",
            "max_hops": 3
        })),
    };

    let res = llm_wiki_core::ipc::handlers::handle_request(&state, req).await;
    assert!(res.error.is_none(), "RPC error: {:?}", res.error);
    let result = res.result.expect("RPC result");

    assert_eq!(result["harness_file"], "AGENTS.md");
    assert!(result["path"].is_array());
    assert!(result["total_tokens_traversed"].as_u64().is_some());
}
