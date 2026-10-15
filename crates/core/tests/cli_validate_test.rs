//! Integration tests for Phase 4: Pre-Commit Admission Control CLI & Dual Hooks.
//! Validates T018 through T023b.

use std::fs;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;

use llm_wiki_core::cli::baseline_cmd::{execute_baseline_init, execute_baseline_reset};
use llm_wiki_core::cli::delta::execute_git_delta;
use llm_wiki_core::cli::validate::execute_validate;
use llm_wiki_core::config::generator::{
    detect_and_configure_tier2_hooks, install_tier3_shell_hooks, write_tier1_prek_if_missing,
    Tier2DetectionResult,
};
use llm_wiki_core::debt::baseline::load_baseline;

fn init_git_repo(dir: &TempDir) {
    let p = dir.path();
    Command::new("git")
        .args(["init"])
        .current_dir(p)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(p)
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(p)
        .output()
        .unwrap();
}

#[tokio::test]
async fn test_cli_validate_command() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    // Initial clean committed state
    let agents_md = dir.path().join("AGENTS.md");
    fs::write(&agents_md, "# Agents entrypoint\nSee [[target]].\n").unwrap();
    let target_md = dir.path().join("target.md");
    fs::write(&target_md, "# Target\nContent here.\n").unwrap();

    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Establish baseline
    execute_baseline_init(dir.path(), false).unwrap();

    // No staged files -> exit 0
    let code = execute_validate(dir.path(), true, true, false, 500).await;
    assert_eq!(code, 0, "Clean stage should exit 0");

    // Stage a new note with a broken link
    let bad_note = dir.path().join("bad.md");
    fs::write(&bad_note, "# Bad note\nLink to [[missing_target]].\n").unwrap();
    Command::new("git")
        .args(["add", "bad.md"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Validation should return exit code 1 (BLOCK)
    let code_bad = execute_validate(dir.path(), true, true, false, 500).await;
    assert_eq!(
        code_bad, 1,
        "New regression should return exit code 1 (BLOCK)"
    );
}

#[tokio::test]
async fn test_cli_validate_json_output() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    // Initial committed baseline state
    let agents = dir.path().join("AGENTS.md");
    fs::write(&agents, "# Entrypoint\nWelcome.\n").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(dir.path())
        .output()
        .unwrap();
    execute_baseline_init(dir.path(), false).unwrap();

    // Stage a note with a new broken link regression
    let doc = dir.path().join("test.md");
    fs::write(&doc, "# Doc\nRef: [[does_not_exist]].\n").unwrap();
    Command::new("git")
        .args(["add", "test.md"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Should return 1 on block with JSON output
    let code = execute_validate(dir.path(), true, false, true, 500).await;
    assert_eq!(code, 1);
}

#[tokio::test]
async fn test_cli_validate_fail_open_on_timeout() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    let doc = dir.path().join("test.md");
    fs::write(&doc, "# Doc\nRef: [[does_not_exist]].\n").unwrap();
    Command::new("git")
        .args(["add", "test.md"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // With 0 ms timeout, timeout fires immediately, triggering fail-open (exit 0)
    let code = execute_validate(dir.path(), true, true, false, 0).await;
    assert_eq!(code, 0, "Fail-open on timeout must return 0");
}

#[tokio::test]
async fn test_cli_baseline_init_command() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    let doc1 = dir.path().join("doc1.md");
    fs::write(&doc1, "# Doc 1\nHas [[broken_ref]].\n").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    execute_baseline_init(dir.path(), false).unwrap();

    let baseline = load_baseline(dir.path())
        .unwrap()
        .expect("Baseline must exist");
    assert_eq!(baseline.broken_links.len(), 1);
    assert_eq!(baseline.orphan_nodes.len(), 1);
    assert_eq!(baseline.total_debt, 2);

    // Re-running without force should preserve existing baseline
    execute_baseline_init(dir.path(), false).unwrap();
    let baseline2 = load_baseline(dir.path()).unwrap().unwrap();
    assert_eq!(baseline2.baseline_commit, baseline.baseline_commit);

    // Reset should refresh baseline
    execute_baseline_reset(dir.path()).unwrap();
    let baseline3 = load_baseline(dir.path()).unwrap().unwrap();
    assert_eq!(baseline3.total_debt, 2);
}

#[tokio::test]
async fn test_cli_git_delta_ratchet_sync() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    // Initial state with AGENTS.md pointing to doc1.md, which has a broken link
    let agents = dir.path().join("AGENTS.md");
    fs::write(&agents, "# Agents\n[[doc1]]\n").unwrap();
    let doc1 = dir.path().join("doc1.md");
    fs::write(&doc1, "# Doc 1\nHas [[broken_ref]].\n").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "broken"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Baseline captured with 1 broken link (0 orphans because AGENTS -> doc1)
    execute_baseline_init(dir.path(), false).unwrap();
    let base1 = load_baseline(dir.path()).unwrap().unwrap();
    assert_eq!(base1.total_debt, 1);
    assert_eq!(base1.broken_links.len(), 1);

    // Now fix the broken link by creating target note
    let target = dir.path().join("broken_ref.md");
    fs::write(&target, "# Target\nFixed!\n").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "fix"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Run post-commit delta
    execute_git_delta(
        dir.path(),
        &["broken_ref.md".to_string()],
        None,
        true,
        Duration::from_millis(500),
    )
    .await
    .unwrap();

    // Baseline should have ratcheted down to 0
    let base2 = load_baseline(dir.path()).unwrap().unwrap();
    assert_eq!(
        base2.total_debt, 0,
        "Resolved debt should ratchet baseline to 0"
    );
    assert!(base2.broken_links.is_empty());
}

#[test]
fn test_dual_hook_config_generation() {
    let dir = TempDir::new().unwrap();
    let created = write_tier1_prek_if_missing(dir.path()).unwrap();
    assert!(created);

    let config_path = dir.path().join(".pre-commit-config.yaml");
    assert!(config_path.is_file());

    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("llm-wiki-validate"));
    assert!(content.contains("stages: [pre-commit]"));
    assert!(content.contains("llm-wiki-sync"));
    assert!(content.contains("stages: [post-commit]"));

    // Second call should return false (not overwriting)
    let created2 = write_tier1_prek_if_missing(dir.path()).unwrap();
    assert!(!created2);
}

#[test]
fn test_tier2_ecosystem_hook_detection() {
    let dir = TempDir::new().unwrap();

    // 1. Python pre-commit config
    let config_path = dir.path().join(".pre-commit-config.yaml");
    fs::write(&config_path, "repos:\n  - repo: https://github.com/astral-sh/ruff-pre-commit\n    rev: v0.0.1\n    hooks:\n      - id: ruff\n").unwrap();

    let res = detect_and_configure_tier2_hooks(dir.path()).unwrap();
    assert_eq!(res, Tier2DetectionResult::PreCommitAppended);

    let updated_content = fs::read_to_string(&config_path).unwrap();
    assert!(updated_content.contains("ruff"));
    assert!(updated_content.contains("llm-wiki-validate"));
    assert!(updated_content.contains("llm-wiki-sync"));

    // Running again detects already configured
    let res2 = detect_and_configure_tier2_hooks(dir.path()).unwrap();
    assert_eq!(res2, Tier2DetectionResult::PreCommitAlreadyConfigured);

    // 2. Lefthook config in separate dir
    let dir2 = TempDir::new().unwrap();
    let left_path = dir2.path().join("lefthook.yml");
    fs::write(&left_path, "colors: true\n").unwrap();

    let left_res = detect_and_configure_tier2_hooks(dir2.path()).unwrap();
    assert_eq!(left_res, Tier2DetectionResult::LefthookAppended);

    let left_content = fs::read_to_string(&left_path).unwrap();
    assert!(left_content.contains("llm-wiki-validate:"));
    assert!(left_content.contains("llm-wiki-sync:"));

    let left_res2 = detect_and_configure_tier2_hooks(dir2.path()).unwrap();
    assert_eq!(left_res2, Tier2DetectionResult::LefthookAlreadyConfigured);
}

#[test]
fn test_tier3_standalone_shell_hook_installation() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    install_tier3_shell_hooks(dir.path()).unwrap();

    let pre_hook = dir.path().join(".git").join("hooks").join("pre-commit");
    let post_hook = dir.path().join(".git").join("hooks").join("post-commit");

    assert!(pre_hook.is_file());
    assert!(post_hook.is_file());

    let pre_content = fs::read_to_string(&pre_hook).unwrap();
    assert!(pre_content.contains("llm-wiki-core validate"));
    assert!(pre_content.contains("exit 0"));

    let post_content = fs::read_to_string(&post_hook).unwrap();
    assert!(post_content.contains("llm-wiki-core git-delta"));
    assert!(post_content.contains("exit 0"));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let pre_perms = fs::metadata(&pre_hook).unwrap().permissions().mode();
        let post_perms = fs::metadata(&post_hook).unwrap().permissions().mode();
        assert_eq!(pre_perms & 0o111, 0o111, "pre-commit must be executable");
        assert_eq!(post_perms & 0o111, 0o111, "post-commit must be executable");
    }
}

#[tokio::test]
async fn test_pre_commit_admission_control_blocks_broken_harness_route_t020() {
    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    // Initial clean commit with valid AGENTS.md
    let agents_md = dir.path().join("AGENTS.md");
    fs::write(
        &agents_md,
        "# Agents\n## Knowledge Index\n### Core\n- [Valid](target.md)\n",
    )
    .unwrap();
    let target_md = dir.path().join("target.md");
    fs::write(&target_md, "# Target\nContent here.\n").unwrap();

    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Establish baseline
    execute_baseline_init(dir.path(), false).unwrap();

    // Now stage a broken route in AGENTS.md
    fs::write(
        &agents_md,
        "# Agents\n## Knowledge Index\n### Core\n- [Valid](target.md)\n- [Broken Route](missing_doc.md)\n",
    )
    .unwrap();
    Command::new("git")
        .args(["add", "AGENTS.md"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Execute validate command
    let exit_code = execute_validate(dir.path(), true, true, false, 500).await;
    // Invariant XII: Must reject admission because a new broken reference is introduced in staged changes
    assert_eq!(
        exit_code, 1,
        "Staged broken index route must block commit (Invariant XII)"
    );
}

#[tokio::test]
async fn test_pr_block_policy_on_unreviewed_code_and_receipt_pass_t017() {
    use llm_wiki_core::drift::review::resolve_document_review;
    use llm_wiki_core::cli::validate::execute_validate_with_paths;

    let dir = TempDir::new().unwrap();
    init_git_repo(&dir);

    // Initial commit: doc citing code
    fs::create_dir_all(dir.path().join("notes")).unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();

    let doc_content = r#"---
code_dependencies:
  - src/auth.rs
---
# Authentication
Documentation for authentication module.
"#;
    fs::write(dir.path().join("notes/auth.md"), doc_content).unwrap();
    fs::write(dir.path().join("src/auth.rs"), "pub fn auth() {}\n").unwrap();

    Command::new("git")
        .args(["add", "."])
        .current_dir(dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Establish baseline
    execute_baseline_init(dir.path(), false).unwrap();

    // Modify code file without updating documentation note
    fs::write(dir.path().join("src/auth.rs"), "pub fn auth_v2() {}\n").unwrap();
    Command::new("git")
        .args(["add", "src/auth.rs"])
        .current_dir(dir.path())
        .output()
        .unwrap();

    // Pre-commit validation without receipt: must BLOCK (exit code 1) per FR-007
    let code_blocked = execute_validate(dir.path(), true, true, false, 500).await;
    assert_eq!(
        code_blocked, 1,
        "Unreviewed modification to cited code file must block commit"
    );

    // Record review receipt for notes/auth.md
    resolve_document_review(dir.path(), &["notes/auth.md".to_string()], Some("HEAD"))
        .expect("record review receipt");

    // Pre-commit validation with receipt: must PASS (exit code 0) per FR-008
    let code_passed = execute_validate(dir.path(), true, true, false, 500).await;
    assert_eq!(
        code_passed, 0,
        "Commit must pass when review receipt is recorded in reviews.json"
    );

    // Also verify execute_validate_with_paths passes cleanly with receipt
    let code_paths_passed = execute_validate_with_paths(
        dir.path(),
        Some(vec!["src/auth.rs".to_string()]),
        true,
        true,
        false,
        500,
    )
    .await;
    assert_eq!(
        code_paths_passed, 0,
        "execute_validate_with_paths must pass when receipt is present"
    );
}

