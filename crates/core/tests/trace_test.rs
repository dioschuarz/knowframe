use std::fs;
use tempfile::tempdir;

use llm_wiki_core::cli::trace_cmd::execute_trace;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{NoteEdge, NoteNode};
use llm_wiki_core::trace::traversal::{
    execute_context_trace, find_shortest_path, select_query_target,
};

use llm_wiki_core::model::{LinkResolutionStatus, LinkType, NodeStatus};

fn make_node(id: &str, label: &str, tags: Vec<String>, tokens: u32) -> NoteNode {
    let mut node = NoteNode::new(id, format!("{}.md", id), label);
    node.tags = tags;
    node.tokens = tokens;
    node.status = NodeStatus::Active;
    node
}

fn make_edge(source: &str, target: &str, is_broken: bool) -> NoteEdge {
    let mut edge = NoteEdge::new(
        source,
        target,
        LinkType::Wikilink,
        format!("[[{}]]", target),
        None,
        Some(1),
    );
    edge.is_broken = is_broken;
    edge.resolution_status = if is_broken {
        Some(LinkResolutionStatus::BrokenWikilink)
    } else {
        Some(LinkResolutionStatus::Resolved)
    };
    edge
}

#[test]
fn test_context_trace_target_selection() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_node("AGENTS.md", "Agent Entrypoint", vec![], 200));
    topology.add_node(make_node(
        "auth.md",
        "Authentication Service",
        vec!["#security".to_string()],
        500,
    ));
    topology.add_node(make_node(
        "billing.md",
        "Payment Processing",
        vec!["#finance".to_string()],
        400,
    ));

    // Query matching title
    let target = select_query_target(&topology, "authentication service", "AGENTS.md");
    assert!(target.is_some());
    let (target_id, score) = target.unwrap();
    assert_eq!(target_id, "auth.md");
    assert!(score > 0.0);

    // Query matching tag
    let target_tag = select_query_target(&topology, "security review", "AGENTS.md");
    assert!(target_tag.is_some());
    assert_eq!(target_tag.unwrap().0, "auth.md");

    // Empty query returns None
    let target_empty = select_query_target(&topology, "", "AGENTS.md");
    assert!(target_empty.is_none());
}

#[test]
fn test_context_trace_dijkstra_traversal() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_node("AGENTS.md", "Entrypoint", vec![], 100));
    topology.add_node(make_node("hub.md", "Architecture Hub", vec![], 300));
    topology.add_node(make_node("db.md", "Database Layer", vec![], 400));
    topology.add_node(make_node("orphan.md", "Isolated Note", vec![], 200));

    // Directed graph: AGENTS.md -> hub.md -> db.md
    topology.add_edge(make_edge("AGENTS.md", "hub.md", false));
    topology.add_edge(make_edge("hub.md", "db.md", false));
    // Broken shortcut edge: AGENTS.md -> db.md (broken)
    topology.add_edge(make_edge("AGENTS.md", "db.md", true));

    let path = find_shortest_path(&topology, "AGENTS.md", "db.md");
    assert!(path.is_some());
    let p = path.unwrap();
    assert_eq!(p, vec!["AGENTS.md", "hub.md", "db.md"]);

    // Unreachable orphan returns None
    let orphan_path = find_shortest_path(&topology, "AGENTS.md", "orphan.md");
    assert!(orphan_path.is_none());
}

#[test]
fn test_context_trace_token_accumulation() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_node("AGENTS.md", "Entrypoint", vec![], 150));
    topology.add_node(make_node("services.md", "Services Overview", vec![], 450));
    topology.add_node(make_node("gateway.md", "API Gateway", vec![], 300));

    topology.add_edge(make_edge("AGENTS.md", "services.md", false));
    topology.add_edge(make_edge("services.md", "gateway.md", false));

    let result = execute_context_trace(&topology, "API Gateway", Some("AGENTS.md"));
    assert_eq!(result.target_node, "gateway.md");
    assert_eq!(result.depth, 2);
    // 150 + 450 + 300 = 900 tokens
    assert_eq!(result.cumulative_bpe_tokens, 900);
    assert_eq!(result.relevant_tokens, 300);
}

#[test]
fn test_context_trace_path_quality_score() {
    let mut topology = GraphTopology::new();
    topology.add_node(make_node("AGENTS.md", "Entrypoint", vec![], 100));
    topology.add_node(make_node("intermediate.md", "Random Intermediate", vec![], 200));
    topology.add_node(make_node("cache.md", "Redis Cache", vec![], 300));

    topology.add_edge(make_edge("AGENTS.md", "intermediate.md", false));
    topology.add_edge(make_edge("intermediate.md", "cache.md", false));

    let result = execute_context_trace(&topology, "Redis Cache", Some("AGENTS.md"));
    assert_eq!(result.depth, 2);
    // Depth > 1 deducts 4% (depth - 1 = 1 hop -> 4%)
    // intermediate.md does not contain "Redis" or "Cache" in title -> weak node -> 5% deduction
    // Expected score: 100 - 4 - 5 = 91.0%
    assert!((result.path_quality_score - 91.0).abs() < 0.1);
    assert_eq!(result.weak_intermediate_nodes, vec!["intermediate.md"]);
}

#[test]
fn test_cli_trace_command() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::write(
        root.join("AGENTS.md"),
        "# AGENTS\n\nEntrypoint referencing [[architecture]].\n",
    )
    .unwrap();
    fs::write(
        root.join("architecture.md"),
        "# Architecture\n\nOverview referencing [[auth]].\n",
    )
    .unwrap();
    fs::write(
        root.join("auth.md"),
        "# Authentication Service\n\nSecure auth service.\n",
    )
    .unwrap();

    // Text execution
    let text_res = execute_trace(root, "Authentication", Some("AGENTS.md"), false);
    assert!(text_res.is_ok());

    // JSON execution
    let json_res = execute_trace(root, "Authentication", Some("AGENTS.md"), true);
    assert!(json_res.is_ok());
}
