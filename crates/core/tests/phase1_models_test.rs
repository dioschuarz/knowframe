use llm_wiki_core::drift::{FreshnessEvidence, FreshnessTier};
use llm_wiki_core::fragment::{FragmentMetadata, MaterializedFragmentEntry, MaterializedWikiFrontmatter};
use llm_wiki_core::model::{LinkResolutionStatus, LinkType, NoteEdge, NoteNode};
use llm_wiki_core::quality::{FivePillarQualityScore, QualityPillar, StructureDebtFinding, StructureDebtType};
use llm_wiki_core::trace::ContextTraceSectionResult;

#[test]
fn test_fragment_models_t001() {
    let fragment = FragmentMetadata {
        anchor: "authentication".to_string(),
        heading: "Authentication".to_string(),
        start_line: 10,
        end_line: 35,
        tokens: 4200,
        inbound: 15,
        outbound: 3,
    };

    let json = serde_json::to_string(&fragment).unwrap();
    assert!(json.contains("\"tokens\":4200"));
    assert!(json.contains("\"inbound\":15"));

    let mut frontmatter = MaterializedWikiFrontmatter::default();
    frontmatter.fragments.insert(
        "authentication".to_string(),
        MaterializedFragmentEntry {
            tokens: 4200,
            inbound: 15,
            outbound: 3,
        },
    );

    let yaml = serde_yaml::to_string(&frontmatter).unwrap();
    assert!(yaml.contains("authentication:"));
    assert!(yaml.contains("tokens: 4200"));
}

#[test]
fn test_note_node_with_fragments_t002() {
    let mut node = NoteNode::new("docs/arch.md", "/repo/docs/arch.md", "Architecture");
    assert_eq!(node.fragments.len(), 0);
    assert!(node.freshness.is_none());

    node.fragments.push(FragmentMetadata {
        anchor: "auth".to_string(),
        heading: "Auth".to_string(),
        start_line: 5,
        end_line: 20,
        tokens: 1500,
        inbound: 4,
        outbound: 1,
    });

    let json = serde_json::to_string(&node).unwrap();
    assert!(json.contains("\"fragments\":[{\"anchor\":\"auth\""));

    let decoded: NoteNode = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.fragments.len(), 1);
    assert_eq!(decoded.fragments[0].tokens, 1500);
}

#[test]
fn test_note_edge_with_target_fragment_t003() {
    let edge = NoteEdge::new(
        "intro.md",
        "arch.md",
        LinkType::Markdown,
        "[Auth](./arch.md#authentication)",
        Some("#authentication".to_string()),
        Some(14),
    );

    assert_eq!(edge.target_fragment, Some("authentication".to_string()));
    assert_eq!(edge.resolution_status, Some(LinkResolutionStatus::Resolved));

    let json = serde_json::to_string(&edge).unwrap();
    assert!(json.contains("\"target_fragment\":\"authentication\""));

    // Verify BrokenFragmentAnchor
    let broken_edge_json = r##"{"id":"a->b","source":"a","target":"b","link_type":"markdown","is_broken":true,"raw_text":"[Broken](./b#missing)","anchor":"#missing","target_fragment":"missing","line":12,"resolution_status":"broken_fragment_anchor"}"##;
    let decoded_edge: NoteEdge = serde_json::from_str(broken_edge_json).unwrap();
    assert_eq!(
        decoded_edge.resolution_status,
        Some(LinkResolutionStatus::BrokenFragmentAnchor)
    );
}

#[test]
fn test_structure_debt_models_t004() {
    let finding = StructureDebtFinding {
        debt_type: StructureDebtType::MissingFragmentMetadata,
        document_path: "docs/huge.md".to_string(),
        fragment_anchor: None,
        raw_metric_value: 52000,
        penalty: 25.0,
        guidance: "Document exceeds 50k tokens and lacks materialized fragment metadata.".to_string(),
    };

    let json = serde_json::to_string(&finding).unwrap();
    assert!(json.contains("\"debt_type\":\"missing_fragment_metadata\""));
    assert!(json.contains("\"raw_metric_value\":52000"));
}

#[test]
fn test_freshness_evidence_models_t005() {
    let evidence = FreshnessEvidence {
        score: 70.0,
        tier: FreshnessTier::PotentiallyStale,
        missing_citations: vec!["src/old_api.rs".to_string()],
        code_modified_after_doc: vec!["src/auth.rs".to_string()],
        deleted_symbols: vec!["OldAuthService".to_string()],
        canonical_contradiction: None,
        reasons: vec!["Missing citation: src/old_api.rs".to_string()],
    };

    let json = serde_json::to_string(&evidence).unwrap();
    assert!(json.contains("\"tier\":\"potentially_stale\""));
    assert!(json.contains("\"score\":70.0"));
}

#[test]
fn test_five_pillar_score_and_trace_section_models_t006() {
    let mut score = FivePillarQualityScore::default();
    score.structure_score = 85.0;
    score.reasons.insert(
        QualityPillar::Structure,
        vec!["Document docs/huge.md exceeds 50k tokens".to_string()],
    );

    let json = serde_json::to_string(&score).unwrap();
    assert!(json.contains("\"structure_score\":85.0"));
    assert!(json.contains("\"structure\":[\"Document docs/huge.md"));

    let trace = ContextTraceSectionResult {
        query: "authentication tokens".to_string(),
        entrypoint: "AGENTS.md".to_string(),
        target_document: "docs/arch.md".to_string(),
        target_fragment: Some("authentication".to_string()),
        path: vec!["AGENTS.md".to_string(), "docs/arch.md#authentication".to_string()],
        depth: 1,
        document_raw_tokens: 50231,
        fragment_raw_tokens: Some(5132),
        context_reduction_percent: 89.8,
        relevance_reasons: vec!["Heading 'Authentication' matches query".to_string()],
    };

    let trace_json = serde_json::to_string(&trace).unwrap();
    assert!(trace_json.contains("\"target_fragment\":\"authentication\""));
    assert!(trace_json.contains("\"context_reduction_percent\":89.8"));
}
