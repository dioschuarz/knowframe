use llm_wiki_core::debt::*;
use tempfile::tempdir;

#[test]
fn test_baseline_save_and_load_lifecycle() {
    let temp = tempdir().expect("create temp dir");
    let root = temp.path();

    // Baseline initially does not exist
    assert!(load_baseline(root).expect("check baseline").is_none());

    // Create baseline
    let mut baseline = KnowledgeDebtBaseline::default();
    baseline.baseline_commit = "deadbeef123".to_string();
    baseline.broken_links.push(BrokenLinkSignature {
        source_path: "notes/a.md".to_string(),
        target_ref: "target.md".to_string(),
        line_number: 15,
    });
    baseline.orphan_nodes.insert("notes/orphan.md".to_string());
    baseline
        .collisions
        .push(CollisionSignature::new("notes/y.md", "notes/x.md"));
    baseline.technical_drifts.push(DriftSignature {
        doc_path: "notes/arch.md".to_string(),
        referenced_code_path: "src/auth.ts".to_string(),
        symbol: "AuthService".to_string(),
    });
    baseline.total_debt = 4;

    save_baseline(root, &baseline).expect("save baseline");

    // Load and assert identical
    let loaded = load_baseline(root)
        .expect("load baseline")
        .expect("baseline exists");
    assert_eq!(loaded.baseline_commit, "deadbeef123");
    assert_eq!(loaded.total_debt, 4);
    assert_eq!(loaded.broken_links.len(), 1);
    assert_eq!(loaded.orphan_nodes.len(), 1);
    assert_eq!(loaded.collisions.len(), 1);
    assert_eq!(loaded.technical_drifts.len(), 1);
    assert_eq!(loaded.collisions[0].doc_a, "notes/x.md");
    assert_eq!(loaded.collisions[0].doc_b, "notes/y.md");
}

#[test]
fn test_differential_evaluation_new_regression_detection() {
    let mut baseline = KnowledgeDebtBaseline::default();
    baseline.baseline_commit = "base1".to_string();
    baseline.broken_links.push(BrokenLinkSignature {
        source_path: "notes/legacy.md".to_string(),
        target_ref: "missing.md".to_string(),
        line_number: 20,
    });
    baseline.total_debt = 1;

    // Staged commit introduces a NEW broken link
    let mut candidate = CandidateFindings::default();
    candidate.broken_links.push(BrokenLinkSignature {
        source_path: "notes/legacy.md".to_string(),
        target_ref: "missing.md".to_string(),
        line_number: 20,
    });
    candidate.broken_links.push(BrokenLinkSignature {
        source_path: "notes/new_feature.md".to_string(),
        target_ref: "non_existent.md".to_string(),
        line_number: 5,
    });

    let diff = diff_against_baseline(&candidate, &baseline);
    assert!(diff.has_regressions());
    assert_eq!(diff.new_broken_links.len(), 1);
    assert_eq!(diff.new_broken_links[0].source_path, "notes/new_feature.md");
    assert_eq!(diff.resolved_broken_links.len(), 0);
    assert_eq!(diff.summary.legacy_debt, 1);
    assert_eq!(diff.summary.new_debt, 1);
    assert_eq!(diff.summary.regressions, 1);
    assert_eq!(diff.summary.total_debt, 2);
}

#[test]
fn test_differential_evaluation_debt_ratchet_on_resolution() {
    let mut baseline = KnowledgeDebtBaseline::default();
    baseline.baseline_commit = "base1".to_string();
    baseline.broken_links.push(BrokenLinkSignature {
        source_path: "notes/legacy.md".to_string(),
        target_ref: "missing.md".to_string(),
        line_number: 20,
    });
    baseline
        .orphan_nodes
        .insert("notes/old_orphan.md".to_string());
    baseline.total_debt = 2;

    // Developer fixes notes/legacy.md link and connects notes/old_orphan.md
    let candidate = CandidateFindings::default();

    let diff = diff_against_baseline(&candidate, &baseline);
    assert!(!diff.has_regressions());
    assert_eq!(diff.new_broken_links.len(), 0);
    assert_eq!(diff.resolved_broken_links.len(), 1);
    assert_eq!(diff.resolved_orphans.len(), 1);
    assert_eq!(diff.summary.resolved_debt, 2);
    assert_eq!(diff.summary.new_debt, 0);
    assert_eq!(diff.summary.regressions, 0);
    assert_eq!(diff.summary.total_debt, 0);

    // Post-commit ratchets baseline down
    ratchet_baseline(&mut baseline, &diff, "commit_fixed_all");
    assert_eq!(baseline.baseline_commit, "commit_fixed_all");
    assert_eq!(baseline.total_debt, 0);
    assert!(baseline.broken_links.is_empty());
    assert!(baseline.orphan_nodes.is_empty());
}

#[test]
fn test_signature_hashing_uniqueness() {
    let sig1 = BrokenLinkSignature {
        source_path: "a.md".to_string(),
        target_ref: "b.md".to_string(),
        line_number: 1,
    };
    let sig2 = BrokenLinkSignature {
        source_path: "a.md".to_string(),
        target_ref: "b.md".to_string(),
        line_number: 2,
    };
    assert_ne!(hash_broken_link(&sig1), hash_broken_link(&sig2));

    let col1 = CollisionSignature::new("doc1.md", "doc2.md");
    let col2 = CollisionSignature::new("doc2.md", "doc1.md");
    assert_eq!(hash_collision(&col1), hash_collision(&col2));

    let orphan1 = hash_orphan("notes/a.md");
    let orphan2 = hash_orphan("notes/b.md");
    assert_ne!(orphan1, orphan2);
}

#[test]
fn test_initial_baseline_capture_on_brownfield() {
    use llm_wiki_core::graph::builder::GraphTopology;
    use llm_wiki_core::model::{LinkType, NodeStatus, NoteEdge, NoteNode};

    let mut topology = GraphTopology::new();

    // Add normal connected node
    topology.add_node(NoteNode {
        id: "notes/hub.md".to_string(),
        file_path: "/workspace/notes/hub.md".to_string(),
        label: "Hub".to_string(),
        tags: vec![],
        tokens: 100,
        in_degree: 1,
        out_degree: 1,
        betweenness: 0.5,
        hop_distance: 0,
        status: NodeStatus::Active,
        color: "#64748B".to_string(),
        broken_links: 1,
        fragments: vec![],
        freshness: None,
        code_dependencies: vec![],
    });

    // Add orphan node
    topology.add_node(NoteNode {
        id: "notes/orphan.md".to_string(),
        file_path: "/workspace/notes/orphan.md".to_string(),
        label: "Orphan".to_string(),
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

    // Add broken edge originating from hub
    topology.add_edge(NoteEdge {
        id: "edge-1".to_string(),
        source: "notes/hub.md".to_string(),
        target: "notes/missing.md".to_string(),
        target_fragment: None,
        is_broken: true,
        link_type: LinkType::Wikilink,
        raw_text: "[[notes/missing.md]]".to_string(),
        anchor: None,
        line: Some(12),
        resolution_status: None,
        raw_target: Some("notes/missing.md".to_string()),
        is_clickable: false,
        non_clickable_reason: None,
        clickable_target: None,
    });

    let baseline = capture_initial_baseline(&topology, "init_sha", vec![], vec![]);
    assert_eq!(baseline.baseline_commit, "init_sha");
    assert_eq!(baseline.broken_links.len(), 1);
    assert_eq!(baseline.broken_links[0].source_path, "notes/hub.md");
    assert_eq!(baseline.broken_links[0].target_ref, "notes/missing.md");
    assert_eq!(baseline.broken_links[0].line_number, 12);
    assert_eq!(baseline.orphan_nodes.len(), 1);
    assert!(baseline.orphan_nodes.contains("notes/orphan.md"));
    assert_eq!(baseline.total_debt, 2);
}

#[test]
fn test_brownfield_zero_regression_guarantee() {
    use llm_wiki_core::candidate::evaluator::{evaluate_candidate, AdmissionConfig};
    use llm_wiki_core::candidate::overlay::CandidateOverlay;
    use llm_wiki_core::candidate::AdmissionDecision;
    use llm_wiki_core::delta::updater::{FileDeltaItem, FileDeltaStatus};
    use llm_wiki_core::delta::GitDelta;
    use llm_wiki_core::graph::builder::GraphTopology;
    use llm_wiki_core::model::{LinkType, NodeStatus, NoteEdge, NoteNode};

    let mut topology = GraphTopology::new();

    // 50 pre-existing broken links across legacy notes
    let mut baseline = KnowledgeDebtBaseline::default();
    baseline.baseline_commit = "legacy_sha_001".to_string();

    let mut hub = NoteNode::new("hub.md", "/vault/hub.md", "Hub");
    hub.status = NodeStatus::Active;
    topology.add_node(hub);

    for i in 0..50 {
        let note_id = format!("note_{:03}.md", i);
        let mut node = NoteNode::new(
            &note_id,
            format!("/vault/{}", note_id),
            format!("Legacy Note {}", i),
        );
        node.status = NodeStatus::Active;
        topology.add_node(node);

        // Valid edge connecting from hub
        let valid_edge = NoteEdge::new(
            "hub.md",
            &note_id,
            LinkType::Wikilink,
            format!("[[{}]]", note_id),
            None,
            Some(1),
        );
        topology.add_edge(valid_edge);

        let target_missing = format!("missing_{:03}.md", i);
        let mut edge = NoteEdge::new(
            &note_id,
            &target_missing,
            LinkType::Wikilink,
            format!("[[{}]]", target_missing),
            None,
            Some(10),
        );
        edge.is_broken = true;
        topology.add_edge(edge);
    }

    topology.compute_degrees();

    let baseline = capture_initial_baseline(&topology, "legacy_sha_001", vec![], vec![]);
    assert_eq!(baseline.total_debt, 50);
    assert_eq!(baseline.broken_links.len(), 50);

    // Unrelated note edit (e.g. README.md or clean_feature.md)
    let mut clean_node = NoteNode::new("feature.md", "/vault/feature.md", "Feature Documentation");
    clean_node.status = NodeStatus::Active;
    topology.add_node(clean_node);

    let temp = tempdir().unwrap();
    let root = temp.path();

    // Write valid feature.md with reference to hub.md so it is connected
    std::fs::write(
        root.join("feature.md"),
        "# Feature Documentation\n\nReferencing [[hub]].\n",
    )
    .unwrap();

    let staged_delta = GitDelta {
        commit_sha: Some("new_commit_sha".to_string()),
        items: vec![FileDeltaItem {
            path: "feature.md".to_string(),
            status: FileDeltaStatus::Modified,
        }],
    };

    let overlay = CandidateOverlay::new(&topology, root, &staged_delta);
    let report = evaluate_candidate(
        &overlay,
        &baseline,
        vec![],
        vec![],
        AdmissionConfig::default(),
    );

    assert_eq!(report.decision, AdmissionDecision::Pass);
    assert_eq!(report.exit_code, 0);
    assert_eq!(report.regressions.len(), 0);
    assert_eq!(report.summary.legacy_debt, 50);
    assert_eq!(report.summary.new_debt, 0);
    assert_eq!(report.summary.regressions, 0);
}
