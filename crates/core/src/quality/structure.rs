use crate::fragment::concentration::has_high_reference_concentration;
use crate::graph::builder::GraphTopology;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructureDebtType {
    /// Document exceeds 50,000 tokens or 10 headings and lacks materialized fragment metadata.
    MissingFragmentMetadata,
    /// Document exceeds 50,000 tokens total size (bloat debt).
    OversizedDocument,
    /// Single fragment absorbs >= 70% of inbound references on document with >= 50,000 tokens.
    HighReferenceConcentration,
    /// Central hub (in-degree >= 10, tokens >= 20,000) lacking heading fragments (<= 1 fragment).
    UnfragmentedCentralHub,
    /// Intermediate routing layer anti-pattern (FR-013, SC-003).
    SecondaryRoutingLayer,
    /// Route divergence across multiple configured AI harness entry points.
    HarnessDrift,
    /// Link targets document without specifying an anchor fragment when target has addressable sections.
    UnanchoredReference,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructureDebtFinding {
    pub debt_type: StructureDebtType,
    pub document_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_anchor: Option<String>,
    /// Raw integer metric (e.g. 50231 tokens or 74% concentration scaled to 74).
    pub raw_metric_value: u32,
    pub penalty: f32,
    pub guidance: String,
}

/// Detects structural debt findings across the graph topology adhering to Pillar 2 rules.
pub fn detect_structure_debt(topology: &GraphTopology) -> Vec<StructureDebtFinding> {
    let mut findings = Vec::new();

    let mut sorted_nodes: Vec<_> = topology.nodes.iter().collect();
    sorted_nodes.sort_by(|a, b| a.0.cmp(b.0));

    for (path, node) in sorted_nodes {
        let headings_count = topology
            .headings
            .get(path)
            .map(|h| h.len())
            .unwrap_or_else(|| node.fragments.len());

        // 1. MissingFragmentMetadata: >50k tokens or >10 headings and 0 fragments
        if (node.tokens > 50_000 || headings_count > 10) && node.fragments.is_empty() {
            findings.push(StructureDebtFinding {
                debt_type: StructureDebtType::MissingFragmentMetadata,
                document_path: path.clone(),
                fragment_anchor: None,
                raw_metric_value: if node.tokens > 50_000 { node.tokens } else { headings_count as u32 },
                penalty: 25.0,
                guidance: format!(
                    "Document exceeds structural threshold ({} tokens, {} headings) without materialized fragment metadata. Run 'knowframe-core materialize' to resolve structural debt.",
                    node.tokens, headings_count
                ),
            });
        }

        // 2. OversizedDocument: >50,000 tokens
        if node.tokens > 50_000 {
            findings.push(StructureDebtFinding {
                debt_type: StructureDebtType::OversizedDocument,
                document_path: path.clone(),
                fragment_anchor: None,
                raw_metric_value: node.tokens,
                penalty: 15.0,
                guidance: format!(
                    "Document exceeds 50,000 tokens ({} tokens). Consider splitting into smaller modular notes.",
                    node.tokens
                ),
            });
        }

        // 3. HighReferenceConcentration: >=70% on docs with >=50,000 tokens
        if let Some((frag, conc)) = has_high_reference_concentration(node.tokens, &node.fragments) {
            let pct = (conc * 100.0).round() as u32;
            findings.push(StructureDebtFinding {
                debt_type: StructureDebtType::HighReferenceConcentration,
                document_path: path.clone(),
                fragment_anchor: Some(frag.anchor.clone()),
                raw_metric_value: pct,
                penalty: 20.0,
                guidance: format!(
                    "{}% of inbound references target section '{}'. Consider splitting document or referencing sections directly.",
                    pct, frag.anchor
                ),
            });
        }

        // 4. UnfragmentedCentralHub: in-degree >= 10, tokens >= 20k, fragments <= 1
        if node.in_degree >= 10 && node.tokens >= 20_000 && node.fragments.len() <= 1 {
            findings.push(StructureDebtFinding {
                debt_type: StructureDebtType::UnfragmentedCentralHub,
                document_path: path.clone(),
                fragment_anchor: None,
                raw_metric_value: node.in_degree,
                penalty: 20.0,
                guidance: format!(
                    "Central hub document has {} inbound references and {} tokens but lacks section headings. Add descriptive headings to make sections addressable.",
                    node.in_degree, node.tokens
                ),
            });
        }

        // 5. SecondaryRoutingLayer: intermediate routing hop anti-pattern (FR-013, SC-003)
        let file_name = std::path::Path::new(path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        if (file_name.eq_ignore_ascii_case("index.md")
            || file_name.eq_ignore_ascii_case("memory.md")
            || file_name.eq_ignore_ascii_case("hot-index.md"))
            && node.out_degree > 0
        {
            findings.push(StructureDebtFinding {
                debt_type: StructureDebtType::SecondaryRoutingLayer,
                document_path: path.clone(),
                fragment_anchor: None,
                raw_metric_value: node.out_degree,
                penalty: 15.0,
                guidance: format!(
                    "Secondary routing layer detected at '{}' with {} outgoing routes. Consolidate routes directly into primary AI Harness Entry Point (FR-013, SC-003).",
                    path, node.out_degree
                ),
            });
        }
    }

    // 6. UnanchoredReference: Link targets document without specifying an anchor fragment when target has addressable sections
    for edge in &topology.edges {
        if !edge.is_broken && edge.anchor.is_none() && edge.target_fragment.is_none() {
            if let Some(target_node) = topology.nodes.get(&edge.target) {
                // If the target document has 2 or more section fragments, an anchor should direct the reader
                if target_node.fragments.len() > 1 {
                    findings.push(StructureDebtFinding {
                        debt_type: StructureDebtType::UnanchoredReference,
                        document_path: edge.source.clone(),
                        fragment_anchor: Some(edge.target.clone()),
                        raw_metric_value: target_node.fragments.len() as u32,
                        penalty: 2.0,
                        guidance: format!(
                            "Link in '{}' (line {}) targets '{}' without a section fragment anchor. The destination has {} sections; target a specific section anchor to guide context reading.",
                            edge.source,
                            edge.line.unwrap_or(1),
                            edge.target,
                            target_node.fragments.len()
                        ),
                    });
                }
            }
        }
    }

    // 7. HarnessDrift: Route divergence across peer AI Harness entry points within the same directory scope
    let standard_harness_names = [
        "AGENTS.md",
        "CLAUDE.md",
        "GEMINI.md",
        ".github/copilot-instructions.md",
    ];

    let mut harness_paths: Vec<&String> = Vec::new();
    for path in topology.nodes.keys() {
        let normalized = path.replace('\\', "/");
        if standard_harness_names
            .iter()
            .any(|h| normalized == *h || normalized.ends_with(&format!("/{}", h)))
        {
            harness_paths.push(path);
        }
    }

    if !harness_paths.is_empty() {
        use std::collections::{HashMap, HashSet};

        harness_paths.sort();

        // Group harness entry points by their directory scope
        let mut scope_groups: HashMap<String, Vec<&String>> = HashMap::new();
        for h_path in &harness_paths {
            let scope = get_harness_dir_scope(h_path);
            scope_groups.entry(scope).or_default().push(h_path);
        }

        let mut sorted_scopes: Vec<(String, Vec<&String>)> = scope_groups.into_iter().collect();
        sorted_scopes.sort_by(|a, b| a.0.cmp(&b.0));

        for (_scope, mut peers) in sorted_scopes {
            // If there's only 1 harness in this scope, there are no competing peer harnesses
            // to drift against (e.g. isolated subproject/submodule harness).
            if peers.len() <= 1 {
                continue;
            }

            let mut harness_targets: HashMap<&str, HashSet<String>> = HashMap::new();
            for h_path in &peers {
                let targets: HashSet<String> = topology
                    .edges
                    .iter()
                    .filter(|e| &e.source == *h_path)
                    .map(|e| e.target.clone())
                    .collect();
                harness_targets.insert(h_path.as_str(), targets);
            }

            let mut all_targets: HashSet<String> = HashSet::new();
            for targets in harness_targets.values() {
                all_targets.extend(targets.iter().cloned());
            }

            peers.sort();

            for h_path in &peers {
                if let Some(h_targets) = harness_targets.get(h_path.as_str()) {
                    let mut missing: Vec<&String> = all_targets.difference(h_targets).collect();
                    missing.sort();
                    if !missing.is_empty() {
                        findings.push(StructureDebtFinding {
                            debt_type: StructureDebtType::HarnessDrift,
                            document_path: (*h_path).clone(),
                            fragment_anchor: None,
                            raw_metric_value: missing.len() as u32,
                            penalty: 15.0,
                            guidance: format!(
                                "AI harness route drift: '{}' has {} routes but is missing {} route(s) configured in peer harnesses (e.g. {}). All harness entry points in this scope must be synchronized and equal. Click 'Sync Harnesses' to synchronize.",
                                h_path,
                                h_targets.len(),
                                missing.len(),
                                missing
                                    .iter()
                                    .take(2)
                                    .map(|s| s.as_str())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                        });
                    }
                }
            }
        }
    }

    findings.sort_by(|a, b| {
        a.document_path
            .cmp(&b.document_path)
            .then_with(|| format!("{:?}", a.debt_type).cmp(&format!("{:?}", b.debt_type)))
            .then_with(|| a.fragment_anchor.cmp(&b.fragment_anchor))
    });

    findings
}

/// Returns the normalized project directory scope for a harness entry point.
///
/// Peer parity for AI harness entry points (e.g. `AGENTS.md` vs `GEMINI.md`) is evaluated
/// strictly within the same directory/project scope boundary. Isolated sub-project harnesses
/// (such as `04_harness/AGENTS.md` or submodule harnesses) are scoped to their directory,
/// preventing cross-project route leakage or false drift penalties.
pub fn get_harness_dir_scope(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    let trimmed = normalized.trim_start_matches("./").trim_start_matches('/');
    let mut dir = std::path::Path::new(trimmed)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    if dir == "." {
        dir = String::new();
    }
    // If it's inside `.github` (e.g. `.github/copilot-instructions.md` or `submodule/.github/copilot-instructions.md`),
    // the project scope is the parent of `.github`.
    if dir == ".github" {
        String::new()
    } else if dir.ends_with("/.github") {
        dir[..dir.len() - "/.github".len()].to_string()
    } else {
        dir
    }
}

/// Calculates Pillar 2 Structure score (0.0 .. 100.0) with topological betweenness weighting.
pub fn calculate_structure_score(
    findings: &[StructureDebtFinding],
    topology: &GraphTopology,
) -> f32 {
    let mut total_deductions = 0.0f32;

    for f in findings {
        let b = topology
            .get_node(&f.document_path)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = 1.0 + 3.0 * (b as f32);
        total_deductions += f.penalty * weight;
    }

    let score = (100.0 - total_deductions).clamp(0.0, 100.0);
    (score * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure_debt_types() {
        let finding = StructureDebtFinding {
            debt_type: StructureDebtType::HighReferenceConcentration,
            document_path: "docs/architecture.md".to_string(),
            fragment_anchor: Some("authentication".to_string()),
            raw_metric_value: 74,
            penalty: 20.0,
            guidance: "74% of inbound references target 'authentication'. Consider splitting document or referencing sections directly.".to_string(),
        };

        let json = serde_json::to_string(&finding).unwrap();
        assert!(json.contains("\"debt_type\":\"high_reference_concentration\""));
        assert!(json.contains("\"raw_metric_value\":74"));

        let deserialized: StructureDebtFinding = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, finding);
    }

    #[test]
    fn test_harness_drift_detection() {
        use crate::model::{LinkType, NoteEdge, NoteNode};

        let mut topology = GraphTopology::new();

        // Add AGENTS.md and GEMINI.md
        let mut agents_node = NoteNode::new("AGENTS.md", "AGENTS.md", "AGENTS.md");
        agents_node.tokens = 500;
        topology.nodes.insert("AGENTS.md".to_string(), agents_node);

        let mut gemini_node = NoteNode::new("GEMINI.md", "GEMINI.md", "GEMINI.md");
        gemini_node.tokens = 500;
        topology.nodes.insert("GEMINI.md".to_string(), gemini_node);

        let doc_a = NoteNode::new("docs/a.md", "docs/a.md", "docs/a.md");
        topology.nodes.insert("docs/a.md".to_string(), doc_a);
        let doc_b = NoteNode::new("docs/b.md", "docs/b.md", "docs/b.md");
        topology.nodes.insert("docs/b.md".to_string(), doc_b);

        // AGENTS.md links to docs/a.md and docs/b.md
        topology.edges.push(NoteEdge::new(
            "AGENTS.md",
            "docs/a.md",
            LinkType::Markdown,
            "[a](docs/a.md)",
            None,
            Some(10),
        ));
        topology.edges.push(NoteEdge::new(
            "AGENTS.md",
            "docs/b.md",
            LinkType::Markdown,
            "[b](docs/b.md)",
            None,
            Some(11),
        ));

        // GEMINI.md only links to docs/a.md (missing docs/b.md)
        topology.edges.push(NoteEdge::new(
            "GEMINI.md",
            "docs/a.md",
            LinkType::Markdown,
            "[a](docs/a.md)",
            None,
            Some(10),
        ));

        // Isolated subproject harness: 04_harness/AGENTS.md
        let mut sub_harness = NoteNode::new("04_harness/AGENTS.md", "04_harness/AGENTS.md", "04_harness/AGENTS.md");
        sub_harness.tokens = 400;
        topology.nodes.insert("04_harness/AGENTS.md".to_string(), sub_harness);
        topology.edges.push(NoteEdge::new(
            "04_harness/AGENTS.md",
            "04_harness/skills/theme.md",
            LinkType::Markdown,
            "[theme](04_harness/skills/theme.md)",
            None,
            Some(5),
        ));

        // Subproject with peers: sub/AGENTS.md and sub/GEMINI.md
        let mut sub_p1 = NoteNode::new("sub/AGENTS.md", "sub/AGENTS.md", "sub/AGENTS.md");
        sub_p1.tokens = 300;
        topology.nodes.insert("sub/AGENTS.md".to_string(), sub_p1);
        let mut sub_p2 = NoteNode::new("sub/GEMINI.md", "sub/GEMINI.md", "sub/GEMINI.md");
        sub_p2.tokens = 300;
        topology.nodes.insert("sub/GEMINI.md".to_string(), sub_p2);

        topology.edges.push(NoteEdge::new(
            "sub/AGENTS.md",
            "sub/doc_x.md",
            LinkType::Markdown,
            "[x](sub/doc_x.md)",
            None,
            Some(10),
        ));
        topology.edges.push(NoteEdge::new(
            "sub/AGENTS.md",
            "sub/doc_y.md",
            LinkType::Markdown,
            "[y](sub/doc_y.md)",
            None,
            Some(12),
        ));
        topology.edges.push(NoteEdge::new(
            "sub/GEMINI.md",
            "sub/doc_x.md",
            LinkType::Markdown,
            "[x](sub/doc_x.md)",
            None,
            Some(10),
        ));

        let findings = detect_structure_debt(&topology);
        let drift_findings: Vec<_> = findings
            .into_iter()
            .filter(|f| f.debt_type == StructureDebtType::HarnessDrift)
            .collect();

        // Exactly 2 drift findings: GEMINI.md in root, and sub/GEMINI.md in sub/
        // 04_harness/AGENTS.md must have ZERO findings because it is isolated!
        assert_eq!(drift_findings.len(), 2);

        let root_drift = drift_findings.iter().find(|f| f.document_path == "GEMINI.md").unwrap();
        assert_eq!(root_drift.raw_metric_value, 1);
        assert!(root_drift.guidance.contains("docs/b.md"));
        assert!(!root_drift.guidance.contains("04_harness"));
        assert!(!root_drift.guidance.contains("sub/doc_y.md"));

        let sub_drift = drift_findings.iter().find(|f| f.document_path == "sub/GEMINI.md").unwrap();
        assert_eq!(sub_drift.raw_metric_value, 1);
        assert!(sub_drift.guidance.contains("sub/doc_y.md"));
        assert!(!sub_drift.guidance.contains("docs/b.md"));
        assert!(!sub_drift.guidance.contains("04_harness"));

        // Verify 04_harness/AGENTS.md has zero drift findings
        assert!(drift_findings.iter().all(|f| f.document_path != "04_harness/AGENTS.md"));
    }
}
