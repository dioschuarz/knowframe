use llm_wiki_core::automation::link_normalizer::normalize_workspace_links;
use llm_wiki_core::cli::scan::{execute_scan, ScanOptions};
use llm_wiki_core::evolution::transformation::{GovernancePlanGenerator, GovernancePlanStore};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::graph::resolver::{compute_relative_posix_path, resolve_edge_integrity_with_policy, ResolverIndex};
use llm_wiki_core::harness::GovernanceStrategyType;
use llm_wiki_core::model::{LinkPortabilityMode, LinkType, NonClickableReason, NoteEdge};
use llm_wiki_core::quality::scorer::{calculate_quality_score, extract_all_issues};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_compute_relative_posix_path_cases() {
    assert_eq!(
        compute_relative_posix_path("notes/AGENTS.md", "notes/00_meta/wiki_index.md"),
        "00_meta/wiki_index.md"
    );
    assert_eq!(
        compute_relative_posix_path("notes/AGENTS.md", "AGENTS.md"),
        "../AGENTS.md"
    );
    assert_eq!(
        compute_relative_posix_path("notes/AGENTS.md", "knowframe/README.md"),
        "../knowframe/README.md"
    );
    assert_eq!(
        compute_relative_posix_path("notes/a/b/doc.md", "notes/x/y.md"),
        "../../x/y.md"
    );
    assert_eq!(
        compute_relative_posix_path("notes/AGENTS.md", "notes/AGENTS.md"),
        ""
    );
}

#[test]
fn test_clickability_detection_across_all_non_portable_forms() {
    let mut known = HashMap::new();
    known.insert("AGENTS.md".to_string(), PathBuf::from("AGENTS.md"));
    known.insert("notes/AGENTS.md".to_string(), PathBuf::from("notes/AGENTS.md"));
    known.insert("notes/00_meta/wiki_index.md".to_string(), PathBuf::from("notes/00_meta/wiki_index.md"));
    known.insert("knowframe/README.md".to_string(), PathBuf::from("knowframe/README.md"));
    known.insert("notes/My Guide.md".to_string(), PathBuf::from("notes/My Guide.md"));

    let mut headings = HashMap::new();
    headings.insert(
        "notes/00_meta/wiki_index.md".to_string(),
        vec!["Master Catalog".to_string()],
    );

    let index = ResolverIndex::new(&known);

    // 1. Wikilink -> non-clickable (WikilinkSyntax), resolves relative path cleanly
    let mut e_wiki = NoteEdge::new(
        "notes/AGENTS.md",
        "00_meta/wiki_index.md",
        LinkType::Wikilink,
        "Wiki Index",
        Some("master-catalog".to_string()),
        Some(10),
    );
    e_wiki.raw_target = Some("00_meta/wiki_index".to_string());
    resolve_edge_integrity_with_policy(&mut e_wiki, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_wiki.is_broken);
    assert!(!e_wiki.is_clickable);
    assert_eq!(e_wiki.non_clickable_reason, Some(NonClickableReason::WikilinkSyntax));
    assert_eq!(
        e_wiki.clickable_target.as_deref(),
        Some("00_meta/wiki_index.md#master-catalog")
    );

    // 2. Same wikilink under ObsidianWikilink policy -> allowed as clickable
    let mut e_obs = e_wiki.clone();
    e_obs.target = "00_meta/wiki_index.md".to_string();
    resolve_edge_integrity_with_policy(&mut e_obs, &index, &headings, LinkPortabilityMode::ObsidianWikilink);
    assert!(!e_obs.is_broken);
    assert!(e_obs.is_clickable);
    assert_eq!(e_obs.non_clickable_reason, None);

    // 3. Bare `[[AGENTS]]` inside `notes/AGENTS.md` resolves to root `AGENTS.md` -> `../AGENTS.md`
    let mut e_root = NoteEdge::new(
        "notes/AGENTS.md",
        "AGENTS.md",
        LinkType::Wikilink,
        "Root Schema",
        None,
        Some(12),
    );
    e_root.raw_target = Some("AGENTS".to_string());
    resolve_edge_integrity_with_policy(&mut e_root, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_root.is_broken);
    assert_eq!(e_root.target, "AGENTS.md");
    assert_eq!(e_root.clickable_target.as_deref(), Some("../AGENTS.md"));

    // 4. Standard Markdown missing `.md` extension -> MissingExtension
    let mut e_ext = NoteEdge::new(
        "notes/AGENTS.md",
        "00_meta/wiki_index",
        LinkType::Markdown,
        "Wiki",
        None,
        Some(14),
    );
    resolve_edge_integrity_with_policy(&mut e_ext, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_ext.is_broken);
    assert!(!e_ext.is_clickable);
    assert_eq!(e_ext.non_clickable_reason, Some(NonClickableReason::MissingExtension));
    assert_eq!(e_ext.clickable_target.as_deref(), Some("00_meta/wiki_index.md"));

    // 5. Standard Markdown with vault-root path (`knowframe/README.md` from `notes/AGENTS.md`) -> NonRelativePath
    let mut e_rel = NoteEdge::new(
        "notes/AGENTS.md",
        "knowframe/README.md",
        LinkType::Markdown,
        "Docs",
        None,
        Some(16),
    );
    resolve_edge_integrity_with_policy(&mut e_rel, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_rel.is_broken);
    assert!(!e_rel.is_clickable);
    assert_eq!(e_rel.non_clickable_reason, Some(NonClickableReason::NonRelativePath));
    assert_eq!(e_rel.clickable_target.as_deref(), Some("../knowframe/README.md"));

    // 6. Standard Markdown with case mismatch -> CaseMismatch
    let mut e_case = NoteEdge::new(
        "notes/AGENTS.md",
        "../knowframe/readme.md",
        LinkType::Markdown,
        "Docs",
        None,
        Some(18),
    );
    resolve_edge_integrity_with_policy(&mut e_case, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_case.is_broken);
    assert!(!e_case.is_clickable);
    assert_eq!(e_case.non_clickable_reason, Some(NonClickableReason::CaseMismatch));
    assert_eq!(e_case.clickable_target.as_deref(), Some("../knowframe/README.md"));

    // 7. Standard Markdown with unslugified anchor -> UnslugifiedAnchor
    let mut e_anch = NoteEdge::new(
        "notes/AGENTS.md",
        "00_meta/wiki_index.md",
        LinkType::Markdown,
        "Catalog",
        Some("Master Catalog".to_string()),
        Some(20),
    );
    resolve_edge_integrity_with_policy(&mut e_anch, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_anch.is_broken);
    assert!(!e_anch.is_clickable);
    assert_eq!(e_anch.non_clickable_reason, Some(NonClickableReason::UnslugifiedAnchor));
    assert_eq!(
        e_anch.clickable_target.as_deref(),
        Some("00_meta/wiki_index.md#master-catalog")
    );

    // 8. Proper standard Markdown relative link with encoded spaces -> CLICKABLE!
    let mut e_ok = NoteEdge::new(
        "notes/AGENTS.md",
        "My%20Guide.md",
        LinkType::Markdown,
        "My Guide",
        None,
        Some(22),
    );
    resolve_edge_integrity_with_policy(&mut e_ok, &index, &headings, LinkPortabilityMode::StandardMarkdown);
    assert!(!e_ok.is_broken);
    assert!(e_ok.is_clickable);
    assert_eq!(e_ok.non_clickable_reason, None);
}

#[test]
fn test_normalize_workspace_links_e2e_and_code_immunity() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join("notes/00_meta")).unwrap();
    fs::create_dir_all(root.join("knowframe")).unwrap();

    fs::write(root.join("AGENTS.md"), "# Root AGENTS\n").unwrap();
    fs::write(
        root.join("notes/00_meta/wiki_index.md"),
        "# Wiki Index\n\n## Core Tenets\nDetails here.\n",
    )
    .unwrap();
    fs::write(root.join("knowframe/README.md"), "# KnowFrame\n").unwrap();

    let notes_agents = r#"---
title: Notes Harness
note_ref: "[[00_meta/wiki_index]]"
---
# AGENTS.md

Always use `[[document#fragment|Label]]` in examples.

```markdown
- [[00_meta/wiki_index|Inside Code Block]]
```

- [[AGENTS|Root Workspace Schema]]
- [[notes/AGENTS|Self Mirror]]
- [[00_meta/wiki_index|Master Wiki Index]]
- [[00_meta/wiki_index#core-tenets|Core Tenets]]
- [[knowframe/README|Docs]]
- [NonRelative](knowframe/README.md)
- [MissingExt](00_meta/wiki_index)
- [[does_not_exist|Broken Target]]
- [[00_meta/wiki_index#missing-anchor|Broken Anchor]]

| Col | Link |
| --- | --- |
| Doc | [[00_meta/wiki_index\|Table Link]] |
"#;
    fs::write(root.join("notes/AGENTS.md"), notes_agents).unwrap();

    // 0. Scan BEFORE normalization and confirm EdgePayload::to_note_edge preserves non_clickable_link diagnostics
    let pre_scan = execute_scan(ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    })
    .unwrap();
    let mut pre_topology = GraphTopology::new();
    for n in &pre_scan.nodes {
        pre_topology.add_node(llm_wiki_core::model::NoteNode::new(&n.id, &n.file_path, &n.label));
    }
    for e in &pre_scan.edges {
        pre_topology.add_edge(e.to_note_edge());
    }
    let pre_issues = extract_all_issues(&pre_topology, &[], &[]);
    let pre_non_clickable: Vec<_> = pre_issues
        .iter()
        .filter(|i| i.category == "non_clickable_link")
        .collect();
    assert!(
        pre_non_clickable.len() >= 7,
        "Expected at least 7 non_clickable_link issues before normalization, got {}",
        pre_non_clickable.len()
    );

    // 1. Dry-run preview must NOT mutate disk
    let dry = normalize_workspace_links(root, Some("notes/AGENTS.md"), false).unwrap();
    assert!(!dry.applied);
    assert_eq!(dry.total_rewrites, 8);
    assert_eq!(dry.total_broken_skipped, 2);
    assert_eq!(fs::read_to_string(root.join("notes/AGENTS.md")).unwrap(), notes_agents);

    // 2. Apply rewrites via GovernancePlanGenerator (with approval token)
    let plan = GovernancePlanGenerator::generate_plan(
        root,
        GovernanceStrategyType::NormalizeLinksToMarkdown,
        "notes/AGENTS.md",
    )
    .unwrap();
    assert!(!plan.unified_diff.is_empty());
    assert_eq!(plan.actions.len(), 1);

    let store = GovernancePlanStore::default();
    store.save_plan(plan.clone());
    let applied = store
        .apply_plan(root, &plan.plan_id, &plan.approval_token)
        .unwrap();
    assert_eq!(applied.files_modified, vec!["notes/AGENTS.md".to_string()]);

    let updated = fs::read_to_string(root.join("notes/AGENTS.md")).unwrap();

    // Frontmatter, inline code, and fenced code blocks MUST remain intact
    assert!(updated.contains("note_ref: \"[[00_meta/wiki_index]]\""));
    assert!(updated.contains("`[[document#fragment|Label]]`"));
    assert!(updated.contains("- [[00_meta/wiki_index|Inside Code Block]]"));

    // Valid links MUST be universally clickable relative Markdown links
    assert!(updated.contains("- [Root Workspace Schema](../AGENTS.md)"));
    assert!(updated.contains("- [Self Mirror](AGENTS.md)"));
    assert!(updated.contains("- [Master Wiki Index](00_meta/wiki_index.md)"));
    assert!(updated.contains("- [Core Tenets](00_meta/wiki_index.md#core-tenets)"));
    assert!(updated.contains("- [Docs](../knowframe/README.md)"));
    assert!(updated.contains("- [NonRelative](../knowframe/README.md)"));
    assert!(updated.contains("- [MissingExt](00_meta/wiki_index.md)"));
    assert!(updated.contains("| Doc | [Table Link](00_meta/wiki_index.md) |"));

    // 2b. Second pass MUST be 100% idempotent (0 rewrites, 8 already clickable)
    let second_pass = normalize_workspace_links(root, Some("notes/AGENTS.md"), false).unwrap();
    assert_eq!(second_pass.total_rewrites, 0);
    assert_eq!(second_pass.total_already_clickable, 8);
    assert_eq!(second_pass.total_broken_skipped, 2);

    // Broken links MUST NOT be touched
    assert!(updated.contains("- [[does_not_exist|Broken Target]]"));
    assert!(updated.contains("- [[00_meta/wiki_index#missing-anchor|Broken Anchor]]"));

    // 3. Re-scanning after apply must report ZERO `non_clickable_link` issues on notes/AGENTS.md
    let scan = execute_scan(ScanOptions {
        workspace_root: root.to_path_buf(),
        hub: None,
        pretty: false,
    })
    .unwrap();

    let mut topology = GraphTopology::new();
    for n in &scan.nodes {
        topology.add_node(llm_wiki_core::model::NoteNode::new(&n.id, &n.file_path, &n.label));
    }
    for e in &scan.edges {
        topology.add_edge(e.to_note_edge());
    }

    let issues = extract_all_issues(&topology, &[], &[]);
    let non_clickable: Vec<_> = issues
        .iter()
        .filter(|i| i.category == "non_clickable_link")
        .collect();
    assert_eq!(non_clickable.len(), 0, "All valid links must now be clickable!");

    let score = calculate_quality_score(&topology, None, &[], &[]);
    assert!(score.integrity_score < 100.0, "The 2 intentionally broken links should still deduct points");
}
