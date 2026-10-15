use llm_wiki_core::ast::parser::parse_markdown_ast;
use llm_wiki_core::cli::materialize_cmd::execute_materialize;
use llm_wiki_core::cli::validate::execute_validate;
use llm_wiki_core::fragment::extractor::extract_fragments_from_document;
use llm_wiki_core::fragment::frontmatter::{
    compute_content_hash, inject_materialized_frontmatter, is_frontmatter_cache_valid,
    parse_materialized_frontmatter, split_frontmatter_and_body,
};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_non_destructive_frontmatter_update_t013() {
    let original = r#"---
title: System Architecture
author: Core Team
tags:
  - architecture
  - backend
custom_setting:
  enabled: true
  retries: 3
---
# System Architecture

## Authentication
OAuth details here.

## Database
PostgreSQL configuration.
"#;

    let parsed = parse_markdown_ast("arch.md", original);
    let fragments = extract_fragments_from_document(original, &parsed.edges);
    assert_eq!(fragments.len(), 3);

    let updated =
        inject_materialized_frontmatter(original, &fragments).expect("inject frontmatter");

    // Verify user keys are preserved
    assert!(updated.contains("title: System Architecture"));
    assert!(updated.contains("author: Core Team"));
    assert!(updated.contains("custom_setting:"));
    assert!(updated.contains("enabled: true"));
    assert!(updated.contains("retries: 3"));

    // Verify wiki fragments are injected
    assert!(updated.contains("wiki:"));
    assert!(updated.contains("content_hash:"));
    assert!(updated.contains("fragments:"));
    assert!(updated.contains("authentication:"));
    assert!(updated.contains("database:"));

    // Verify body content is intact
    assert!(updated.contains("# System Architecture"));
    assert!(updated.contains("OAuth details here."));
    assert!(updated.contains("PostgreSQL configuration."));

    // Verify parsing materialized frontmatter from updated file
    let (fm_opt, body) = split_frontmatter_and_body(&updated);
    let fm_str = fm_opt.expect("frontmatter exists");
    let mat = parse_materialized_frontmatter(&fm_str).expect("parsed materialized frontmatter");
    assert_eq!(mat.fragments.len(), 3);
    assert!(mat.fragments.contains_key("authentication"));
    assert!(mat.fragments.contains_key("database"));
    assert!(is_frontmatter_cache_valid(&mat, &body));
}

#[test]
fn test_frontmatter_cache_validity_and_invalidation_t014() {
    let body = "# Section\nContent here.\n";
    let hash = compute_content_hash(body);

    let mut fragments = std::collections::BTreeMap::new();
    fragments.insert(
        "section".to_string(),
        llm_wiki_core::fragment::MaterializedFragmentEntry {
            tokens: 10,
            inbound: 2,
            outbound: 1,
        },
    );

    let valid_frontmatter = llm_wiki_core::fragment::MaterializedWikiFrontmatter {
        content_hash: Some(hash),
        fragments: fragments.clone(),
    };

    assert!(is_frontmatter_cache_valid(&valid_frontmatter, body));

    // Invalidate by changing body text
    let mutated_body = "# Section\nMutated content.\n";
    assert!(!is_frontmatter_cache_valid(
        &valid_frontmatter,
        mutated_body
    ));
}

#[test]
fn test_cli_materialize_command_t016() {
    let dir = tempdir().expect("create temp dir");
    let doc_path = dir.path().join("service.md");
    fs::write(
        &doc_path,
        "# Payment Service\n\n## Card Processing\nStripe API integration.\n\n## Webhooks\nWebhook handling.\n",
    )
    .expect("write service.md");

    let count = execute_materialize(&doc_path, false).expect("execute materialize");
    assert_eq!(count, 1);

    let updated = fs::read_to_string(&doc_path).expect("read materialized service.md");
    assert!(updated.contains("wiki:"));
    assert!(updated.contains("card-processing:"));
    assert!(updated.contains("webhooks:"));

    // Re-running without modifications should report up to date (0 updated)
    let count2 = execute_materialize(&doc_path, false).expect("execute materialize second time");
    assert_eq!(count2, 0);
}

#[tokio::test]
async fn test_pre_commit_blocks_broken_fragment_anchor_t017b() {
    let dir = tempdir().expect("create temp dir");
    let root = dir.path();

    // Initialize git repository
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(root)
        .output()
        .expect("git init");

    std::process::Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(root)
        .output()
        .expect("git config name");

    std::process::Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(root)
        .output()
        .expect("git config email");

    // Committed target file
    let target_file = root.join("target.md");
    fs::write(
        &target_file,
        "# Target Note\n\n## Existing Heading\nContent.\n",
    )
    .expect("write target.md");

    std::process::Command::new("git")
        .args(["add", "target.md"])
        .current_dir(root)
        .output()
        .expect("git add target.md");

    std::process::Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(root)
        .output()
        .expect("git commit");

    // Establish baseline from clean initial commit
    llm_wiki_core::cli::baseline_cmd::execute_baseline_init(root, false).expect("init baseline");

    // Staged source file with broken fragment anchor
    let source_file = root.join("source.md");
    fs::write(
        &source_file,
        "# Source Note\nLink to [Missing Anchor](./target.md#non-existent-anchor)\n",
    )
    .expect("write source.md");

    std::process::Command::new("git")
        .args(["add", "source.md"])
        .current_dir(root)
        .output()
        .expect("git add source.md");

    // Execute validate: must BLOCK with exit code 1
    let exit_code = execute_validate(root, true, false, false, 5000).await;
    assert_eq!(
        exit_code, 1,
        "Pre-commit validator must block on broken fragment anchor regression"
    );
}
