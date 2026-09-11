use llm_wiki_core::ast::parser::parse_markdown_ast;
use llm_wiki_core::model::LinkType;

#[test]
fn test_standard_markdown_link_and_title() {
    let content = r#"# Architecture Overview

This document links to [Security Specs](security.md) and [System Topology](arch/topology.md#database-layer).
"#;
    let parsed = parse_markdown_ast("docs/index.md", content);
    assert_eq!(parsed.title.as_deref(), Some("Architecture Overview"));
    assert_eq!(parsed.edges.len(), 2);

    let edge1 = &parsed.edges[0];
    assert_eq!(edge1.target, "security.md");
    assert_eq!(edge1.raw_text, "Security Specs");
    assert_eq!(edge1.link_type, LinkType::Markdown);
    assert_eq!(edge1.anchor, None);
    assert_eq!(edge1.line, Some(3));

    let edge2 = &parsed.edges[1];
    assert_eq!(edge2.target, "arch/topology.md");
    assert_eq!(edge2.anchor.as_deref(), Some("database-layer"));
    assert_eq!(edge2.line, Some(3));
}

#[test]
fn test_code_span_and_fenced_block_immunity() {
    let content = r#"# Code Immunity Test

Here is inline code: `[ignored](fake.md)` and `[[ignored_wiki]]`.

```markdown
# Fenced block
[also_ignored](fenced.md)
[[also_ignored_wiki]]
```

Real link outside: [Real Link](real.md).
"#;
    let parsed = parse_markdown_ast("test.md", content);
    assert_eq!(parsed.edges.len(), 1);
    assert_eq!(parsed.edges[0].target, "real.md");
}

#[test]
fn test_obsidian_wikilinks_and_aliases() {
    let content = r#"# Wikilink Test

Check [[getting_started]] and [[architecture/design|System Design]] and [[notes/adr#Decisions]].
"#;
    let parsed = parse_markdown_ast("readme.md", content);
    assert_eq!(parsed.edges.len(), 3);

    assert_eq!(parsed.edges[0].target, "getting_started.md");
    assert_eq!(parsed.edges[0].link_type, LinkType::Wikilink);
    assert_eq!(parsed.edges[0].anchor, None);

    assert_eq!(parsed.edges[1].target, "architecture/design.md");
    assert_eq!(parsed.edges[1].raw_text, "System Design");
    assert_eq!(parsed.edges[1].link_type, LinkType::Wikilink);

    assert_eq!(parsed.edges[2].target, "notes/adr.md");
    assert_eq!(parsed.edges[2].anchor.as_deref(), Some("Decisions"));
}

#[test]
fn test_tag_extraction() {
    let content = r#"---
tags:
  - Architecture
  - Core
---

# Tag Test

Check #Security and #DEV_OPS/v1 inline tags.
"#;
    let parsed = parse_markdown_ast("tags.md", content);
    assert!(parsed.tags.contains(&"architecture".to_string()));
    assert!(parsed.tags.contains(&"core".to_string()));
    assert!(parsed.tags.contains(&"security".to_string()));
    assert!(parsed.tags.contains(&"dev_ops/v1".to_string()));
}
