use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use tempfile::tempdir;

#[test]
fn test_cross_submodule_relative_links_resolution() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create directory layout mimicking multi-repo / submodule workspace:
    // root/
    //   services/
    //     auth/
    //       README.md -> links to ../billing/README.md
    //     billing/
    //       README.md -> links to ../../docs/architecture.md#core-design
    //   docs/
    //     architecture.md -> # Core Design
    let auth_dir = root.join("services").join("auth");
    let billing_dir = root.join("services").join("billing");
    let docs_dir = root.join("docs");

    std::fs::create_dir_all(&auth_dir).unwrap();
    std::fs::create_dir_all(&billing_dir).unwrap();
    std::fs::create_dir_all(&docs_dir).unwrap();

    std::fs::write(
        auth_dir.join("README.md"),
        "# Auth Service\nSee billing at [Billing](../billing/README.md) and [[docs/architecture.md]].",
    )
    .unwrap();

    std::fs::write(
        billing_dir.join("README.md"),
        "# Billing Service\nRefer to [Architecture](../../docs/architecture.md#core-design).",
    )
    .unwrap();

    std::fs::write(
        docs_dir.join("architecture.md"),
        "# Enterprise Architecture\n## Core Design\nMulti-service architecture details.",
    )
    .unwrap();

    let options = ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    };

    let result = execute_scan(options).expect("Scan should succeed");

    assert_eq!(result.nodes.len(), 3);

    // Verify all edges are valid and unbroken
    for edge in &result.edges {
        assert!(
            !edge.is_broken,
            "Edge {} -> {} should NOT be broken (is_broken: {})",
            edge.source, edge.target, edge.is_broken
        );
    }

    // Verify cross-submodule relative link resolved
    let auth_to_billing = result
        .edges
        .iter()
        .find(|e| e.source == "services/auth/README.md" && e.target == "services/billing/README.md");
    assert!(auth_to_billing.is_some(), "auth README should link to billing README");
    assert!(!auth_to_billing.unwrap().is_broken);

    // Verify nested relative traversal with section anchor resolved
    let billing_to_arch = result
        .edges
        .iter()
        .find(|e| e.source == "services/billing/README.md" && e.target == "docs/architecture.md");
    assert!(billing_to_arch.is_some(), "billing README should link to docs/architecture.md");
    assert!(!billing_to_arch.unwrap().is_broken);
}

#[test]
fn test_vault_case_folded_wikilink_resolution() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let docs_dir = root.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();

    std::fs::write(
        root.join("index.md"),
        "# Index\nLink to [[Architecture]] in mixed-case.",
    )
    .unwrap();

    std::fs::write(
        docs_dir.join("architecture.md"),
        "# Architecture Overview\nDocumentation.",
    )
    .unwrap();

    let options = ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    };

    let result = execute_scan(options).expect("Scan should succeed");

    assert_eq!(result.nodes.len(), 2);
    let edge = result.edges.iter().find(|e| e.source == "index.md");
    assert!(edge.is_some());
    assert!(!edge.unwrap().is_broken);
    assert_eq!(edge.unwrap().target, "docs/architecture.md");
}
