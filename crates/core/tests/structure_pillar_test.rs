use std::fs;
use tempfile::TempDir;
use llm_wiki_core::fragment::concentration::{calculate_reference_concentration, has_high_reference_concentration};
use llm_wiki_core::fragment::FragmentMetadata;
use llm_wiki_core::graph::builder::GraphTopology;
use llm_wiki_core::model::NoteNode;
use llm_wiki_core::quality::structure::{detect_structure_debt, calculate_structure_score, StructureDebtType};
use llm_wiki_core::cli::issues_cmd::execute_issues;

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
    assert_eq!(calculate_reference_concentration(&fragments[0], &fragments), 0.0);
    assert!(has_high_reference_concentration(55000, &fragments).is_none());
}

#[test]
fn test_detect_structure_debt_oversized_and_missing_metadata() {
    let mut topology = GraphTopology::new();

    // Node 1: Oversized document (60k tokens) with >10 headings, no fragments materialized
    let mut node1 = NoteNode::new("big.md".to_string(), "big.md".to_string(), "Big Document".to_string());
    node1.tokens = 60000;
    topology.add_node(node1);
    topology.set_headings("big.md".to_string(), (1..=15).map(|i| format!("h{}", i)).collect());

    // Node 2: Normal document with fragments materialized
    let mut node2 = NoteNode::new("normal.md".to_string(), "normal.md".to_string(), "Normal".to_string());
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
    let mut hub = NoteNode::new("hub.md".to_string(), "hub.md".to_string(), "Hub".to_string());
    hub.tokens = 22000;
    hub.in_degree = 12;
    hub.betweenness = 0.5; // High centrality hub
    topology.add_node(hub);

    let findings = detect_structure_debt(&topology);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].debt_type, StructureDebtType::UnfragmentedCentralHub);
    assert_eq!(findings[0].penalty, 20.0);

    let score = calculate_structure_score(&findings, &topology);
    // Weight = 1.0 + 3.0 * 0.5 = 2.5
    // Penalty = 20.0 * 2.5 = 50.0 -> score = 100 - 50 = 50.0
    assert!((score - 50.0).abs() < 1e-4);
}

#[test]
fn test_detect_structure_debt_high_reference_concentration() {
    let mut topology = GraphTopology::new();

    let mut doc = NoteNode::new("huge.md".to_string(), "huge.md".to_string(), "Huge".to_string());
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
