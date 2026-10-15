use llm_wiki_core::cli::issues_cmd::execute_issues;
use llm_wiki_core::fragment::concentration::{
    calculate_reference_concentration, has_high_reference_concentration,
};
use llm_wiki_core::fragment::FragmentMetadata;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::{LinkType, NoteEdge, NoteNode};
use llm_wiki_core::quality::structure::{
    calculate_structure_score, detect_structure_debt, StructureDebtType,
};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_reference_concentration_calculation() {
    let mut fragments = vec![
        FragmentMetadata {
            anchor: "sec-a".to_string(),
            heading: "Section A".to_string(),
            start_line: 1,
            end_line: 10,
            tokens: 500,
            inbound: 8,
            outbound: 1,
        },
        FragmentMetadata {
            anchor: "sec-b".to_string(),
            heading: "Section B".to_string(),
            start_line: 11,
            end_line: 20,
            tokens: 500,
            inbound: 2,
            outbound: 0,
        },
    ];

    // Total inbound = 10. sec-a = 8/10 = 0.8 (80%)
    let conc_a = calculate_reference_concentration(&fragments[0], &fragments);
    assert!((conc_a - 0.8).abs() < 1e-6);

    let conc_b = calculate_reference_concentration(&fragments[1], &fragments);
    assert!((conc_b - 0.2).abs() < 1e-6);

    // On a document < 50k tokens (e.g. 1000 tokens), should not flag high concentration debt
    assert!(has_high_reference_concentration(1000, &fragments).is_none());

    // On a document >= 50k tokens, 80% >= 70% should flag high concentration
    let high = has_high_reference_concentration(55000, &fragments);
    assert!(high.is_some());
    let (frag, conc) = high.unwrap();
    assert_eq!(frag.anchor, "sec-a");
    assert!((conc - 0.8).abs() < 1e-6);

    // When total inbound is 0, concentration should be 0.0
    fragments[0].inbound = 0;
    fragments[1].inbound = 0;
    assert_eq!(
        calculate_reference_concentration(&fragments[0], &fragments),
        0.0
    );
    assert!(has_high_reference_concentration(55000, &fragments).is_none());
}

#[test]
fn test_detect_structure_debt_oversized_and_missing_metadata() {
    let mut topology = GraphTopology::new();

    // Node 1: Oversized document (60k tokens) with >10 headings, no fragments materialized
    let mut node1 = NoteNode::new(
        "big.md".to_string(),
        "big.md".to_string(),
        "Big Document".to_string(),
    );
    node1.tokens = 60000;
    topology.add_node(node1);
    topology.set_headings(
        "big.md".to_string(),
        (1..=15).map(|i| format!("h{}", i)).collect(),
    );

    // Node 2: Normal document with fragments materialized
    let mut node2 = NoteNode::new(
        "normal.md".to_string(),
        "normal.md".to_string(),
        "Normal".to_string(),
    );
    node2.tokens = 5000;
    node2.fragments = vec![FragmentMetadata {
        anchor: "frag1".to_string(),
        heading: "Frag 1".to_string(),
        start_line: 1,
        end_line: 50,
        tokens: 5000,
        inbound: 1,
        outbound: 0,
    }];
    topology.add_node(node2);

    let findings = detect_structure_debt(&topology);
    assert_eq!(findings.len(), 2);

    let types: Vec<_> = findings.iter().map(|f| f.debt_type).collect();
    assert!(types.contains(&StructureDebtType::MissingFragmentMetadata));
    assert!(types.contains(&StructureDebtType::OversizedDocument));

    let score = calculate_structure_score(&findings, &topology);
    // Base penalties: MissingFragmentMetadata (25) + OversizedDocument (15) = 40
    // Betweenness = 0 -> weight = 1.0 -> score = 100 - 40 = 60.0
    assert!((score - 60.0).abs() < 1e-4);
}

#[test]
fn test_detect_structure_debt_unfragmented_central_hub() {
    let mut topology = GraphTopology::new();

    // Hub node with in_degree >= 10, tokens >= 20000, fragments <= 1
    let mut hub = NoteNode::new(
        "hub.md".to_string(),
        "hub.md".to_string(),
        "Hub".to_string(),
    );
    hub.tokens = 22000;
    hub.in_degree = 12;
    hub.betweenness = 0.5; // High centrality hub
    topology.add_node(hub);

    let findings = detect_structure_debt(&topology);
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].debt_type,
        StructureDebtType::UnfragmentedCentralHub
    );
    assert_eq!(findings[0].penalty, 20.0);

    let score = calculate_structure_score(&findings, &topology);
    // Weight = 1.0 + 3.0 * 0.5 = 2.5
    // Penalty = 20.0 * 2.5 = 50.0 -> score = 100 - 50 = 50.0
    assert!((score - 50.0).abs() < 1e-4);
}

#[test]
fn test_detect_structure_debt_high_reference_concentration() {
    let mut topology = GraphTopology::new();

    let mut doc = NoteNode::new(
        "huge.md".to_string(),
        "huge.md".to_string(),
        "Huge".to_string(),
    );
    doc.tokens = 60000;
    doc.fragments = vec![
        FragmentMetadata {
            anchor: "hotspot".to_string(),
            heading: "Hotspot Section".to_string(),
            start_line: 1,
            end_line: 200,
            tokens: 20000,
            inbound: 9,
            outbound: 0,
        },
        FragmentMetadata {
            anchor: "cold".to_string(),
            heading: "Cold Section".to_string(),
            start_line: 201,
            end_line: 400,
            tokens: 40000,
            inbound: 1,
            outbound: 0,
        },
    ];
    topology.add_node(doc);

    let findings = detect_structure_debt(&topology);
    assert_eq!(findings.len(), 2); // OversizedDocument (60k >= 50k) + HighReferenceConcentration (9/10 = 90% >= 70%)
    let types: Vec<_> = findings.iter().map(|f| f.debt_type).collect();
    assert!(types.contains(&StructureDebtType::OversizedDocument));
    assert!(types.contains(&StructureDebtType::HighReferenceConcentration));
}

#[test]
fn test_cli_execute_issues_pillar_structure() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();

    // Create an oversized note with many headings without materialized frontmatter
    let mut content = String::from("# Oversized Doc\n\nContent paragraph.\n\n");
    for i in 1..=12 {
        content.push_str(&format!("## Heading {}\n\nContent section {}\n\n", i, i));
    }
    fs::write(root.join("big_note.md"), content).unwrap();

    // Run execute_issues with pillar filter "structure"
    let res = execute_issues(root, "structure", None, false);
    assert!(res.is_ok());

    let res_json = execute_issues(root, "structure", None, true);
    assert!(res_json.is_ok());
}

#[test]
fn test_harness_drift_scoped_peer_parity_and_subproject_isolation() {
    let mut topology = GraphTopology::new();

    // Root harnesses: AGENTS.md and GEMINI.md
    let mut root_agents = NoteNode::new("AGENTS.md", "AGENTS.md", "AGENTS.md");
    root_agents.tokens = 500;
    topology.nodes.insert("AGENTS.md".to_string(), root_agents);

    let mut root_gemini = NoteNode::new("GEMINI.md", "GEMINI.md", "GEMINI.md");
    root_gemini.tokens = 500;
    topology.nodes.insert("GEMINI.md".to_string(), root_gemini);

    let doc_root1 = NoteNode::new("docs/arch.md", "docs/arch.md", "docs/arch.md");
    topology.nodes.insert("docs/arch.md".to_string(), doc_root1);
    let doc_root2 = NoteNode::new("docs/api.md", "docs/api.md", "docs/api.md");
    topology.nodes.insert("docs/api.md".to_string(), doc_root2);

    topology.edges.push(NoteEdge::new(
        "AGENTS.md",
        "docs/arch.md",
        LinkType::Markdown,
        "[arch](docs/arch.md)",
        None,
        Some(10),
    ));
    topology.edges.push(NoteEdge::new(
        "AGENTS.md",
        "docs/api.md",
        LinkType::Markdown,
        "[api](docs/api.md)",
        None,
        Some(12),
    ));
    // GEMINI.md in root only links to docs/arch.md (missing docs/api.md)
    topology.edges.push(NoteEdge::new(
        "GEMINI.md",
        "docs/arch.md",
        LinkType::Markdown,
        "[arch](docs/arch.md)",
        None,
        Some(10),
    ));

    // Isolated subproject harness: 04_harness/AGENTS.md (alone in 04_harness)
    let mut sub_isolated = NoteNode::new("04_harness/AGENTS.md", "04_harness/AGENTS.md", "04_harness/AGENTS.md");
    sub_isolated.tokens = 300;
    topology.nodes.insert("04_harness/AGENTS.md".to_string(), sub_isolated);
    topology.edges.push(NoteEdge::new(
        "04_harness/AGENTS.md",
        "04_harness/skills/theme.md",
        LinkType::Markdown,
        "[theme](04_harness/skills/theme.md)",
        None,
        Some(5),
    ));

    // Subproject with peers: showcases/ai_utils/AGENTS.md and showcases/ai_utils/GEMINI.md
    let mut show_agents = NoteNode::new("showcases/ai_utils/AGENTS.md", "showcases/ai_utils/AGENTS.md", "showcases/ai_utils/AGENTS.md");
    show_agents.tokens = 400;
    topology.nodes.insert("showcases/ai_utils/AGENTS.md".to_string(), show_agents);
    let mut show_gemini = NoteNode::new("showcases/ai_utils/GEMINI.md", "showcases/ai_utils/GEMINI.md", "showcases/ai_utils/GEMINI.md");
    show_gemini.tokens = 400;
    topology.nodes.insert("showcases/ai_utils/GEMINI.md".to_string(), show_gemini);

    topology.edges.push(NoteEdge::new(
        "showcases/ai_utils/AGENTS.md",
        "showcases/ai_utils/skills/code.md",
        LinkType::Markdown,
        "[code](showcases/ai_utils/skills/code.md)",
        None,
        Some(8),
    ));
    topology.edges.push(NoteEdge::new(
        "showcases/ai_utils/AGENTS.md",
        "showcases/ai_utils/skills/eval.md",
        LinkType::Markdown,
        "[eval](showcases/ai_utils/skills/eval.md)",
        None,
        Some(9),
    ));
    topology.edges.push(NoteEdge::new(
        "showcases/ai_utils/GEMINI.md",
        "showcases/ai_utils/skills/code.md",
        LinkType::Markdown,
        "[code](showcases/ai_utils/skills/code.md)",
        None,
        Some(8),
    ));

    let findings = detect_structure_debt(&topology);
    let drift_findings: Vec<_> = findings
        .into_iter()
        .filter(|f| f.debt_type == StructureDebtType::HarnessDrift)
        .collect();

    // Must be exactly 2 drift findings: GEMINI.md in root, and showcases/ai_utils/GEMINI.md
    assert_eq!(drift_findings.len(), 2);

    // 04_harness/AGENTS.md must NOT have any drift finding!
    assert!(drift_findings.iter().all(|f| f.document_path != "04_harness/AGENTS.md"));

    let root_drift = drift_findings.iter().find(|f| f.document_path == "GEMINI.md").unwrap();
    assert_eq!(root_drift.raw_metric_value, 1);
    assert!(root_drift.guidance.contains("docs/api.md"));
    // Root drift MUST NOT reference subproject files
    assert!(!root_drift.guidance.contains("04_harness"));
    assert!(!root_drift.guidance.contains("showcases/ai_utils"));

    let show_drift = drift_findings.iter().find(|f| f.document_path == "showcases/ai_utils/GEMINI.md").unwrap();
    assert_eq!(show_drift.raw_metric_value, 1);
    assert!(show_drift.guidance.contains("showcases/ai_utils/skills/eval.md"));
    // Subproject drift MUST NOT reference root or 04_harness files
    assert!(!show_drift.guidance.contains("docs/api.md"));
    assert!(!show_drift.guidance.contains("04_harness"));
}
