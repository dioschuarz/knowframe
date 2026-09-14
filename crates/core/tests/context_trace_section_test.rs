use std::fs;
use tempfile::TempDir;
use llm_wiki_core::fragment::FragmentMetadata;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{NoteEdge, NoteNode};
use llm_wiki_core::trace::beam::{
    calculate_context_reduction, compute_candidate_score, refine_section_target,
    trace_context_section,
};
use llm_wiki_core::cli::trace_cmd::execute_trace;

#[test]
fn test_context_reduction_calculation() {
    // 20k doc tokens, 2k fragment tokens -> 90.0% context reduction
    let r1 = calculate_context_reduction(20_000, Some(2_000));
    assert_eq!(r1, 90.0);
    assert!(r1 >= 80.0);

    // 50k doc tokens, 5k fragment tokens -> 90.0% context reduction
    let r2 = calculate_context_reduction(50_000, Some(5_000));
    assert_eq!(r2, 90.0);
    assert!(r2 >= 80.0);

    // No fragment targeted (None) -> 0.0%
    let r3 = calculate_context_reduction(20_000, None);
    assert_eq!(r3, 0.0);

    // Fragment tokens >= doc tokens -> 0.0%
    let r4 = calculate_context_reduction(5_000, Some(5_000));
    assert_eq!(r4, 0.0);

    // Zero doc tokens -> 0.0%
    let r5 = calculate_context_reduction(0, Some(500));
    assert_eq!(r5, 0.0);
}

#[test]
fn test_candidate_scoring_and_section_refinement() {
    let mut node = NoteNode::new("arch.md".to_string(), "arch.md".to_string(), "System Architecture".to_string());
    node.tokens = 25_000;
    node.betweenness = 0.4;
    node.fragments = vec![
        FragmentMetadata {
            anchor: "database-schema".to_string(),
            heading: "Database Schema".to_string(),
            start_line: 1,
            end_line: 100,
            tokens: 15_000,
            inbound: 2,
            outbound: 0,
        },
        FragmentMetadata {
            anchor: "oauth-authentication".to_string(),
            heading: "OAuth Authentication".to_string(),
            start_line: 101,
            end_line: 150,
            tokens: 2_500,
            inbound: 5,
            outbound: 1,
        },
    ];

    let query_tokens = vec!["oauth".to_string(), "authentication".to_string()];
    let (score, reasons) = compute_candidate_score(&node, &query_tokens, 1);
    assert!(score > 0.4);
    assert!(!reasons.is_empty());

    let (frag_anchor, frag_tokens, reduction, refine_reasons) = refine_section_target(&node, &query_tokens);
    assert_eq!(frag_anchor, Some("oauth-authentication".to_string()));
    assert_eq!(frag_tokens, Some(2_500));
    // Reduction = (25000 - 2500) / 25000 = 22500 / 25000 = 90.0%
    assert_eq!(reduction, 90.0);
    assert!(reduction >= 80.0);
    assert!(!refine_reasons.is_empty());
}

#[test]
fn test_beam_search_traversal_and_section_routing() {
    let mut topology = GraphTopology::new();

    // Node 1: AGENTS.md (Entrypoint)
    let mut agents = NoteNode::new("AGENTS.md".to_string(), "AGENTS.md".to_string(), "Agents Operating System".to_string());
    agents.tokens = 1_200;
    agents.betweenness = 0.8;
    topology.add_node(agents);

    // Node 2: architecture.md (Hop 1) with large doc (30k tokens) and small section (3k tokens)
    let mut arch = NoteNode::new("architecture.md".to_string(), "architecture.md".to_string(), "Architecture Overview".to_string());
    arch.tokens = 30_000;
    arch.betweenness = 0.5;
    arch.fragments = vec![
        FragmentMetadata {
            anchor: "overview".to_string(),
            heading: "System Overview".to_string(),
            start_line: 1,
            end_line: 100,
            tokens: 27_000,
            inbound: 1,
            outbound: 0,
        },
        FragmentMetadata {
            anchor: "jwt-token-service".to_string(),
            heading: "JWT Token Service".to_string(),
            start_line: 101,
            end_line: 140,
            tokens: 3_000,
            inbound: 6,
            outbound: 1,
        },
    ];
    topology.add_node(arch);

    // Edge from AGENTS.md -> architecture.md
    topology.add_edge(NoteEdge::new(
        "AGENTS.md".to_string(),
        "architecture.md".to_string(),
        llm_wiki_core::model::LinkType::Markdown,
        String::new(),
        None,
        Some(10),
    ));

    // Execute section trace with query "jwt token"
    let res = trace_context_section(&topology, "jwt token", Some("AGENTS.md"));
    assert!(res.is_some());
    let trace = res.unwrap();

    assert_eq!(trace.entrypoint, "AGENTS.md");
    assert_eq!(trace.target_document, "architecture.md");
    assert_eq!(trace.target_fragment, Some("jwt-token-service".to_string()));
    assert_eq!(trace.depth, 1);
    assert_eq!(trace.document_raw_tokens, 30_000);
    assert_eq!(trace.fragment_raw_tokens, Some(3_000));
    // Reduction = (30000 - 3000) / 30000 = 90.0% >= 80%
    assert_eq!(trace.context_reduction_percent, 90.0);
    assert!(trace.context_reduction_percent >= 80.0);
    assert!(!trace.relevance_reasons.is_empty());
}

#[test]
fn test_cli_execute_trace_section() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    fs::write(root.join("AGENTS.md"), "# AGENTS\n\nEntrypoint to [docs/security.md](docs/security.md).\n").unwrap();
    fs::create_dir_all(root.join("docs")).unwrap();
    let sec_content = r#"# Security Architecture

General intro...

## OAuth Token Management

Section for managing tokens.
"#;
    fs::write(root.join("docs/security.md"), sec_content).unwrap();

    let res = execute_trace(root, "token", Some("AGENTS.md"), false);
    assert!(res.is_ok());

    let res_json = execute_trace(root, "token", Some("AGENTS.md"), true);
    assert!(res_json.is_ok());
}
