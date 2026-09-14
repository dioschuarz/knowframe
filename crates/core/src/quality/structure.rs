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

    for (path, node) in &topology.nodes {
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
                    "Document exceeds structural threshold ({} tokens, {} headings) without materialized fragment metadata. Run 'llm-wiki-core materialize' to resolve structural debt.",
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
    }

    findings
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
}
