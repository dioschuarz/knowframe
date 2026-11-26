//! Regression tests for issue #1: symlinked skill catalogs, glob excludes,
//! repo-prefixed folder aliases and GFM slugs for headings containing `&`.
#![cfg(unix)]

use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use llm_wiki_core::fragment::extractor::github_slug;
use llm_wiki_core::ipc::protocol::{EdgePayload, ScanResultPayload};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use tempfile::tempdir;

fn scan(root: &Path) -> ScanResultPayload {
    execute_scan(ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    })
    .unwrap()
}

fn edge<'a>(scan: &'a ScanResultPayload, source: &str, raw: &str) -> &'a EdgePayload {
    scan.edges
        .iter()
        .find(|e| e.source == source && e.raw_target.as_deref() == Some(raw))
        .unwrap_or_else(|| panic!("edge {source} -> {raw} not found"))
}

/// Vault layout mirroring ADR-163: the canonical skill lives in a nested repo whose
/// skills folder is excluded from the scan; the catalog exposes it through a symlink,
/// plus a repo-prefixed alias symlink.
fn build_vault(root: &Path) {
    fs::create_dir_all(root.join("showcases/hermes/skills/radar/references")).unwrap();
    fs::write(
        root.join("showcases/hermes/skills/radar/SKILL.md"),
        "# Radar\n\nSee [spec](references/spec.md).\n",
    )
    .unwrap();
    fs::write(
        root.join("showcases/hermes/skills/radar/references/spec.md"),
        "# Spec\n\n## Skill Catalog & Canonical Location\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("showcases/hermes/graphify-out")).unwrap();
    fs::write(
        root.join("showcases/hermes/graphify-out/GRAPH_REPORT.md"),
        "# Report\n\nCites `missing_module.py`.\n",
    )
    .unwrap();

    fs::create_dir_all(root.join("harness/skills/other")).unwrap();
    fs::write(root.join("harness/skills/other/SKILL.md"), "# Other\n").unwrap();
    symlink(
        "../../showcases/hermes/skills/radar",
        root.join("harness/skills/radar"),
    )
    .unwrap();
    symlink("radar", root.join("harness/skills/hermes-radar")).unwrap();
    // A cycle must not hang the scan.
    symlink("..", root.join("harness/skills/other/loop")).unwrap();

    fs::write(
        root.join("index.md"),
        "# Index\n\n\
         - [catalog](harness/skills/radar/SKILL.md)\n\
         - [prefixed alias](harness/skills/hermes-radar/SKILL.md)\n\
         - [anchor gfm](harness/skills/radar/references/spec.md#skill-catalog--canonical-location)\n\
         - [anchor legacy](harness/skills/radar/references/spec.md#skill-catalog-canonical-location)\n\
         - [broken](harness/skills/missing/SKILL.md)\n\
         - [broken anchor](harness/skills/radar/references/spec.md#nope)\n",
    )
    .unwrap();

    fs::write(
        root.join(".context-vault.yaml"),
        "version: '2.0'\n\
         exclude:\n  - '**/graphify-out/**'\n\
         scan:\n  include:\n    - '**/*.md'\n  exclude:\n    - 'showcases/hermes/skills/**'\n",
    )
    .unwrap();
}

#[test]
fn symlinked_catalog_resolves_and_genuine_breaks_remain() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    build_vault(root);

    let result = scan(root);
    let mut ids: Vec<_> = result.nodes.iter().map(|n| n.id.as_str()).collect();
    ids.sort();
    assert_eq!(
        ids,
        vec![
            "harness/skills/other/SKILL.md",
            "harness/skills/radar/SKILL.md",
            "harness/skills/radar/references/spec.md",
            "index.md",
        ],
        "symlinked dir scanned once per canonical file (path with fewest symlink hops), \
         real path excluded, glob exclude from top-level list honored, cycle skipped"
    );

    let catalog = edge(&result, "index.md", "harness/skills/radar/SKILL.md");
    assert!(!catalog.is_broken);
    assert!(catalog.is_clickable, "{:?}", catalog.non_clickable_reason);
    assert_eq!(catalog.target, "harness/skills/radar/SKILL.md");

    let prefixed = edge(&result, "index.md", "harness/skills/hermes-radar/SKILL.md");
    assert!(!prefixed.is_broken && prefixed.is_clickable);
    assert_eq!(prefixed.target, "harness/skills/radar/SKILL.md", "alias maps to canonical node");

    let gfm = edge(
        &result,
        "index.md",
        "harness/skills/radar/references/spec.md",
    );
    assert!(!gfm.is_broken);

    let broken = edge(&result, "index.md", "harness/skills/missing/SKILL.md");
    assert!(
        broken.is_broken,
        "directory-qualified link must not fall back to an unrelated SKILL.md"
    );

    let anchors: Vec<_> = result
        .edges
        .iter()
        .filter(|e| e.source == "index.md" && e.anchor.is_some())
        .map(|e| (e.anchor.clone().unwrap(), e.is_broken, e.is_clickable))
        .collect();
    assert!(anchors.contains(&("skill-catalog--canonical-location".into(), false, true)));
    assert!(anchors.contains(&("skill-catalog-canonical-location".into(), false, true)));
    assert!(anchors.iter().any(|(a, broken, _)| a == "nope" && *broken));

    // Relative link inside the symlinked folder resolves too.
    let inner = result
        .edges
        .iter()
        .find(|e| e.source == "harness/skills/radar/SKILL.md")
        .unwrap();
    assert!(!inner.is_broken && inner.is_clickable);
}

#[test]
fn github_slug_keeps_double_hyphen_for_ampersand() {
    assert_eq!(
        github_slug("Skill Catalog & Canonical Location"),
        "skill-catalog--canonical-location"
    );
    assert_eq!(github_slug("snake_case Heading"), "snake_case-heading");
    assert_eq!(github_slug("Decisões & Riscos"), "decisões--riscos");
}
