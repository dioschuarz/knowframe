use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NodeStatus, NoteEdge, NoteNode};

#[test]
fn test_degrees_and_betweenness_centrality() {
    let mut topology = GraphTopology::new();

    let n1 = NoteNode::new("hub.md", "/repo/hub.md", "Hub");
    let n2 = NoteNode::new("bridge.md", "/repo/bridge.md", "Bridge");
    let n3 = NoteNode::new("leaf.md", "/repo/leaf.md", "Leaf");
    let orphan = NoteNode::new("orphan.md", "/repo/orphan.md", "Orphan");

    topology.add_node(n1);
    topology.add_node(n2);
    topology.add_node(n3);
    topology.add_node(orphan);

    // hub -> bridge -> leaf
    topology.add_edge(NoteEdge::new("hub.md", "bridge.md", LinkType::Markdown, "b", None, Some(1)));
    topology.add_edge(NoteEdge::new("bridge.md", "leaf.md", LinkType::Markdown, "l", None, Some(1)));

    topology.compute_degrees();
    topology.compute_betweenness();

    assert_eq!(topology.get_node("hub.md").unwrap().out_degree, 1);
    assert_eq!(topology.get_node("hub.md").unwrap().in_degree, 0);

    assert_eq!(topology.get_node("bridge.md").unwrap().out_degree, 1);
    assert_eq!(topology.get_node("bridge.md").unwrap().in_degree, 1);

    assert_eq!(topology.get_node("leaf.md").unwrap().out_degree, 0);
    assert_eq!(topology.get_node("leaf.md").unwrap().in_degree, 1);

    assert_eq!(topology.get_node("orphan.md").unwrap().in_degree, 0);
    assert_eq!(topology.get_node("orphan.md").unwrap().out_degree, 0);

    // Bridge connects hub to leaf, so it has positive betweenness centrality
    assert!(topology.get_node("bridge.md").unwrap().betweenness > 0.0);
}

#[test]
fn test_circular_link_bfs_hop_distance() {
    let mut topology = GraphTopology::new();

    topology.add_node(NoteNode::new("a.md", "/repo/a.md", "A"));
    topology.add_node(NoteNode::new("b.md", "/repo/b.md", "B"));
    topology.add_node(NoteNode::new("c.md", "/repo/c.md", "C"));
    topology.add_node(NoteNode::new("orphan.md", "/repo/orphan.md", "Orphan"));

    // Cycle: a -> b -> a
    topology.add_edge(NoteEdge::new("a.md", "b.md", LinkType::Markdown, "b", None, None));
    topology.add_edge(NoteEdge::new("b.md", "a.md", LinkType::Markdown, "a", None, None));
    // Path: b -> c
    topology.add_edge(NoteEdge::new("b.md", "c.md", LinkType::Markdown, "c", None, None));

    // Designate a.md as Hub
    topology.compute_hops("a.md");

    let a = topology.get_node("a.md").unwrap();
    let b = topology.get_node("b.md").unwrap();
    let c = topology.get_node("c.md").unwrap();
    let orphan = topology.get_node("orphan.md").unwrap();

    assert_eq!(a.hop_distance, 0);
    assert_eq!(a.status, NodeStatus::Active);

    assert_eq!(b.hop_distance, 1);
    assert_eq!(b.status, NodeStatus::ConnectedInactive);

    assert_eq!(c.hop_distance, 2);
    assert_eq!(c.status, NodeStatus::ConnectedInactive);

    assert_eq!(orphan.hop_distance, -1);
    assert_eq!(orphan.status, NodeStatus::Orphan);
}
