use llm_wiki_core::harness::discovery::{generate_template_agents_md, HarnessDetector};
use llm_wiki_core::harness::index_parser::IndexParser;
use llm_wiki_core::harness::secondary_layer::SecondaryLayerDetector;
use llm_wiki_core::harness::{HarnessType, RouteResolutionStatus};
use tempfile::tempdir;

#[test]
fn test_harness_detection_and_multi_harness_t007() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Initially empty workspace
    let empty_harnesses = HarnessDetector::detect_all(root, &[]);
    assert!(empty_harnesses.is_empty(), "Should detect 0 harnesses in empty dir");

    // 2. Generate template AGENTS.md (FR-002)
    let template = generate_template_agents_md(Some("TestProject"));
    assert!(template.contains("# AGENTS.md — AI Harness Entry Point"));
    assert!(template.contains("## Knowledge Index"));
    assert!(template.contains("### Architecture"));

    // Write AGENTS.md, CLAUDE.md, and custom harness
    std::fs::write(root.join("AGENTS.md"), &template).unwrap();
    std::fs::write(root.join("CLAUDE.md"), "# Claude Instructions\nSome instructions.").unwrap();
    std::fs::write(root.join("my-harness.md"), "# Custom Harness\nCustom routes.").unwrap();

    let custom_paths = vec!["my-harness.md".to_string()];
    let harnesses = HarnessDetector::detect_all(root, &custom_paths);

    assert_eq!(harnesses.len(), 3, "Expected 3 harnesses detected");

    // Check types and primary assignment
    let agents_entry = harnesses.iter().find(|h| h.harness_type == HarnessType::AgentsMd).unwrap();
    assert!(agents_entry.is_primary, "First standard harness should be primary");
    assert_eq!(agents_entry.file_path, "AGENTS.md");

    let claude_entry = harnesses.iter().find(|h| h.harness_type == HarnessType::ClaudeMd).unwrap();
    assert!(!claude_entry.is_primary);

    let custom_entry = harnesses.iter().find(|h| h.file_path == "my-harness.md").unwrap();
    assert_eq!(custom_entry.harness_type, HarnessType::Custom);
}

#[test]
fn test_embedded_knowledge_index_ast_parsing_and_resolution_t008() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Setup docs
    let docs_dir = root.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();
    std::fs::write(
        docs_dir.join("arch.md"),
        "# Architecture\n\n## Subsystems\nDetails.\n\n## Data Flow\nData details.",
    )
    .unwrap();
    std::fs::write(docs_dir.join("ops.md"), "# Operations\nRunbooks.").unwrap();

    // Create AGENTS.md with embedded Knowledge Index
    let agents_content = r#"# AGENTS.md

Welcome to the project.

## Knowledge Index

### Architecture
- [System Architecture](docs/arch.md): High-level system structure.
- [Subsystems](docs/arch.md#subsystems): Subsystem breakdown.
- [Non Existent Section](docs/arch.md#non-existent-anchor): Dead fragment anchor.

### Operations
- [Runbooks](docs/ops.md): Standard operating procedures.
- [Missing Doc](docs/missing.md): File does not exist.
"#;

    std::fs::write(root.join("AGENTS.md"), agents_content).unwrap();

    let parser = IndexParser::new(root);
    let index_meta = parser.parse_harness_index("AGENTS.md", agents_content).expect("Failed to parse index");

    assert_eq!(index_meta.heading_anchor, "knowledge-index");
    assert_eq!(index_meta.routes.len(), 5, "Expected 5 extracted routes");

    // Route 1: docs/arch.md -> Resolved
    let r1 = &index_meta.routes[0];
    assert_eq!(r1.domain, "Architecture");
    assert_eq!(r1.label, "System Architecture");
    assert_eq!(r1.target_file, "docs/arch.md");
    assert_eq!(r1.target_anchor, None);
    assert_eq!(r1.resolution_status, RouteResolutionStatus::Resolved);
    assert_eq!(r1.hop_distance_from_entry, 1);

    // Route 2: docs/arch.md#subsystems -> Resolved with anchor
    let r2 = &index_meta.routes[1];
    assert_eq!(r2.target_anchor.as_deref(), Some("subsystems"));
    assert_eq!(r2.resolution_status, RouteResolutionStatus::Resolved);

    // Route 3: docs/arch.md#non-existent-anchor -> BrokenAnchor
    let r3 = &index_meta.routes[2];
    assert_eq!(r3.target_anchor.as_deref(), Some("non-existent-anchor"));
    assert_eq!(r3.resolution_status, RouteResolutionStatus::BrokenAnchor);

    // Route 4: docs/ops.md -> Resolved
    let r4 = &index_meta.routes[3];
    assert_eq!(r4.domain, "Operations");
    assert_eq!(r4.resolution_status, RouteResolutionStatus::Resolved);

    // Route 5: docs/missing.md -> BrokenFile
    let r5 = &index_meta.routes[4];
    assert_eq!(r5.resolution_status, RouteResolutionStatus::BrokenFile);
}

#[test]
fn test_secondary_layer_detection_t013() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create intermediate routing hops
    let docs_dir = root.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();

    std::fs::write(
        root.join("AGENTS.md"),
        "# AGENTS.md\n## Knowledge Index\n- [Docs Index](docs/index.md)\n- [Memory](memory.md)",
    )
    .unwrap();

    std::fs::write(
        docs_dir.join("index.md"),
        "# Docs Index\n- [Architecture](arch.md)\n- [Security](sec.md)",
    )
    .unwrap();

    std::fs::write(
        root.join("memory.md"),
        "# Memory Index\n- [Notes](notes.md)",
    )
    .unwrap();

    let detector = SecondaryLayerDetector::new(root);
    let layers = detector.detect_secondary_layers("AGENTS.md");

    assert_eq!(layers.len(), 2, "Expected 2 secondary routing layers flagged");
    assert!(layers.iter().any(|l| l.file_path == "docs/index.md"));
    assert!(layers.iter().any(|l| l.file_path == "memory.md"));
    for layer in &layers {
        assert!(layer.penalty_points > 0.0);
        assert!(!layer.recommendation_hint.is_empty());
    }
}

#[tokio::test]
async fn test_get_harness_status_and_set_active_rpc_t009_t019() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    std::fs::write(
        root.join("AGENTS.md"),
        "# AGENTS.md\n## Knowledge Index\n### Architecture\n- [Arch](arch.md)",
    )
    .unwrap();
    std::fs::write(root.join("arch.md"), "# Arch\nArch content.").unwrap();
    std::fs::write(root.join("CLAUDE.md"), "# Claude\nClaude content.").unwrap();

    let state = std::sync::Arc::new(tokio::sync::RwLock::new(
        llm_wiki_core::ipc::handlers::AppState::new(root.to_path_buf()),
    ));

    // 1. Call governance/getHarnessStatus
    let req = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: "governance/getHarnessStatus".to_string(),
        params: None,
    };
    let res = llm_wiki_core::ipc::handlers::handle_request(&state, req).await;
    assert!(res.error.is_none(), "Unexpected error: {:?}", res.error);
    let result = res.result.unwrap();

    let harnesses = result["harnesses"].as_array().unwrap();
    assert_eq!(harnesses.len(), 2);
    assert_eq!(result["active_harness"]["file_path"], "AGENTS.md");
    assert!(result["capability_matrix"]["overall_governance_score"].is_number());

    // Check embedded index on AGENTS.md
    let agents_harness = harnesses
        .iter()
        .find(|h| h["file_path"] == "AGENTS.md")
        .unwrap();
    assert!(agents_harness["embedded_index"].is_object());
    assert_eq!(
        agents_harness["embedded_index"]["routes"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    // 2. Call governance/setActiveHarness to switch to CLAUDE.md
    let req2 = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(2),
        method: "governance/setActiveHarness".to_string(),
        params: Some(serde_json::json!({ "harness_file": "CLAUDE.md" })),
    };
    let res2 = llm_wiki_core::ipc::handlers::handle_request(&state, req2).await;
    assert!(res2.error.is_none());
    let result2 = res2.result.unwrap();
    assert_eq!(result2["active_harness"], "CLAUDE.md");

    // 3. Confirm getHarnessStatus now returns CLAUDE.md as active
    let req3 = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(3),
        method: "governance/getHarnessStatus".to_string(),
        params: None,
    };
    let res3 = llm_wiki_core::ipc::handlers::handle_request(&state, req3).await;
    let result3 = res3.result.unwrap();
    assert_eq!(result3["active_harness"]["file_path"], "CLAUDE.md");
}

#[test]
fn test_harness_quality_scoring_t017() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let docs = root.join("docs");
    std::fs::create_dir_all(&docs).unwrap();
    std::fs::write(docs.join("arch.md"), "# Architecture\nContent").unwrap();

    let agents_content = r#"# AGENTS.md
Introduction.
## Knowledge Index
### Arch
- [Arch](docs/arch.md)
- [Broken](docs/missing.md)
"#;
    std::fs::write(root.join("AGENTS.md"), agents_content).unwrap();

    let mut harnesses = HarnessDetector::detect_all(root, &[]);
    let parser = IndexParser::new(root);
    harnesses[0].embedded_index = parser.parse_harness_index("AGENTS.md", agents_content);

    let ratings = llm_wiki_core::harness::HarnessQualityAuditor::audit_harness(
        &harnesses[0],
        agents_content,
    );

    // Broken file causes integrity penalty
    assert!(ratings.integrity_score < 100.0, "Integrity score should have deduction for missing.md");
    assert!(ratings.discoverability_score > 0.0);
    assert!(ratings.context_efficiency > 0.0);
}

#[test]
fn test_large_cohesive_harness_no_size_penalty_invariant_xiii_t018() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let docs = root.join("docs");
    std::fs::create_dir_all(&docs).unwrap();
    std::fs::write(docs.join("core.md"), "# Core\nCore design.").unwrap();

    // Construct a large 25k-token cohesive document
    let mut large_content = String::from("# AGENTS.md — Large Architecture Guide\n\n## Overview\n");
    for i in 0..1500 {
        large_content.push_str(&format!("Comprehensive architectural context sentence number {} with detailed explanations.\n", i));
    }
    large_content.push_str("\n## Knowledge Index\n### Architecture\n- [Core Design](docs/core.md): Core design.\n");

    std::fs::write(root.join("AGENTS.md"), &large_content).unwrap();

    let mut harnesses = HarnessDetector::detect_all(root, &[]);
    let parser = IndexParser::new(root);
    harnesses[0].embedded_index = parser.parse_harness_index("AGENTS.md", &large_content);

    let ratings = llm_wiki_core::harness::HarnessQualityAuditor::audit_harness(
        &harnesses[0],
        &large_content,
    );

    // Invariant XIII: Structure score must NOT be penalized simply for being large (>20,000 tokens)
    // as long as there are no broken routes or secondary routing layers.
    assert_eq!(ratings.integrity_score, 100.0);
    assert_eq!(ratings.structure_score, 100.0, "Invariant XIII: Healthy large harness must not be penalized for size alone");
}

#[tokio::test]
async fn test_configure_harness_base_rpc() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Initial state: create AGENTS.md, CLAUDE.md, and legacy index.md with routes
    std::fs::write(
        root.join("AGENTS.md"),
        "# AGENTS.md\n## Knowledge Index\n### Architecture\n- [Design](design.md)\n",
    )
    .unwrap();
    std::fs::write(root.join("CLAUDE.md"), "# Old Claude\nOld instructions.\n").unwrap();
    std::fs::write(
        root.join("index.md"),
        "# Knowledge Index\n- [Legacy Target](legacy.md)\n",
    )
    .unwrap();
    std::fs::write(root.join("design.md"), "# Design\nSystem design.\n").unwrap();

    let state = std::sync::Arc::new(tokio::sync::RwLock::new(
        llm_wiki_core::ipc::handlers::AppState::new(root.to_path_buf()),
    ));

    // 2. Call governance/configureHarnessBase with Cursor/Codex (AGENTS.md) + Agy (GEMINI.md)
    // Claude Code is omitted, clean_unused: true, consolidate_index: true, create_folders: [.cursor/rules, .agents]
    let req = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: "governance/configureHarnessBase".to_string(),
        params: Some(serde_json::json!({
            "active_harness": "AGENTS.md",
            "enabled_harnesses": ["AGENTS.md", "GEMINI.md"],
            "create_folders": [".cursor/rules", ".agents"],
            "consolidate_index": true,
            "clean_unused": true
        })),
    };

    let res = llm_wiki_core::ipc::handlers::handle_request(&state, req).await;
    assert!(res.error.is_none(), "configureHarnessBase must succeed");
    let result = res.result.unwrap();

    assert_eq!(result["active_harness"], "AGENTS.md");
    assert_eq!(
        result["enabled_harnesses"].as_array().unwrap().len(),
        2
    );

    // 3. Verify filesystem changes:
    // .cursor/rules and .agents must exist
    assert!(root.join(".cursor/rules").is_dir(), ".cursor/rules must be created");
    assert!(root.join(".agents").is_dir(), ".agents must be created");

    // GEMINI.md must be initialized with template including Knowledge Index
    assert!(root.join("GEMINI.md").is_file(), "GEMINI.md must be generated");
    let gemini_content = std::fs::read_to_string(root.join("GEMINI.md")).unwrap();
    assert!(gemini_content.contains("Knowledge Index"), "GEMINI.md must have embedded Knowledge Index");

    // CLAUDE.md must be deleted (clean_unused was true and not enabled)
    assert!(!root.join("CLAUDE.md").exists(), "CLAUDE.md must be removed when unselected");

    // index.md must be safely deleted
    assert!(!root.join("index.md").exists(), "index.md must be safely deleted when consolidated");

    // .context-vault.yaml must reflect active harness and enabled entry points
    let cfg = llm_wiki_core::config::ContextVaultConfig::load_from_workspace(root);
    assert_eq!(cfg.get_active_harness(), Some("AGENTS.md".to_string()));
    assert_eq!(
        cfg.get_harness_entry_points(),
        vec!["AGENTS.md".to_string(), "GEMINI.md".to_string()]
    );
}
