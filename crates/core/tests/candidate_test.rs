use llm_wiki_core::candidate::*;
use llm_wiki_core::debt::*;
use llm_wiki_core::delta::updater::{FileDeltaItem, FileDeltaStatus};
use llm_wiki_core::delta::GitDelta;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NodeStatus, NoteEdge, NoteNode};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_candidate_overlay_routing_and_isolation() {
    let temp = tempdir().expect("create temp dir");
    let root = temp.path();

    let mut base_graph = GraphTopology::new();
    base_graph.add_node(NoteNode {
        id: "notes/existing.md".to_string(),
        file_path: root.join("notes/existing.md").to_string_lossy().to_string(),
        label: "Existing Doc".to_string(),
        tags: vec![],
        tokens: 100,
        in_degree: 0,
        out_degree: 0,
        betweenness: 0.0,
        hop_distance: 0,
        status: NodeStatus::Active,
        color: "#64748B".to_string(),
        broken_links: 0,
        fragments: vec![],
        freshness: None,
        code_dependencies: vec![],
    });
    base_graph.add_node(NoteNode {
        id: "notes/to_delete.md".to_string(),
        file_path: root
            .join("notes/to_delete.md")
            .to_string_lossy()
            .to_string(),
        label: "To Delete".to_string(),
        tags: vec![],
        tokens: 50,
        in_degree: 0,
        out_degree: 0,
        betweenness: 0.0,
        hop_distance: -1,
        status: NodeStatus::Orphan,
        color: "#6B7280".to_string(),
        broken_links: 0,
        fragments: vec![],
        freshness: None,
        code_dependencies: vec![],
    });

    // Create staged files on disk
    fs::create_dir_all(root.join("notes")).expect("create notes dir");
    fs::write(
        root.join("notes/new_staged.md"),
        "# New Staged Note\nContent with link to [[existing]].\n",
    )
    .expect("write staged file");

    let staged_delta = GitDelta {
        commit_sha: None,
        items: vec![
            FileDeltaItem {
                path: "notes/new_staged.md".to_string(),
                status: FileDeltaStatus::Added,
            },
            FileDeltaItem {
                path: "notes/to_delete.md".to_string(),
                status: FileDeltaStatus::Deleted,
            },
        ],
    };

    let overlay = CandidateOverlay::new(&base_graph, root, &staged_delta);

    // 1. New staged note is available in overlay
    assert!(overlay.node_exists("notes/new_staged.md"));
    let new_node = overlay
        .get_node("notes/new_staged.md")
        .expect("get new node");
    assert_eq!(new_node.label, "New Staged Note");

    // 2. Deleted note is shadowed/hidden in overlay
    assert!(!overlay.node_exists("notes/to_delete.md"));
    assert!(overlay.get_node("notes/to_delete.md").is_none());

    // 3. Untouched note falls through to base graph
    assert!(overlay.node_exists("notes/existing.md"));
    assert_eq!(
        overlay.get_node("notes/existing.md").unwrap().label,
        "Existing Doc"
    );

    // 4. Base graph is completely unmodified (State Boundary Invariant)
    assert_eq!(base_graph.nodes.len(), 2);
    assert!(base_graph.nodes.contains_key("notes/to_delete.md"));
    assert!(!base_graph.nodes.contains_key("notes/new_staged.md"));
}

#[test]
fn test_candidate_admission_decision_gate() {
    let temp = tempdir().expect("create temp dir");
    let root = temp.path();
    fs::create_dir_all(root.join("notes")).expect("create notes dir");

    let mut base_graph = GraphTopology::new();
    base_graph.add_node(NoteNode {
        id: "notes/hub.md".to_string(),
        file_path: root.join("notes/hub.md").to_string_lossy().to_string(),
        label: "Hub".to_string(),
        tags: vec![],
        tokens: 100,
        in_degree: 0,
        out_degree: 1,
        betweenness: 0.0,
        hop_distance: 0,
        status: NodeStatus::Active,
        color: "#64748B".to_string(),
        broken_links: 1,
        fragments: vec![],
        freshness: None,
        code_dependencies: vec![],
    });
    base_graph.add_edge(NoteEdge {
        id: "edge-1".to_string(),
        source: "notes/hub.md".to_string(),
        target: "notes/legacy_missing.md".to_string(),
        target_fragment: None,
        is_broken: true,
        raw_text: "[[notes/legacy_missing.md]]".to_string(),
        link_type: LinkType::Wikilink,
        anchor: None,
        line: Some(5),
        resolution_status: None,
    });

    // Baseline records existing legacy broken link
    let mut baseline = KnowledgeDebtBaseline::default();
    baseline.baseline_commit = "base_123".to_string();
    baseline.broken_links.push(BrokenLinkSignature {
        source_path: "notes/hub.md".to_string(),
        target_ref: "notes/legacy_missing.md".to_string(),
        line_number: 5,
    });
    baseline.total_debt = 1;

    // Scenario A: Staging a clean note that doesn't fix legacy debt and doesn't regress
    fs::write(
        root.join("notes/clean.md"),
        "# Clean Note\nThis note links to [[hub]].\n",
    )
    .expect("write clean");
    let clean_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "notes/clean.md".to_string(),
            status: FileDeltaStatus::Added,
        }],
    };
    let overlay_clean = CandidateOverlay::new(&base_graph, root, &clean_delta);
    let report_clean = evaluate_candidate(
        &overlay_clean,
        &baseline,
        vec![],
        vec![],
        AdmissionConfig::default(),
    );

    // Commit proceeds (PASS, exit code 0) even though legacy debt exists
    assert_eq!(report_clean.decision, AdmissionDecision::Pass);
    assert_eq!(report_clean.exit_code, 0);
    assert_eq!(report_clean.regressions.len(), 0);
    assert_eq!(report_clean.summary.legacy_debt, 1);

    // Scenario B: Staging a note with a new broken reference -> BLOCKED
    fs::write(
        root.join("notes/broken.md"),
        "# Broken Note\nThis note links to [[completely_nonexistent]].\n",
    )
    .expect("write broken");
    let broken_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "notes/broken.md".to_string(),
            status: FileDeltaStatus::Added,
        }],
    };
    let overlay_broken = CandidateOverlay::new(&base_graph, root, &broken_delta);
    let report_broken = evaluate_candidate(
        &overlay_broken,
        &baseline,
        vec![],
        vec![],
        AdmissionConfig::default(),
    );

    // Commit is BLOCKED (exit code 1)
    assert_eq!(report_broken.decision, AdmissionDecision::Block);
    assert_eq!(report_broken.exit_code, 1);
    assert_eq!(report_broken.regressions.len(), 1);
    assert_eq!(
        report_broken.regressions[0].category,
        RegressionCategory::BrokenReference
    );
    assert_eq!(
        report_broken.regressions[0].severity,
        RegressionSeverity::Blocker
    );

    // Scenario C: Staging a note that resolves legacy debt
    fs::write(
        root.join("notes/legacy_missing.md"),
        "# Legacy Target\nNow I exist!\n",
    )
    .expect("write resolved");
    let resolve_delta = GitDelta {
        commit_sha: None,
        items: vec![FileDeltaItem {
            path: "notes/legacy_missing.md".to_string(),
            status: FileDeltaStatus::Added,
        }],
    };
    let overlay_resolve = CandidateOverlay::new(&base_graph, root, &resolve_delta);
    let report_resolve = evaluate_candidate(
        &overlay_resolve,
        &baseline,
        vec![],
        vec![],
        AdmissionConfig::default(),
    );

    assert_eq!(report_resolve.decision, AdmissionDecision::Pass);
    assert_eq!(report_resolve.exit_code, 0);
    assert_eq!(report_resolve.resolved_findings.len(), 1);
    assert_eq!(report_resolve.summary.resolved_debt, 1);
}

#[test]
fn test_review_status_transitions_t012() {
    use llm_wiki_core::drift::review::{calculate_affected_documents, resolve_document_review};
    use llm_wiki_core::drift::ReverseImpactIndex;
    use llm_wiki_core::model::{CodeDependency, DependencySource, ReviewGatingConfig, ReviewStatus};

    let temp = tempdir().expect("create temp dir");
    let root = temp.path();

    fs::create_dir_all(root.join("notes")).unwrap();
    fs::create_dir_all(root.join("src")).unwrap();

    let doc_path = "notes/auth.md";
    let doc_content = r#"---
code_dependencies:
  - src/auth.rs
---
# Authentication
Cites auth module.
"#;
    fs::write(root.join(doc_path), doc_content).unwrap();
    fs::write(root.join("src/auth.rs"), "pub fn auth() {}\n").unwrap();

    let mut reverse_index = ReverseImpactIndex::new();
    reverse_index.insert_dependency(
        doc_path,
        &CodeDependency {
            code_path: "src/auth.rs".to_string(),
            source: DependencySource::Frontmatter,
            line_number: None,
        },
    );

    let code_changed_delta = vec![FileDeltaItem {
        path: "src/auth.rs".to_string(),
        status: FileDeltaStatus::Modified,
    }];

    // 1. Transition to BLOCKED: unreviewed code change under enforce_doc_review = true
    let blocking_config = ReviewGatingConfig {
        enforce_doc_review: true,
        block_on_legacy_debt: false,
    };
    let rep_blocked = calculate_affected_documents(
        root,
        &code_changed_delta,
        &reverse_index,
        Some(&blocking_config),
        None,
        None,
    );
    assert_eq!(rep_blocked.obligations.len(), 1);
    assert_eq!(rep_blocked.obligations[0].status, ReviewStatus::Blocked);
    assert_eq!(rep_blocked.blocked_count, 1);

    // 2. Transition to PENDING: unreviewed code change under enforce_doc_review = false
    let warn_config = ReviewGatingConfig {
        enforce_doc_review: false,
        block_on_legacy_debt: false,
    };
    let rep_pending = calculate_affected_documents(
        root,
        &code_changed_delta,
        &reverse_index,
        Some(&warn_config),
        None,
        None,
    );
    assert_eq!(rep_pending.obligations.len(), 1);
    assert_eq!(rep_pending.obligations[0].status, ReviewStatus::Pending);
    assert_eq!(rep_pending.pending_count, 1);

    // 3. Transition to UPDATED: doc is concurrently modified in same delta
    let doc_updated_delta = vec![
        FileDeltaItem {
            path: "src/auth.rs".to_string(),
            status: FileDeltaStatus::Modified,
        },
        FileDeltaItem {
            path: doc_path.to_string(),
            status: FileDeltaStatus::Modified,
        },
    ];
    let rep_updated = calculate_affected_documents(
        root,
        &doc_updated_delta,
        &reverse_index,
        Some(&blocking_config),
        None,
        None,
    );
    assert_eq!(rep_updated.obligations.len(), 1);
    assert_eq!(rep_updated.obligations[0].status, ReviewStatus::Updated);
    assert_eq!(rep_updated.blocked_count, 0);

    // 4. Transition to REVIEWED_UNCHANGED via recorded review receipt
    resolve_document_review(root, &[doc_path.to_string()], Some("HEAD")).expect("save receipt");
    let rep_reviewed = calculate_affected_documents(
        root,
        &code_changed_delta,
        &reverse_index,
        Some(&blocking_config),
        None,
        None,
    );
    assert_eq!(rep_reviewed.obligations.len(), 1);
    assert_eq!(rep_reviewed.obligations[0].status, ReviewStatus::ReviewedUnchanged);
    assert_eq!(rep_reviewed.reviewed_count, 1);
    assert_eq!(rep_reviewed.blocked_count, 0);

    // 5. Transition to REVIEWED_UNCHANGED via commit trailer
    let temp_fresh = tempdir().unwrap();
    let rep_trailer = calculate_affected_documents(
        temp_fresh.path(),
        &code_changed_delta,
        &reverse_index,
        Some(&blocking_config),
        None,
        Some("fix(auth): update login flow\n\nReviewed-Docs: notes/auth.md"),
    );
    assert_eq!(rep_trailer.obligations.len(), 1);
    assert_eq!(rep_trailer.obligations[0].status, ReviewStatus::ReviewedUnchanged);
    assert_eq!(rep_trailer.reviewed_count, 1);
}

