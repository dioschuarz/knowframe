use llm_wiki_core::harness::directory_metrics::DirectoryMetricsAnalyzer;
use llm_wiki_core::harness::{
    CONCENTRATION_ENTROPY_THRESHOLD,
    OrganizationalSignalType,
};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{NoteEdge, NoteNode, LinkType};
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

#[test]
fn test_directory_depth_and_entropy_metrics_t054() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Create a deep nesting hierarchy (> 5 levels deep)
    let deep_dir = root
        .join("l1")
        .join("l2")
        .join("l3")
        .join("l4")
        .join("l5")
        .join("l6");
    fs::create_dir_all(&deep_dir).unwrap();
    fs::write(deep_dir.join("deep_doc.md"), "# Deep Doc\nContent\n").unwrap();

    // 2. Create an overconcentrated folder (30 files in one folder, 1 file in another)
    let conc_dir = root.join("big_folder");
    fs::create_dir_all(&conc_dir).unwrap();
    for i in 0..30 {
        fs::write(
            conc_dir.join(format!("doc_{}.md", i)),
            format!("# Doc {}\nContent\n", i),
        )
        .unwrap();
    }

    let other_dir = root.join("tiny_folder");
    fs::create_dir_all(&other_dir).unwrap();
    fs::write(other_dir.join("single.md"), "# Single\nContent\n").unwrap();

    // 3. Setup topology with cross-folder coupling
    let mut topology = GraphTopology::new();
    let n1 = NoteNode::new("tiny_folder/single.md", "tiny_folder/single.md", "Single");
    topology.add_node(n1);

    for i in 0..5 {
        let id = format!("big_folder/doc_{}.md", i);
        let n = NoteNode::new(&id, id.clone(), &format!("Doc {}", i));
        topology.add_node(n);

        // Every link from tiny_folder points to big_folder (100% external)
        let edge = NoteEdge::new(
            "tiny_folder/single.md",
            &id,
            LinkType::Wikilink,
            String::new(),
            None,
            Some(1),
        );
        topology.add_edge(edge);
    }

    let metrics = DirectoryMetricsAnalyzer::analyze(root, Some(&topology));

    // Verify depth calculation
    assert!(
        metrics.max_directory_depth >= 6,
        "Max depth must be at least 6, got {}",
        metrics.max_directory_depth
    );
    assert!(metrics.average_directory_depth > 0.0);

    // Verify excessive depth signal
    let has_excessive_depth = metrics
        .organizational_signals
        .iter()
        .any(|s| s.signal_type == OrganizationalSignalType::ExcessiveDepth);
    assert!(has_excessive_depth, "Must flag excessive depth signal");

    // Verify overconcentration signal (entropy < 0.35)
    assert!(
        metrics.concentration_entropy < CONCENTRATION_ENTROPY_THRESHOLD,
        "Entropy must be below threshold, got {}",
        metrics.concentration_entropy
    );
    let has_overconcentration = metrics
        .organizational_signals
        .iter()
        .any(|s| s.signal_type == OrganizationalSignalType::DocumentOverconcentration);
    assert!(has_overconcentration, "Must flag overconcentration signal");

    // Verify cross-folder coupling signal
    assert!(
        metrics.cross_folder_coupling > 0.5,
        "Cross folder coupling should be high, got {}",
        metrics.cross_folder_coupling
    );
    let has_coupling_signal = metrics
        .organizational_signals
        .iter()
        .any(|s| s.signal_type == OrganizationalSignalType::HighNavigationalCoupling);
    assert!(has_coupling_signal, "Must flag high navigational coupling signal");
}

#[tokio::test]
async fn test_analyze_directory_structure_rpc_t055() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create simple structure
    let docs = root.join("docs");
    fs::create_dir_all(&docs).unwrap();
    fs::write(docs.join("guide.md"), "# Guide\nContent\n").unwrap();
    fs::write(root.join("AGENTS.md"), "# AGENTS.md\nContent\n").unwrap();

    let state = Arc::new(RwLock::new(llm_wiki_core::ipc::handlers::AppState::new(
        root.to_path_buf(),
    )));

    let req = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: "governance/analyzeDirectoryStructure".to_string(),
        params: Some(serde_json::json!({})),
    };

    let res = llm_wiki_core::ipc::handlers::handle_request(&state, req).await;
    assert!(res.error.is_none(), "RPC error: {:?}", res.error);

    let val = res.result.expect("Result payload");
    assert!(val["max_directory_depth"].as_u64().is_some());
    assert!(val["average_directory_depth"].as_f64().is_some());
    assert!(val["concentration_entropy"].as_f64().is_some());
    assert!(val["domain_mixing_score"].as_f64().is_some());
    assert!(val["cross_folder_coupling"].as_f64().is_some());
    assert!(val["organizational_signals"].as_array().is_some());
}
