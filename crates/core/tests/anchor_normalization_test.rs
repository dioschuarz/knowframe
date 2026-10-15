use llm_wiki_core::fragment::extractor::{slugify_heading, AnchorDisambiguator};

#[test]
fn test_slugify_heading_basic() {
    assert_eq!(slugify_heading("Introduction"), "introduction");
    assert_eq!(
        slugify_heading("System Architecture"),
        "system-architecture"
    );
    assert_eq!(
        slugify_heading("2.1 Authentication & Tokens (OAuth2)"),
        "21-authentication-tokens-oauth2"
    );
}

#[test]
fn test_slugify_heading_punctuation_and_whitespace() {
    assert_eq!(
        slugify_heading("  Getting Started -- Quick!  "),
        "getting-started-quick"
    );
    assert_eq!(slugify_heading("API: v2 / Endpoints"), "api-v2-endpoints");
    assert_eq!(
        slugify_heading("`code_snippet()` in heading"),
        "code-snippet-in-heading"
    );
    assert_eq!(slugify_heading("???"), "section");
}

#[test]
fn test_anchor_disambiguation() {
    let mut disambiguator = AnchorDisambiguator::new();
    assert_eq!(disambiguator.disambiguate("configuration"), "configuration");
    assert_eq!(
        disambiguator.disambiguate("configuration"),
        "configuration-1"
    );
    assert_eq!(
        disambiguator.disambiguate("configuration"),
        "configuration-2"
    );
    assert_eq!(disambiguator.disambiguate("architecture"), "architecture");
    assert_eq!(disambiguator.disambiguate("architecture"), "architecture-1");
}

#[test]
fn test_cross_file_heading_rename_impact_t018() {
    use llm_wiki_core::graph::resolver::{resolve_edge_integrity, validate_fragment_anchors, ResolverIndex};
    use llm_wiki_core::model::{DeterministicFindingSeverity, LinkResolutionStatus, LinkType, NoteEdge};
    use std::collections::HashMap;
    use std::path::PathBuf;

    let mut known_nodes = HashMap::new();
    known_nodes.insert(
        "notes/overview.md".to_string(),
        PathBuf::from("/vault/notes/overview.md"),
    );
    known_nodes.insert(
        "notes/architecture.md".to_string(),
        PathBuf::from("/vault/notes/architecture.md"),
    );

    let index = ResolverIndex::new(&known_nodes);

    // Initial state: target document has heading "System Architecture"
    let mut headings = HashMap::new();
    headings.insert(
        "notes/architecture.md".to_string(),
        vec!["System Architecture".to_string()],
    );

    let mut edge = NoteEdge::new(
        "notes/overview.md",
        "architecture.md",
        LinkType::Markdown,
        "System Architecture",
        Some("system-architecture".to_string()),
        Some(10),
    );

    resolve_edge_integrity(&mut edge, &index, &headings);
    assert!(!edge.is_broken);
    assert_eq!(edge.resolution_status, Some(LinkResolutionStatus::Resolved));
    assert_eq!(edge.target_fragment.as_deref(), Some("system-architecture"));

    // Mutation: Target heading is renamed from "System Architecture" to "System Topology"
    headings.insert(
        "notes/architecture.md".to_string(),
        vec!["System Topology".to_string()],
    );

    let mut renamed_edge = NoteEdge::new(
        "notes/overview.md",
        "architecture.md",
        LinkType::Markdown,
        "System Architecture",
        Some("system-architecture".to_string()),
        Some(10),
    );

    resolve_edge_integrity(&mut renamed_edge, &index, &headings);
    assert!(renamed_edge.is_broken);
    assert_eq!(
        renamed_edge.resolution_status,
        Some(LinkResolutionStatus::BrokenFragmentAnchor)
    );

    // Validate that validate_fragment_anchors emits a standardized BROKEN_FRAGMENT_ANCHOR finding
    let findings = validate_fragment_anchors(&[renamed_edge], &index, &headings);
    assert_eq!(findings.len(), 1);
    let finding = &findings[0];
    assert_eq!(finding.rule_id, "BROKEN_FRAGMENT_ANCHOR");
    assert_eq!(finding.severity, DeterministicFindingSeverity::Error);
    assert_eq!(finding.subject, "notes/overview.md");
    assert_eq!(finding.evidence.target.as_deref(), Some("notes/architecture.md"));
    assert_eq!(finding.evidence.anchor.as_deref(), Some("system-architecture"));
    assert_eq!(finding.evidence.line_number, Some(10));
}

#[test]
fn test_cross_file_disambiguated_anchor_resolution_t018() {
    use llm_wiki_core::graph::resolver::{resolve_edge_integrity, ResolverIndex};
    use llm_wiki_core::model::{LinkResolutionStatus, LinkType, NoteEdge};
    use std::collections::HashMap;
    use std::path::PathBuf;

    let mut known_nodes = HashMap::new();
    known_nodes.insert(
        "notes/config.md".to_string(),
        PathBuf::from("/vault/notes/config.md"),
    );
    let index = ResolverIndex::new(&known_nodes);

    let mut headings = HashMap::new();
    headings.insert(
        "notes/config.md".to_string(),
        vec!["Configuration".to_string(), "Configuration".to_string()],
    );

    // Edge pointing to first heading #configuration
    let mut edge1 = NoteEdge::new(
        "notes/index.md",
        "config.md",
        LinkType::Markdown,
        "Config 1",
        Some("configuration".to_string()),
        Some(5),
    );
    resolve_edge_integrity(&mut edge1, &index, &headings);
    assert!(!edge1.is_broken);
    assert_eq!(edge1.target_fragment.as_deref(), Some("configuration"));

    // Edge pointing to disambiguated second heading #configuration-1
    let mut edge2 = NoteEdge::new(
        "notes/index.md",
        "config.md",
        LinkType::Markdown,
        "Config 2",
        Some("configuration-1".to_string()),
        Some(12),
    );
    resolve_edge_integrity(&mut edge2, &index, &headings);
    assert!(!edge2.is_broken);
    assert_eq!(edge2.target_fragment.as_deref(), Some("configuration-1"));

    // Edge pointing to non-existent third heading #configuration-2
    let mut edge3 = NoteEdge::new(
        "notes/index.md",
        "config.md",
        LinkType::Markdown,
        "Config 3",
        Some("configuration-2".to_string()),
        Some(20),
    );
    resolve_edge_integrity(&mut edge3, &index, &headings);
    assert!(edge3.is_broken);
    assert_eq!(
        edge3.resolution_status,
        Some(LinkResolutionStatus::BrokenFragmentAnchor)
    );
}

#[test]
fn test_ast_visitor_cross_document_resolution_t019() {
    use llm_wiki_core::ast::visitor::AstVisitor;
    use std::collections::HashMap;

    let doc_content = r#"# Main Doc
Links to [Section One](target.md#section-one) and [Invalid](target.md#section-three).
"#;
    let mut doc = AstVisitor::visit("main.md", doc_content);
    assert_eq!(doc.references.len(), 2);

    let mut target_headings = HashMap::new();
    target_headings.insert(
        "target.md".to_string(),
        vec!["Section One".to_string(), "Section Two".to_string()],
    );

    let findings = AstVisitor::resolve_cross_document_headings(&mut doc, &target_headings);

    // First reference resolved successfully
    assert!(doc.references[0].is_resolved);
    assert_eq!(doc.references[0].target_fragment.as_deref(), Some("section-one"));

    // Second reference failed to resolve and emitted BROKEN_FRAGMENT_ANCHOR
    assert!(!doc.references[1].is_resolved);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule_id, "BROKEN_FRAGMENT_ANCHOR");
    assert_eq!(findings[0].evidence.anchor.as_deref(), Some("section-three"));
}
