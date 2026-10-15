use llm_wiki_core::harness::GovernanceStrategyType;
use llm_wiki_core::evolution::transformation::{GovernancePlanGenerator, GovernancePlanStore};
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

#[test]
fn test_all_seven_governance_strategies_t045() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Prepare sample files
    let agents_md = root.join("AGENTS.md");
    fs::write(&agents_md, "# AGENTS.md\nWelcome to repo.\n").unwrap();

    let docs_dir = root.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(docs_dir.join("index.md"), "# Intermediate Index\n- [Arch](arch.md)\n").unwrap();
    fs::write(docs_dir.join("arch.md"), "# Architecture\n## System\nDetails\n").unwrap();

    let skills_dir = root.join(".agents").join("skills");
    fs::create_dir_all(skills_dir.join("lint-a")).unwrap();
    fs::write(skills_dir.join("lint-a").join("SKILL.md"), "# Lint A\n").unwrap();
    fs::create_dir_all(skills_dir.join("lint-b")).unwrap();
    fs::write(skills_dir.join("lint-b").join("SKILL.md"), "# Lint B\n").unwrap();

    let strategies = [
        (GovernanceStrategyType::EmbedKnowledgeIndex, "AGENTS.md"),
        (GovernanceStrategyType::EliminateSecondaryRoutingLayer, "docs/index.md"),
        (GovernanceStrategyType::CreateSkillFromProcedure, "proc_123"),
        (GovernanceStrategyType::ConsolidateDuplicateSkills, ".agents/skills/lint-a"),
        (GovernanceStrategyType::DecomposeHarness, "AGENTS.md"),
        (GovernanceStrategyType::IntroduceSectionAddressing, "docs/arch.md"),
        (GovernanceStrategyType::RestructureDirectoryDomain, "docs"),
    ];

    for (strat, target) in strategies {
        let plan = GovernancePlanGenerator::generate_plan(root, strat, target).unwrap();
        assert!(!plan.plan_id.is_empty(), "Plan ID must be non-empty for {:?}", strat);
        assert!(!plan.approval_token.is_empty(), "Approval token must be generated for {:?}", strat);
        assert_eq!(plan.strategy_type, strat);
        assert!(!plan.title.is_empty(), "Title must be set for {:?}", strat);
        assert!(!plan.unified_diff.is_empty(), "Diff preview must be non-empty for {:?}", strat);
        assert!(!plan.expected_benefits.is_empty(), "Expected benefits must be populated for {:?}", strat);
        assert!(!plan.countervailing_trade_offs.is_empty(), "Trade-offs must be populated for {:?}", strat);
    }
}

#[test]
fn test_approval_token_enforcement_invariant_xvi_t046() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let agents_md = root.join("AGENTS.md");
    fs::write(&agents_md, "# AGENTS.md\nInitial content\n").unwrap();

    let store = GovernancePlanStore::default();
    let plan = GovernancePlanGenerator::generate_plan(root, GovernanceStrategyType::EmbedKnowledgeIndex, "AGENTS.md").unwrap();
    let plan_id = plan.plan_id.clone();
    let valid_token = plan.approval_token.clone();
    store.save_plan(plan);

    // 1. Attempt apply with invalid approval token -> MUST FAIL (Invariant XVI)
    let bad_result = store.apply_plan(root, &plan_id, "invalid_forged_token");
    assert!(bad_result.is_err(), "Must reject invalid approval token");

    let untouched = fs::read_to_string(&agents_md).unwrap();
    assert_eq!(untouched, "# AGENTS.md\nInitial content\n", "Filesystem must remain untouched upon token validation failure");

    // 2. Apply with valid approval token -> MUST SUCCEED
    let good_result = store.apply_plan(root, &plan_id, &valid_token);
    assert!(good_result.is_ok(), "Must apply successfully with valid approval token");

    let applied_report = good_result.unwrap();
    assert!(applied_report.applied);
    assert_eq!(applied_report.plan_id, plan_id);
    assert!(applied_report.files_modified.contains(&"AGENTS.md".to_string()));

    let updated = fs::read_to_string(&agents_md).unwrap();
    assert!(updated.contains("## Knowledge Index"), "Target document should have been modified");
}

#[tokio::test]
async fn test_generate_and_apply_governance_plan_rpc_t047() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let agents = root.join("AGENTS.md");
    fs::write(&agents, "# AGENTS.md\nOriginal\n").unwrap();

    let state = Arc::new(RwLock::new(llm_wiki_core::ipc::handlers::AppState::new(root.to_path_buf())));

    // 1. Call governance/generateGovernancePlan
    let req_gen = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: "governance/generateGovernancePlan".to_string(),
        params: Some(serde_json::json!({
            "strategy_type": "embed_knowledge_index",
            "target_file_or_dir": "AGENTS.md"
        })),
    };

    let res_gen = llm_wiki_core::ipc::handlers::handle_request(&state, req_gen).await;
    assert!(res_gen.error.is_none(), "RPC error: {:?}", res_gen.error);
    let plan = res_gen.result.expect("Generated plan");

    let plan_id = plan["plan_id"].as_str().unwrap().to_string();
    let approval_token = plan["approval_token"].as_str().unwrap().to_string();
    assert!(!approval_token.is_empty());

    // 2. Call governance/applyGovernancePlan with invalid token -> should return error
    let req_apply_bad = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(2),
        method: "governance/applyGovernancePlan".to_string(),
        params: Some(serde_json::json!({
            "plan_id": plan_id,
            "approval_token": "wrong_token"
        })),
    };
    let res_apply_bad = llm_wiki_core::ipc::handlers::handle_request(&state, req_apply_bad).await;
    assert!(res_apply_bad.error.is_some(), "Must return error for wrong approval token");

    // 3. Call governance/applyGovernancePlan with valid token -> should succeed
    let req_apply_good = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(3),
        method: "governance/applyGovernancePlan".to_string(),
        params: Some(serde_json::json!({
            "plan_id": plan_id,
            "approval_token": approval_token
        })),
    };
    let res_apply_good = llm_wiki_core::ipc::handlers::handle_request(&state, req_apply_good).await;
    assert!(res_apply_good.error.is_none(), "RPC error: {:?}", res_apply_good.error);
    let apply_res = res_apply_good.result.expect("Apply result");

    assert_eq!(apply_res["applied"], true);
    assert_eq!(apply_res["plan_id"], plan_id);
    assert!(apply_res["re_measurement"]["delta_tokens"].as_i64().is_some());
}

#[test]
fn test_eliminate_secondary_routing_layer_safe_verification_and_removal() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Setup workspace with AGENTS.md, index.md, and referenced target files
    let arch_md = root.join("docs").join("arch.md");
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(&arch_md, "# Architecture\nSystem design.\n").unwrap();

    let sec_md = root.join("sec.md");
    fs::write(&sec_md, "# Security\nSecurity specs.\n").unwrap();

    // AGENTS.md already contains arch.md, but links to index.md
    let agents_md = root.join("AGENTS.md");
    fs::write(
        &agents_md,
        "# AGENTS.md\n\n## Knowledge Index\n- [Architecture](docs/arch.md)\n- [Legacy Index](index.md)\n",
    )
    .unwrap();

    // index.md has 3 routes:
    // - docs/arch.md (valid, already in AGENTS.md)
    // - sec.md (valid, MISSING from AGENTS.md -> must be migrated)
    // - nonexistent.md (invalid/broken -> skipped with warning)
    let index_md = root.join("index.md");
    fs::write(
        &index_md,
        "# Knowledge Index\n- [Architecture](docs/arch.md)\n- [Security](sec.md)\n- [Ghost](nonexistent.md)\n",
    )
    .unwrap();

    // 2. Audit routes
    let audit = llm_wiki_core::evolution::transformation::audit_secondary_layer_routes(
        root,
        "index.md",
        "AGENTS.md",
    );
    assert_eq!(audit.total_routes, 3);
    assert_eq!(audit.valid_routes.len(), 2);
    assert_eq!(audit.invalid_routes.len(), 1);
    assert_eq!(audit.already_in_harness.len(), 1);
    assert_eq!(audit.missing_from_harness.len(), 1);
    assert!(audit.missing_from_harness[0].contains("sec.md"));

    // 3. Generate governance plan
    let store = GovernancePlanStore::default();
    let plan = GovernancePlanGenerator::generate_plan(
        root,
        GovernanceStrategyType::EliminateSecondaryRoutingLayer,
        "index.md",
    )
    .unwrap();

    assert!(plan.title.contains("Safely Remove Secondary Routing Layer"));
    assert!(plan.unified_diff.contains("--- a/index.md"));
    assert!(plan.unified_diff.contains("+++ /dev/null"));
    assert!(plan.actions.iter().any(|a| matches!(a, llm_wiki_core::evolution::transformation::TransformationAction::DeleteDocument { file_path } if file_path == "index.md")));
    assert!(plan.actions.iter().any(|a| matches!(a, llm_wiki_core::evolution::transformation::TransformationAction::UpdateDocumentContent { file_path, .. } if file_path == "AGENTS.md")));

    let plan_id = plan.plan_id.clone();
    let approval_token = plan.approval_token.clone();
    store.save_plan(plan);

    // 4. Apply plan
    let result = store.apply_plan(root, &plan_id, &approval_token).unwrap();
    assert!(result.applied);

    // 5. Verify index.md was deleted
    assert!(!root.join("index.md").exists(), "index.md must be deleted after safe consolidation");

    // 6. Verify AGENTS.md now contains the missing valid route (sec.md) and has cleaned the link to index.md
    let updated_agents = fs::read_to_string(&agents_md).unwrap();
    assert!(updated_agents.contains("sec.md"), "AGENTS.md must have received missing valid route");
    assert!(!updated_agents.contains("(index.md)"), "AGENTS.md must not contain link to deleted index.md");
    assert!(!updated_agents.contains("Legacy Index"), "Reference to legacy index must be removed");
    assert!(updated_agents.contains("docs/arch.md"), "Existing valid route must be preserved");
}

#[test]
fn test_eliminate_secondary_routing_layer_multi_harness_drift_prevention() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Setup workspace with AGENTS.md, GEMINI.md, and index.md
    let arch_md = root.join("docs").join("arch.md");
    fs::create_dir_all(root.join("docs")).unwrap();
    fs::write(&arch_md, "# Architecture\nSystem design.\n").unwrap();

    let sec_md = root.join("sec.md");
    fs::write(&sec_md, "# Security\nSecurity specs.\n").unwrap();

    let agents_md = root.join("AGENTS.md");
    fs::write(
        &agents_md,
        "# AGENTS.md\n\n## Knowledge Index\n- [Architecture](docs/arch.md)\n- [Legacy Index](index.md)\n",
    )
    .unwrap();

    let gemini_md = root.join("GEMINI.md");
    fs::write(
        &gemini_md,
        "# GEMINI.md\n\n## Knowledge Index\n- [Architecture](docs/arch.md)\n- [Old Route](index.md)\n",
    )
    .unwrap();

    let index_md = root.join("index.md");
    fs::write(
        &index_md,
        "# Knowledge Index\n- [Architecture](docs/arch.md)\n- [Security](sec.md)\n",
    )
    .unwrap();

    // 2. Generate plan
    let store = GovernancePlanStore::default();
    let plan = GovernancePlanGenerator::generate_plan(
        root,
        GovernanceStrategyType::EliminateSecondaryRoutingLayer,
        "index.md",
    )
    .unwrap();

    // Verify both AGENTS.md and GEMINI.md are included in actions and diff
    assert!(plan.actions.iter().any(|a| matches!(a, llm_wiki_core::evolution::transformation::TransformationAction::UpdateDocumentContent { file_path, .. } if file_path == "AGENTS.md")));
    assert!(plan.actions.iter().any(|a| matches!(a, llm_wiki_core::evolution::transformation::TransformationAction::UpdateDocumentContent { file_path, .. } if file_path == "GEMINI.md")));
    assert!(plan.actions.iter().any(|a| matches!(a, llm_wiki_core::evolution::transformation::TransformationAction::DeleteDocument { file_path } if file_path == "index.md")));
    assert!(plan.unified_diff.contains("--- a/AGENTS.md"));
    assert!(plan.unified_diff.contains("--- a/GEMINI.md"));
    assert!(plan.unified_diff.contains("--- a/index.md"));

    // 3. Apply plan
    let plan_id = plan.plan_id.clone();
    let approval_token = plan.approval_token.clone();
    store.save_plan(plan);
    let result = store.apply_plan(root, &plan_id, &approval_token).unwrap();
    assert!(result.applied);

    // 4. Verify BOTH files were updated and neither contains index.md links
    let updated_agents = fs::read_to_string(&agents_md).unwrap();
    assert!(updated_agents.contains("sec.md"), "AGENTS.md must receive missing route");
    assert!(!updated_agents.contains("index.md"), "AGENTS.md must clean index.md link");

    let updated_gemini = fs::read_to_string(&gemini_md).unwrap();
    assert!(updated_gemini.contains("sec.md"), "GEMINI.md must receive missing route");
    assert!(!updated_gemini.contains("index.md"), "GEMINI.md must clean index.md link");

    assert!(!root.join("index.md").exists(), "index.md must be deleted");
}

#[tokio::test]
async fn test_sync_harnesses_scoped_by_directory_and_isolated_subprojects() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Root harnesses: AGENTS.md (has arch.md and api.md) vs GEMINI.md (only has arch.md)
    let agents_md = root.join("AGENTS.md");
    fs::write(
        &agents_md,
        "# AGENTS.md\n\n## Knowledge Index\n- [Architecture](docs/arch.md)\n- [API](docs/api.md)\n",
    )
    .unwrap();

    let gemini_md = root.join("GEMINI.md");
    fs::write(
        &gemini_md,
        "# GEMINI.md\n\n## Knowledge Index\n- [Architecture](docs/arch.md)\n",
    )
    .unwrap();

    // 2. Isolated subproject harness: 04_harness/AGENTS.md (alone in 04_harness/)
    let harness_dir = root.join("04_harness");
    fs::create_dir_all(&harness_dir).unwrap();
    let sub_agents_md = harness_dir.join("AGENTS.md");
    fs::write(
        &sub_agents_md,
        "# 04_harness AGENTS\n\n## Knowledge Index\n- [Theme Skill](04_harness/skills/theme.md)\n",
    )
    .unwrap();

    // 3. Subproject with peers: showcases/ai_utils/AGENTS.md vs showcases/ai_utils/GEMINI.md
    let ai_utils_dir = root.join("showcases").join("ai_utils");
    fs::create_dir_all(&ai_utils_dir).unwrap();
    let show_agents_md = ai_utils_dir.join("AGENTS.md");
    fs::write(
        &show_agents_md,
        "# ai_utils AGENTS\n\n## Knowledge Index\n- [Code](showcases/ai_utils/skills/code.md)\n- [Eval](showcases/ai_utils/skills/eval.md)\n",
    )
    .unwrap();
    let show_gemini_md = ai_utils_dir.join("GEMINI.md");
    fs::write(
        &show_gemini_md,
        "# ai_utils GEMINI\n\n## Knowledge Index\n- [Code](showcases/ai_utils/skills/code.md)\n",
    )
    .unwrap();

    let state = Arc::new(RwLock::new(llm_wiki_core::ipc::handlers::AppState::new(root.to_path_buf())));

    // Populate topology in state so that subproject harnesses are known
    {
        let mut st = state.write().await;
        st.topology.nodes.insert("AGENTS.md".to_string(), llm_wiki_core::model::NoteNode::new("AGENTS.md", "AGENTS.md", "AGENTS.md"));
        st.topology.nodes.insert("GEMINI.md".to_string(), llm_wiki_core::model::NoteNode::new("GEMINI.md", "GEMINI.md", "GEMINI.md"));
        st.topology.nodes.insert("04_harness/AGENTS.md".to_string(), llm_wiki_core::model::NoteNode::new("04_harness/AGENTS.md", "04_harness/AGENTS.md", "04_harness/AGENTS.md"));
        st.topology.nodes.insert("showcases/ai_utils/AGENTS.md".to_string(), llm_wiki_core::model::NoteNode::new("showcases/ai_utils/AGENTS.md", "showcases/ai_utils/AGENTS.md", "showcases/ai_utils/AGENTS.md"));
        st.topology.nodes.insert("showcases/ai_utils/GEMINI.md".to_string(), llm_wiki_core::model::NoteNode::new("showcases/ai_utils/GEMINI.md", "showcases/ai_utils/GEMINI.md", "showcases/ai_utils/GEMINI.md"));
    }

    // Call governance/syncHarnesses
    let req = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(42),
        method: "governance/syncHarnesses".to_string(),
        params: Some(serde_json::json!({})),
    };

    let res = llm_wiki_core::ipc::handlers::handle_request(&state, req).await;
    assert!(res.error.is_none(), "RPC error: {:?}", res.error);
    let result = res.result.expect("Sync result");
    assert_eq!(result["synced_count"], 2); // GEMINI.md in root + showcases/ai_utils/GEMINI.md

    // 1. Verify root GEMINI.md received docs/api.md from root AGENTS.md, but NOT anything from 04_harness or showcases
    let updated_root_gemini = fs::read_to_string(&gemini_md).unwrap();
    assert!(updated_root_gemini.contains("docs/api.md"));
    assert!(!updated_root_gemini.contains("04_harness"));
    assert!(!updated_root_gemini.contains("showcases/ai_utils"));

    // 2. Verify isolated 04_harness/AGENTS.md was 100% UNTOUCHED
    let sub_agents_content = fs::read_to_string(&sub_agents_md).unwrap();
    assert!(!sub_agents_content.contains("docs/arch.md"));
    assert!(!sub_agents_content.contains("docs/api.md"));
    assert!(!sub_agents_content.contains("showcases/ai_utils"));
    assert!(sub_agents_content.contains("04_harness/skills/theme.md"));

    // 3. Verify showcases/ai_utils/GEMINI.md received eval.md from its peer in ai_utils, but NOT root or 04_harness routes
    let updated_show_gemini = fs::read_to_string(&show_gemini_md).unwrap();
    assert!(updated_show_gemini.contains("showcases/ai_utils/skills/eval.md"));
    assert!(!updated_show_gemini.contains("docs/arch.md"));
    assert!(!updated_show_gemini.contains("04_harness"));
}

#[test]
fn test_sync_harness_routes_reviewable_governance_plan_and_diff() {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    let sub_rules_dir = root.join("08_showcases").join("mcp-runner").join(".agents").join("rules");
    fs::create_dir_all(&sub_rules_dir).unwrap();

    let agents_md = sub_rules_dir.join("AGENTS.md");
    fs::write(
        &agents_md,
        "# MCP Runner Schema\n\n## 1. Project Overview\n- Master Wiki: [[mcp_runner]]\n- Active Context: [[active_context]]\n\n## Skills\n- [Runner Skill](skills/runner.md)\n",
    ).unwrap();

    let gemini_md = sub_rules_dir.join("GEMINI.md");
    fs::write(
        &gemini_md,
        "# GEMINI.md - Generic Instructions\n\nSome generic instructions without routes.\n",
    ).unwrap();

    let target = "08_showcases/mcp-runner/.agents/rules/GEMINI.md";

    // 1. Generate Governance Plan
    let plan = GovernancePlanGenerator::generate_plan(
        root,
        GovernanceStrategyType::SyncHarnessRoutes,
        target,
    )
    .unwrap();

    assert_eq!(plan.strategy_type, GovernanceStrategyType::SyncHarnessRoutes);
    assert_eq!(plan.target_file_or_dir, target);
    assert!(plan.title.contains("08_showcases/mcp-runner/.agents/rules"));
    assert!(!plan.approval_token.is_empty());
    assert_eq!(plan.approval_token.len(), 64); // SHA-256

    // 2. Verify Unified Diff preview
    assert!(plan.unified_diff.contains("--- a/08_showcases/mcp-runner/.agents/rules/GEMINI.md"));
    assert!(plan.unified_diff.contains("+++ b/08_showcases/mcp-runner/.agents/rules/GEMINI.md"));
    assert!(plan.unified_diff.contains("+## Knowledge Index"));
    assert!(plan.unified_diff.contains("mcp_runner"));
    assert!(plan.unified_diff.contains("active_context"));
    assert!(plan.unified_diff.contains("skills/runner.md"));

    // 3. Verify Invariant XV Itemized Benefits and Countervailing Trade-offs
    assert!(!plan.expected_benefits.is_empty());
    assert!(plan.expected_benefits.iter().any(|b| b.contains("route drift")));
    assert!(!plan.countervailing_trade_offs.is_empty());

    // 4. Test Invariant XVI Approval Gate
    let store = GovernancePlanStore::default();
    store.save_plan(plan.clone());

    // Tampered or missing token MUST fail
    let err_res = store.apply_plan(root, &plan.plan_id, "invalid_approval_token");
    assert!(err_res.is_err(), "Must reject invalid approval token");

    // Pre-apply: GEMINI.md must remain untouched
    let gemini_before = fs::read_to_string(&gemini_md).unwrap();
    assert!(!gemini_before.contains("## Knowledge Index"));

    // Valid approval token succeeds
    let apply_res = store.apply_plan(root, &plan.plan_id, &plan.approval_token).unwrap();
    assert!(apply_res.applied);
    assert_eq!(apply_res.files_modified, vec![target.to_string()]);

    // Post-apply: GEMINI.md has received the routes
    let gemini_after = fs::read_to_string(&gemini_md).unwrap();
    assert!(gemini_after.contains("## Knowledge Index"));
    assert!(gemini_after.contains("mcp_runner"));
    assert!(gemini_after.contains("active_context"));
    assert!(gemini_after.contains("skills/runner.md"));

    // AGENTS.md was the source and was untouched
    let agents_after = fs::read_to_string(&agents_md).unwrap();
    assert_eq!(agents_after, "# MCP Runner Schema\n\n## 1. Project Overview\n- Master Wiki: [[mcp_runner]]\n- Active Context: [[active_context]]\n\n## Skills\n- [Runner Skill](skills/runner.md)\n");
}

#[test]
fn test_sync_harness_routes_includes_callout_blockquotes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let root = temp_dir.path();

    let sub_rules_dir = root.join("08_showcases").join("mcp-runner").join(".agents").join("rules");
    fs::create_dir_all(&sub_rules_dir).unwrap();

    let agents_md = sub_rules_dir.join("AGENTS.md");
    fs::write(
        &agents_md,
        "# MCP Runner Schema\n\n> [!IMPORTANT]\n> **MANDATORY RULES**: All LLM agents MUST comply with [[MANDATORY_RULES]] and enforce the [[secret_storage_spec]] secret protection architecture.\n\n## 1. Project Overview\n- Master Wiki: [[mcp_runner]]\n",
    ).unwrap();

    let gemini_md = sub_rules_dir.join("GEMINI.md");
    fs::write(
        &gemini_md,
        "# GEMINI.md - Instructions\n\n## Knowledge Index\n- [[mcp_runner]]\n",
    ).unwrap();

    let target = "08_showcases/mcp-runner/.agents/rules/GEMINI.md";

    // 1. Generate plan for GEMINI.md
    let plan = GovernancePlanGenerator::generate_plan(
        root,
        GovernanceStrategyType::SyncHarnessRoutes,
        target,
    )
    .unwrap();

    // Must detect the 2 missing callout routes from AGENTS.md
    assert!(plan.unified_diff.contains("MANDATORY_RULES"));
    assert!(plan.unified_diff.contains("secret_storage_spec"));

    // 2. Apply plan
    let store = GovernancePlanStore::default();
    store.save_plan(plan.clone());
    let apply_res = store.apply_plan(root, &plan.plan_id, &plan.approval_token).unwrap();
    assert!(apply_res.applied);

    // 3. Verify GEMINI.md received both routes appended to its Knowledge Index
    let gemini_after = fs::read_to_string(&gemini_md).unwrap();
    assert!(gemini_after.contains("- [[MANDATORY_RULES]]"));
    assert!(gemini_after.contains("- [[secret_storage_spec]]"));

    // 4. Second plan generation must now report 100% parity
    let plan2 = GovernancePlanGenerator::generate_plan(
        root,
        GovernanceStrategyType::SyncHarnessRoutes,
        target,
    )
    .unwrap();

    assert!(plan2.unified_diff.contains("100% route parity"));
}



