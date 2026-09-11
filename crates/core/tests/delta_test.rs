use llm_wiki_core::delta::updater::{apply_file_delta, FileDeltaItem, FileDeltaStatus};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};
use tempfile::tempdir;

#[test]
fn test_incremental_delta_add_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create initial topology with file_a.md
    let file_a_path = root.join("file_a.md");
    std::fs::write(&file_a_path, "# File A\nIntro content").unwrap();

    let mut topology = GraphTopology::new();
    let mut node_a = NoteNode::new(
        "file_a.md",
        file_a_path.to_string_lossy().to_string(),
        "File A".to_string(),
    );
    node_a.tokens = 10;
    topology.add_node(node_a);
    topology.resolve_links();

    assert_eq!(topology.nodes.len(), 1);

    // Now write file_b.md that links to file_a.md
    let file_b_path = root.join("file_b.md");
    std::fs::write(&file_b_path, "# File B\nLink to [[file_a.md]]").unwrap();

    let delta = vec![FileDeltaItem {
        path: "file_b.md".to_string(),
        status: FileDeltaStatus::Added,
    }];

    apply_file_delta(&mut topology, root, &delta, Some("file_a.md"));

    // Verify file_b.md added
    assert_eq!(topology.nodes.len(), 2);
    assert!(topology.nodes.contains_key("file_b.md"));
    let node_b = topology.get_node("file_b.md").unwrap();
    assert_eq!(node_b.label, "File B");
    assert!(node_b.tokens > 0);

    // Verify edge from file_b to file_a exists and is not broken
    let b_to_a = topology
        .edges
        .iter()
        .find(|e| e.source == "file_b.md" && e.target == "file_a.md");
    assert!(b_to_a.is_some());
    assert!(!b_to_a.unwrap().is_broken);
}

#[test]
fn test_incremental_delta_modify_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let file_a_path = root.join("file_a.md");
    std::fs::write(&file_a_path, "# File A\nOriginal content with no links").unwrap();

    let mut topology = GraphTopology::new();
    let mut node_a = NoteNode::new(
        "file_a.md",
        file_a_path.to_string_lossy().to_string(),
        "File A".to_string(),
    );
    node_a.tokens = 8;
    topology.add_node(node_a);
    topology.resolve_links();

    assert_eq!(topology.edges.len(), 0);

    // Modify file_a.md to add broken link to nonexistent note
    std::fs::write(
        &file_a_path,
        "# File A\nModified content with [[missing_note.md]]",
    )
    .unwrap();

    let delta = vec![FileDeltaItem {
        path: "file_a.md".to_string(),
        status: FileDeltaStatus::Modified,
    }];

    apply_file_delta(&mut topology, root, &delta, Some("file_a.md"));

    assert_eq!(topology.nodes.len(), 1);
    let edges_from_a: Vec<_> = topology
        .edges
        .iter()
        .filter(|e| e.source == "file_a.md")
        .collect();
    assert_eq!(edges_from_a.len(), 1);
    assert!(edges_from_a[0].is_broken);
}

#[test]
fn test_incremental_delta_delete_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let file_a_path = root.join("file_a.md");
    let file_b_path = root.join("file_b.md");
    std::fs::write(&file_a_path, "# File A\nLinks to [[file_b.md]]").unwrap();
    std::fs::write(&file_b_path, "# File B\nTarget note").unwrap();

    let mut topology = GraphTopology::new();
    let mut node_a = NoteNode::new(
        "file_a.md",
        file_a_path.to_string_lossy().to_string(),
        "File A".to_string(),
    );
    let mut node_b = NoteNode::new(
        "file_b.md",
        file_b_path.to_string_lossy().to_string(),
        "File B".to_string(),
    );
    node_a.tokens = 10;
    node_b.tokens = 10;
    topology.add_node(node_a);
    topology.add_node(node_b);
    topology.add_edge(NoteEdge::new(
        "file_a.md",
        "file_b.md",
        LinkType::Wikilink,
        "[[file_b.md]]".to_string(),
        None,
        Some(2),
    ));
    topology.resolve_links();

    assert!(!topology.edges[0].is_broken);

    // Delete file_b.md
    std::fs::remove_file(&file_b_path).unwrap();

    let delta = vec![FileDeltaItem {
        path: "file_b.md".to_string(),
        status: FileDeltaStatus::Deleted,
    }];

    apply_file_delta(&mut topology, root, &delta, Some("file_a.md"));

    // Verify file_b.md node is removed
    assert_eq!(topology.nodes.len(), 1);
    assert!(!topology.nodes.contains_key("file_b.md"));

    // Verify edge from file_a to file_b is now marked as broken
    let edge = topology
        .edges
        .iter()
        .find(|e| e.source == "file_a.md" && e.target == "file_b.md");
    assert!(edge.is_some());
    assert!(edge.unwrap().is_broken);
}
