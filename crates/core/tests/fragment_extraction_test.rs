use llm_wiki_core::fragment::extractor::{
    extract_fragments_from_document, populate_fragment_inbounds,
};
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkResolutionStatus, LinkType, NoteEdge, NoteNode};
use std::collections::HashMap;

#[test]
fn test_ast_heading_extraction_and_line_ranges() {
    let content = r#"# Main Architecture

Introductory text in main architecture.

## Authentication
Authentication details here.
Line 2 of auth.

### OAuth2 Subflow
OAuth token exchange details.

## Database Storage
Storage details here.
Final line.
"#;

    let fragments = extract_fragments_from_document(content, &[]);
    assert_eq!(fragments.len(), 4);

    // Fragment 0: H1 Main Architecture
    assert_eq!(fragments[0].heading, "Main Architecture");
    assert_eq!(fragments[0].anchor, "main-architecture");
    assert_eq!(fragments[0].start_line, 1);
    // H1 is followed only by H2/H3 (lower ranks), so it spans to EOF
    assert_eq!(fragments[0].end_line, 14);

    // Fragment 1: H2 Authentication
    assert_eq!(fragments[1].heading, "Authentication");
    assert_eq!(fragments[1].anchor, "authentication");
    assert_eq!(fragments[1].start_line, 5);
    // H2 ends right before the next H2 ("Database Storage" at line 12)
    assert_eq!(fragments[1].end_line, 11);

    // Fragment 2: H3 OAuth2 Subflow
    assert_eq!(fragments[2].heading, "OAuth2 Subflow");
    assert_eq!(fragments[2].anchor, "oauth2-subflow");
    assert_eq!(fragments[2].start_line, 9);
    assert_eq!(fragments[2].end_line, 11);

    // Fragment 3: H2 Database Storage
    assert_eq!(fragments[3].heading, "Database Storage");
    assert_eq!(fragments[3].anchor, "database-storage");
    assert_eq!(fragments[3].start_line, 12);
    assert_eq!(fragments[3].end_line, 14);
}

#[test]
fn test_fragment_bpe_token_counting() {
    let content = r#"## Section One
This is a short section with some plain words.

## Section Two
This is another section with more detailed content to ensure raw integer BPE token counting works precisely.
"#;

    let fragments = extract_fragments_from_document(content, &[]);
    assert_eq!(fragments.len(), 2);
    assert!(fragments[0].tokens > 0);
    assert!(fragments[1].tokens > 0);
    assert!(fragments[1].tokens > fragments[0].tokens);
}

#[test]
fn test_fragment_reference_counts() {
    let content = r#"## Section Alpha
Check out [Beta](./beta.md) for details.

## Section Beta
No links here.
"#;

    let edge = NoteEdge {
        id: "alpha->beta".to_string(),
        source: "alpha.md".to_string(),
        target: "beta.md".to_string(),
        target_fragment: None,
        link_type: LinkType::Markdown,
        is_broken: false,
        raw_text: "[Beta](./beta.md)".to_string(),
        anchor: None,
        line: Some(2), // Originates in Section Alpha (lines 1..=3)
        resolution_status: Some(LinkResolutionStatus::Resolved),
        raw_target: Some("./beta.md".to_string()),
        is_clickable: true,
        non_clickable_reason: None,
        clickable_target: Some("beta.md".to_string()),
    };

    let fragments = extract_fragments_from_document(content, &[edge]);
    assert_eq!(fragments[0].outbound, 1);
    assert_eq!(fragments[1].outbound, 0);

    // Test inbound population
    let mut node_b = NoteNode::new("beta.md", "/path/beta.md", "Beta");
    node_b.fragments = vec![llm_wiki_core::fragment::FragmentMetadata {
        anchor: "target-section".to_string(),
        heading: "Target Section".to_string(),
        start_line: 1,
        end_line: 10,
        tokens: 50,
        inbound: 0,
        outbound: 0,
    }];

    let mut nodes = HashMap::new();
    nodes.insert("beta.md".to_string(), node_b);

    let inbound_edge = NoteEdge {
        id: "caller->beta".to_string(),
        source: "caller.md".to_string(),
        target: "beta.md".to_string(),
        target_fragment: Some("target-section".to_string()),
        link_type: LinkType::Markdown,
        is_broken: false,
        raw_text: "[Target](./beta.md#target-section)".to_string(),
        anchor: Some("#target-section".to_string()),
        line: Some(1),
        resolution_status: Some(LinkResolutionStatus::Resolved),
        raw_target: Some("./beta.md".to_string()),
        is_clickable: true,
        non_clickable_reason: None,
        clickable_target: Some("beta.md#target-section".to_string()),
    };

    populate_fragment_inbounds(&mut nodes, &[inbound_edge]);
    assert_eq!(nodes.get("beta.md").unwrap().fragments[0].inbound, 1);
}

#[test]
fn test_fragment_link_resolution_and_dead_anchor_reporting() {
    let mut topology = GraphTopology::new();

    let mut target_node = NoteNode::new("target.md", "/workspace/target.md", "Target Doc");
    target_node.fragments = vec![llm_wiki_core::fragment::FragmentMetadata {
        anchor: "auth-flow".to_string(),
        heading: "Auth Flow".to_string(),
        start_line: 5,
        end_line: 15,
        tokens: 120,
        inbound: 0,
        outbound: 0,
    }];
    topology.add_node(target_node);
    topology.set_headings("target.md".to_string(), vec!["Auth Flow".to_string()]);

    let source_node = NoteNode::new("source.md", "/workspace/source.md", "Source Doc");
    topology.add_node(source_node);

    // Valid anchor link
    let valid_edge = NoteEdge {
        id: "source->target-valid".to_string(),
        source: "source.md".to_string(),
        target: "target.md".to_string(),
        target_fragment: Some("auth-flow".to_string()),
        link_type: LinkType::Markdown,
        is_broken: false,
        raw_text: "[Auth](./target.md#auth-flow)".to_string(),
        anchor: Some("#auth-flow".to_string()),
        line: Some(2),
        resolution_status: None,
        raw_target: Some("./target.md".to_string()),
        is_clickable: true,
        non_clickable_reason: None,
        clickable_target: None,
    };
    topology.add_edge(valid_edge);

    // Broken anchor link
    let broken_edge = NoteEdge {
        id: "source->target-broken".to_string(),
        source: "source.md".to_string(),
        target: "target.md".to_string(),
        target_fragment: Some("non-existent-section".to_string()),
        link_type: LinkType::Markdown,
        is_broken: false,
        raw_text: "[Broken](./target.md#non-existent-section)".to_string(),
        anchor: Some("#non-existent-section".to_string()),
        line: Some(4),
        resolution_status: None,
        raw_target: Some("./target.md".to_string()),
        is_clickable: true,
        non_clickable_reason: None,
        clickable_target: None,
    };
    topology.add_edge(broken_edge);

    topology.resolve_links();

    let valid = topology
        .edges
        .iter()
        .find(|e| e.id == "source.md->target.md" && !e.is_broken)
        .unwrap();
    assert_eq!(
        valid.resolution_status,
        Some(LinkResolutionStatus::Resolved)
    );
    assert_eq!(valid.target_fragment, Some("auth-flow".to_string()));

    let broken = topology
        .edges
        .iter()
        .find(|e| e.id == "source.md->target.md" && e.is_broken)
        .unwrap();
    assert_eq!(
        broken.resolution_status,
        Some(LinkResolutionStatus::BrokenFragmentAnchor)
    );

    // Verify inbound count on target fragment was incremented for the valid link
    let target = topology.get_node("target.md").unwrap();
    assert_eq!(target.fragments[0].inbound, 1);
}

#[test]
fn test_anchor_collision_handling_within_document() {
    let content = r#"## Configuration
Initial config.

## Configuration
Second config with identical heading.

## Configuration
Third config.
"#;

    let fragments = extract_fragments_from_document(content, &[]);
    assert_eq!(fragments.len(), 3);
    assert_eq!(fragments[0].anchor, "configuration");
    assert_eq!(fragments[1].anchor, "configuration-1");
    assert_eq!(fragments[2].anchor, "configuration-2");
}
