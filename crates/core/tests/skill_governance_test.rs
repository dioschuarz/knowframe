use llm_wiki_core::harness::procedure::ProcedureExtractor;
use llm_wiki_core::harness::skill::SkillAuditor;
use llm_wiki_core::harness::SkillValidationStatus;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::RwLock;

#[test]
fn test_skill_structure_validation_t027() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Valid Skill with script and template
    let valid_skill_dir = root.join(".agents").join("skills").join("deploy-service");
    fs::create_dir_all(valid_skill_dir.join("scripts")).unwrap();
    fs::create_dir_all(valid_skill_dir.join("templates")).unwrap();
    fs::write(
        valid_skill_dir.join("SKILL.md"),
        r#"---
name: deploy-service
description: Deploys service to staging or production environment.
---
# Deploy Service
Run the deploy script [deploy.sh](scripts/deploy.sh) using template [config.yaml](templates/config.yaml).
"#,
    )
    .unwrap();
    fs::write(valid_skill_dir.join("scripts").join("deploy.sh"), "#!/bin/bash\necho deploy").unwrap();
    fs::write(valid_skill_dir.join("templates").join("config.yaml"), "env: prod\n").unwrap();

    // 2. Skill with broken asset reference
    let broken_skill_dir = root.join(".agents").join("skills").join("broken-asset");
    fs::create_dir_all(broken_skill_dir.join("scripts")).unwrap();
    fs::write(
        broken_skill_dir.join("SKILL.md"),
        r#"---
name: broken-asset
description: Has missing supporting script.
---
# Broken Asset
Execute missing script [missing.sh](scripts/missing.sh).
"#,
    )
    .unwrap();
    // note: scripts/missing.sh intentionally NOT created on disk

    // 3. Skill with missing manifest (directory without SKILL.md)
    let missing_manifest_dir = root.join(".agents").join("skills").join("no-manifest");
    fs::create_dir_all(missing_manifest_dir.join("scripts")).unwrap();
    fs::write(missing_manifest_dir.join("scripts").join("run.sh"), "#!/bin/bash\n").unwrap();

    // 4. Duplicate capability skills
    let dup_a_dir = root.join(".agents").join("skills").join("lint-code-a");
    fs::create_dir_all(&dup_a_dir).unwrap();
    fs::write(
        dup_a_dir.join("SKILL.md"),
        r#"---
name: code-linter
description: Run static code analysis and format verification across all project files.
---
# Code Linter
Run static code analysis and format verification across all project files.
"#,
    )
    .unwrap();

    let dup_b_dir = root.join(".agents").join("skills").join("lint-code-b");
    fs::create_dir_all(&dup_b_dir).unwrap();
    fs::write(
        dup_b_dir.join("SKILL.md"),
        r#"---
name: code-linter
description: Run static code analysis and format verification across all project files.
---
# Code Linter
Run static code analysis and format verification across all project files.
"#,
    )
    .unwrap();

    let vault_files = vec!["01_projects/guide.md".to_string()];
    let harness_routes = vec![];

    let skills = SkillAuditor::audit_repository(root, &harness_routes, &vault_files);

    // Verify valid skill
    let valid_skill = skills.iter().find(|s| s.skill_id == "deploy-service").expect("deploy-service found");
    assert_eq!(valid_skill.status, SkillValidationStatus::Valid);
    assert_eq!(valid_skill.name, "deploy-service");
    assert!(valid_skill.description.contains("Deploys service"));
    assert_eq!(valid_skill.assets.len(), 2);
    assert!(valid_skill.assets.iter().all(|a| a.exists_on_disk));
    assert!(valid_skill.unreferenced, "Should be unreferenced when not in routes or vault files");

    // Verify broken asset reference
    let broken_skill = skills.iter().find(|s| s.skill_id == "broken-asset").expect("broken-asset found");
    assert_eq!(broken_skill.status, SkillValidationStatus::BrokenAssetReference);
    let missing_asset = broken_skill.assets.iter().find(|a| a.file_path.contains("missing.sh")).expect("missing asset found");
    assert!(!missing_asset.exists_on_disk);

    // Verify missing manifest
    let no_manifest = skills.iter().find(|s| s.skill_id == "no-manifest").expect("no-manifest found");
    assert_eq!(no_manifest.status, SkillValidationStatus::MissingManifest);

    // Verify duplicate capability
    let dup_a = skills.iter().find(|s| s.skill_id == "lint-code-a").expect("lint-code-a found");
    let dup_b = skills.iter().find(|s| s.skill_id == "lint-code-b").expect("lint-code-b found");
    assert_eq!(dup_a.status, SkillValidationStatus::DuplicateCapability);
    assert_eq!(dup_b.status, SkillValidationStatus::DuplicateCapability);
    assert!(dup_a.overlap_ids.contains(&"lint-code-b".to_string()));
    assert!(dup_b.overlap_ids.contains(&"lint-code-a".to_string()));
}

#[test]
fn test_procedure_extraction_and_candidate_scoring_t028() {
    let mut documents = HashMap::new();

    let doc1 = r#"# Production Setup
Follow these steps for database migration:
1. Run backup script `scripts/db_backup.sh`
2. Apply migration schema
3. Verify index integrity
4. Notify slack channel
"#;

    let doc2 = r#"# Disaster Recovery
In case of recovery:
1. Run backup script `scripts/db_backup.sh`
2. Apply migration schema
3. Verify index integrity
4. Notify slack channel
"#;

    let doc3 = r#"# Release Checklist
Before releasing:
1. Run backup script `scripts/db_backup.sh`
2. Apply migration schema
3. Verify index integrity
4. Notify slack channel
"#;

    let doc4 = r#"# One off task
1. Single step procedure that never repeats
"#;

    documents.insert("docs/setup.md".to_string(), doc1.to_string());
    documents.insert("docs/recovery.md".to_string(), doc2.to_string());
    documents.insert("docs/release.md".to_string(), doc3.to_string());
    documents.insert("docs/single.md".to_string(), doc4.to_string());

    let candidates = ProcedureExtractor::extract_candidates(&documents);

    assert_eq!(candidates.len(), 1, "Expected exactly 1 recurring procedure candidate");
    let cand = &candidates[0];
    assert_eq!(cand.occurrence_count, 3);
    assert!(cand.recurrence_score >= 0.70, "Recurrence score must be >= 0.70");
    assert_eq!(cand.step_signatures.len(), 4);
    assert!(cand.source_documents.contains(&"docs/setup.md".to_string()));
    assert!(cand.source_documents.contains(&"docs/recovery.md".to_string()));
    assert!(cand.source_documents.contains(&"docs/release.md".to_string()));
    assert!(cand.associated_scripts.iter().any(|s| s.contains("db_backup.sh")));
    assert!(!cand.recommended_skill_name.is_empty());
}

#[tokio::test]
async fn test_get_skill_catalog_rpc_t029() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create a skill
    let skill_dir = root.join(".agents").join("skills").join("test-skill");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        r#"---
name: test-skill
description: A test skill for RPC.
---
# Test Skill
Test content.
"#,
    )
    .unwrap();

    // Create documents with a repeating procedure
    let docs_dir = root.join("docs");
    fs::create_dir_all(&docs_dir).unwrap();
    let proc_text = r#"# Guide
1. Step alpha in workflow
2. Step beta in workflow
3. Step gamma in workflow
"#;
    fs::write(docs_dir.join("doc1.md"), proc_text).unwrap();
    fs::write(docs_dir.join("doc2.md"), proc_text).unwrap();

    let state = Arc::new(RwLock::new(llm_wiki_core::ipc::handlers::AppState::new(root.to_path_buf())));

    let req = llm_wiki_core::ipc::protocol::JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(10),
        method: "governance/getSkillCatalog".to_string(),
        params: None,
    };

    let res = llm_wiki_core::ipc::handlers::handle_request(&state, req).await;
    assert!(res.error.is_none(), "RPC error: {:?}", res.error);
    let result = res.result.expect("RPC result");

    assert!(result["skills"].is_array());
    let skills = result["skills"].as_array().unwrap();
    assert!(skills.iter().any(|s| s["skill_id"] == "test-skill"));

    assert!(result["procedure_candidates"].is_array());
    let candidates = result["procedure_candidates"].as_array().unwrap();
    assert!(!candidates.is_empty());
}
