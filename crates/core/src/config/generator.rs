//! Auto-generator for .context-vault.yaml default configuration and Git hook manifests.

use super::ContextVaultConfig;
use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn generate_default_config(discovered_hub: Option<&str>) -> ContextVaultConfig {
    let mut config = ContextVaultConfig::default();
    if let Some(hub) = discovered_hub {
        config.vault.auto_discovered_hub = Some(hub.to_string());
        config.vault.hubs.push(hub.to_string());
    }
    config
}

pub fn write_config_if_missing(
    workspace_root: &Path,
    discovered_hub: Option<&str>,
) -> Result<bool, std::io::Error> {
    if ContextVaultConfig::find_config_path(workspace_root).is_some() {
        return Ok(false);
    }
    let config_path = ContextVaultConfig::canonical_config_path(workspace_root);
    let config = generate_default_config(discovered_hub);
    let yaml = serde_yaml::to_string(&config).map_err(|e| std::io::Error::other(e.to_string()))?;
    std::fs::write(&config_path, yaml)?;
    Ok(true)
}

/// Generates Tier 1 (.pre-commit-config.yaml for `prek`) dual hook content.
pub fn generate_tier1_prek_yaml() -> String {
    r#"# LLM Wiki Admission Control & Sync Hooks (Tier 1: prek / pre-commit)
repos:
  - repo: local
    hooks:
      - id: llm-wiki-validate
        name: LLM Wiki Pre-Commit Knowledge Admission Validator
        entry: llm-wiki-core validate --staged-only --timeout-ms 500
        language: system
        stages: [pre-commit]
        pass_filenames: false
      - id: llm-wiki-sync
        name: LLM Wiki Post-Commit Knowledge Graph Synchronizer
        entry: llm-wiki-core git-delta --timeout-ms 500
        language: system
        stages: [post-commit]
        always_run: true
        pass_filenames: false
"#
    .to_string()
}

/// Writes Tier 1 (.pre-commit-config.yaml) if not present.
pub fn write_tier1_prek_if_missing(workspace_root: &Path) -> Result<bool, std::io::Error> {
    let config_path = workspace_root.join(".pre-commit-config.yaml");
    if config_path.exists() {
        return Ok(false);
    }
    let content = generate_tier1_prek_yaml();
    fs::write(&config_path, content)?;
    Ok(true)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tier2DetectionResult {
    PreCommitAppended,
    PreCommitAlreadyConfigured,
    LefthookAppended,
    LefthookAlreadyConfigured,
    NoneDetected,
}

/// Detects Tier 2 ecosystem hook managers (pre-commit or lefthook) and appends hooks without collisions.
pub fn detect_and_configure_tier2_hooks(
    workspace_root: &Path,
) -> Result<Tier2DetectionResult, std::io::Error> {
    // 1. Check existing .pre-commit-config.yaml
    let pre_commit_path = workspace_root.join(".pre-commit-config.yaml");
    if pre_commit_path.is_file() {
        let content = fs::read_to_string(&pre_commit_path)?;
        if content.contains("llm-wiki-validate") || content.contains("llm-wiki-sync") {
            return Ok(Tier2DetectionResult::PreCommitAlreadyConfigured);
        }

        let snippet = r#"
  - repo: local
    hooks:
      - id: llm-wiki-validate
        name: LLM Wiki Pre-Commit Knowledge Admission Validator
        entry: llm-wiki-core validate --staged-only --timeout-ms 500
        language: system
        stages: [pre-commit]
        pass_filenames: false
      - id: llm-wiki-sync
        name: LLM Wiki Post-Commit Knowledge Graph Synchronizer
        entry: llm-wiki-core git-delta --timeout-ms 500
        language: system
        stages: [post-commit]
        always_run: true
        pass_filenames: false
"#;
        let mut new_content = content;
        if !new_content.contains("repos:") {
            new_content.push_str("\nrepos:\n");
        }
        new_content.push_str(snippet);
        fs::write(&pre_commit_path, new_content)?;
        return Ok(Tier2DetectionResult::PreCommitAppended);
    }

    // 2. Check lefthook.yml or .lefthook.yml
    let lefthook_path = workspace_root.join("lefthook.yml");
    let dot_lefthook_path = workspace_root.join(".lefthook.yml");
    let target_left = if lefthook_path.is_file() {
        Some(lefthook_path)
    } else if dot_lefthook_path.is_file() {
        Some(dot_lefthook_path)
    } else {
        None
    };

    if let Some(path) = target_left {
        let content = fs::read_to_string(&path)?;
        if content.contains("llm-wiki-core validate") || content.contains("llm-wiki-core git-delta")
        {
            return Ok(Tier2DetectionResult::LefthookAlreadyConfigured);
        }

        let snippet = r#"
pre-commit:
  commands:
    llm-wiki-validate:
      run: llm-wiki-core validate --staged-only --timeout-ms 500
      fail_text: "Knowledge admission validation failed. Run 'llm-wiki-core validate' for details."

post-commit:
  commands:
    llm-wiki-sync:
      run: llm-wiki-core git-delta --timeout-ms 500
"#;
        let mut new_content = content;
        new_content.push_str(snippet);
        fs::write(&path, new_content)?;
        return Ok(Tier2DetectionResult::LefthookAppended);
    }

    Ok(Tier2DetectionResult::NoneDetected)
}

/// Installs Tier 3 zero-dependency fallback shell scripts into .git/hooks/.
pub fn install_tier3_shell_hooks(workspace_root: &Path) -> Result<(), std::io::Error> {
    let hooks_dir = workspace_root.join(".git").join("hooks");
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir)?;
    }

    let pre_commit_script = r#"#!/bin/sh
# LLM Wiki Pre-Commit Knowledge Admission Control Hook
# Strict <= 500ms non-blocking fail-open ceiling
if command -v llm-wiki-core >/dev/null 2>&1; then
  llm-wiki-core validate --staged-only --timeout-ms 500 || exit $?
fi
exit 0
"#;

    let post_commit_script = r#"#!/bin/sh
# LLM Wiki Post-Commit Knowledge Graph Synchronizer Hook
# Non-blocking fail-open
if command -v llm-wiki-core >/dev/null 2>&1; then
  llm-wiki-core git-delta --timeout-ms 500 >/dev/null 2>&1 || true
fi
exit 0
"#;

    let pre_commit_path = hooks_dir.join("pre-commit");
    let post_commit_path = hooks_dir.join("post-commit");

    fs::write(&pre_commit_path, pre_commit_script)?;
    fs::write(&post_commit_path, post_commit_script)?;

    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&pre_commit_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit_path, perms)?;

        let mut perms_post = fs::metadata(&post_commit_path)?.permissions();
        perms_post.set_mode(0o755);
        fs::set_permissions(&post_commit_path, perms_post)?;
    }

    Ok(())
}
