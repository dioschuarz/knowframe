use crate::fragment::FragmentMetadata;
use crate::model::NoteNode;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// 10-Dimensional structural metrics for knowledge graph document nodes.
/// Invariant IX: Numerical metric fields MUST retain complete raw floating point representations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralMetrics {
    pub cohesion: f32,
    pub coupling: f32,
    pub reference_concentration: f32,
    pub granularity: f32,
    pub context_efficiency: f32,
    pub discoverability: f32,
    pub reusability: f32,
    pub semantic_diversity: f32,
    pub navigational_complexity: f32,
    pub maintenance_coupling: f32,
}

impl Default for StructuralMetrics {
    fn default() -> Self {
        Self {
            cohesion: 0.5,
            coupling: 0.5,
            reference_concentration: 0.5,
            granularity: 0.5,
            context_efficiency: 0.5,
            discoverability: 0.5,
            reusability: 0.5,
            semantic_diversity: 0.5,
            navigational_complexity: 0.5,
            maintenance_coupling: 0.5,
        }
    }
}

/// Tokenizes text into lowercase words (>2 chars) for lightweight term analysis.
fn extract_terms(text: &str) -> HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .filter(|w| w.len() > 2)
        .map(|w| w.to_lowercase())
        .collect()
}

/// Computes Cohesion (C) in [0.0, 1.0]: concept overlap across document sections.
pub fn compute_cohesion(fragments: &[FragmentMetadata]) -> f32 {
    if fragments.len() <= 1 {
        return 1.0;
    }

    let mut term_sets: Vec<HashSet<String>> = Vec::with_capacity(fragments.len());
    for f in fragments {
        term_sets.push(extract_terms(&f.heading));
    }

    let mut total_sim = 0.0;
    let mut comparisons = 0;

    for i in 0..term_sets.len() {
        for j in (i + 1)..term_sets.len() {
            let intersection = term_sets[i].intersection(&term_sets[j]).count();
            let union = term_sets[i].union(&term_sets[j]).count();
            let sim = if union == 0 {
                0.5 // Both empty or no keywords
            } else {
                intersection as f32 / union as f32
            };
            total_sim += sim;
            comparisons += 1;
        }
    }

    if comparisons == 0 {
        1.0
    } else {
        (total_sim / comparisons as f32).clamp(0.0, 1.0)
    }
}

/// Computes Semantic Diversity (S) in [0.0, 1.0]: distinct topic spread across sections.
pub fn compute_semantic_diversity(cohesion: f32) -> f32 {
    (1.0 - cohesion).clamp(0.0, 1.0)
}

/// Computes Coupling (K) in [0.0, 1.0]: external document dependencies.
pub fn compute_coupling(node: &NoteNode) -> f32 {
    let total_degree = (node.in_degree + node.out_degree) as f32;
    (total_degree / 15.0).clamp(0.0, 1.0)
}

/// Computes Reference Concentration (R) in [0.0, 1.0]: ratio targeting top section.
pub fn compute_reference_concentration(fragments: &[FragmentMetadata]) -> f32 {
    let total_inbound: u32 = fragments.iter().map(|f| f.inbound).sum();
    if total_inbound == 0 {
        return 0.0;
    }
    let max_inbound = fragments.iter().map(|f| f.inbound).max().unwrap_or(0);
    (max_inbound as f32 / total_inbound as f32).clamp(0.0, 1.0)
}

/// Computes Granularity (G) in [0.0, 1.0]: ratio of actual fragment token count to optimal target.
pub fn compute_granularity(node: &NoteNode, fragments: &[FragmentMetadata], fragment_target: u32) -> f32 {
    if fragment_target == 0 {
        return 1.0;
    }
    let fragment_count = fragments.len().max(1);
    let avg_tokens = (node.tokens / fragment_count as u32) as f32;
    (avg_tokens / fragment_target as f32).clamp(0.0, 1.0)
}

/// Computes Reusability (U) in [0.0, 1.0]: frequency of section-level citations without full loading.
pub fn compute_reusability(node: &NoteNode, fragments: &[FragmentMetadata]) -> f32 {
    let fragment_inbound: u32 = fragments.iter().map(|f| f.inbound).sum();
    let total_references = node.in_degree + fragment_inbound;
    if total_references == 0 {
        0.0
    } else {
        (fragment_inbound as f32 / total_references as f32).clamp(0.0, 1.0)
    }
}

/// Computes Context Efficiency (E) in [0.0, 1.0]: relevant tokens retrieved vs total traversed.
pub fn compute_context_efficiency(node: &NoteNode, fragments: &[FragmentMetadata]) -> f32 {
    if node.tokens == 0 {
        return 1.0;
    }
    let reusability = compute_reusability(node, fragments);
    if fragments.is_empty() {
        // Full document load baseline efficiency
        (500.0 / (node.tokens as f32).max(500.0)).clamp(0.05, 1.0)
    } else {
        let avg_section_tokens = (node.tokens / fragments.len() as u32) as f32;
        let addressed_ratio = (avg_section_tokens / node.tokens as f32).clamp(0.05, 1.0);
        let unaddressed_ratio = (500.0 / (node.tokens as f32).max(500.0)).clamp(0.05, 1.0);
        (reusability * (1.0 - addressed_ratio) + (1.0 - reusability) * unaddressed_ratio).clamp(0.0, 1.0)
    }
}

/// Computes Discoverability (D) in [0.0, 1.0]: reachability score from entry hubs.
pub fn compute_discoverability(node: &NoteNode) -> f32 {
    if node.hop_distance < 0 {
        0.0 // Orphan
    } else if node.hop_distance == 0 {
        1.0 // Hub
    } else {
        (1.0 / (1.0 + 0.5 * node.hop_distance as f32)).clamp(0.0, 1.0)
    }
}

/// Computes Navigational Complexity (N) in [0.0, 1.0]: average hop distance and branch factor.
pub fn compute_navigational_complexity(node: &NoteNode) -> f32 {
    if node.hop_distance < 0 {
        return 1.0;
    }
    let hop_component = (node.hop_distance as f32 / 6.0).clamp(0.0, 1.0);
    let branch_component = (node.out_degree as f32 / 12.0).clamp(0.0, 1.0);
    (0.5 * hop_component + 0.5 * branch_component).clamp(0.0, 1.0)
}

/// Computes Maintenance Coupling (M) in [0.0, 1.0]: number of files requiring updates when concepts shift.
pub fn compute_maintenance_coupling(node: &NoteNode) -> f32 {
    (node.in_degree as f32 / 15.0).clamp(0.0, 1.0)
}

/// Extracts all 10 structural dimensions for a single document node.
pub fn extract_metrics_for_node(node: &NoteNode, fragment_target: u32) -> StructuralMetrics {
    let cohesion = compute_cohesion(&node.fragments);
    let semantic_diversity = compute_semantic_diversity(cohesion);
    let coupling = compute_coupling(node);
    let reference_concentration = compute_reference_concentration(&node.fragments);
    let granularity = compute_granularity(node, &node.fragments, fragment_target);
    let reusability = compute_reusability(node, &node.fragments);
    let context_efficiency = compute_context_efficiency(node, &node.fragments);
    let discoverability = compute_discoverability(node);
    let navigational_complexity = compute_navigational_complexity(node);
    let maintenance_coupling = compute_maintenance_coupling(node);

    StructuralMetrics {
        cohesion,
        coupling,
        reference_concentration,
        granularity,
        context_efficiency,
        discoverability,
        reusability,
        semantic_diversity,
        navigational_complexity,
        maintenance_coupling,
    }
}

/// Parallel multi-file structural metric extractor using Rayon (NFR-001, NFR-003, T023).
pub fn extract_metrics_parallel(nodes: &[NoteNode], fragment_target: u32) -> Vec<StructuralMetrics> {
    nodes
        .par_iter()
        .map(|node| extract_metrics_for_node(node, fragment_target))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_structural_metrics_struct() {
        let metrics = StructuralMetrics {
            cohesion: 0.85,
            coupling: 0.20,
            reference_concentration: 0.25,
            granularity: 0.90,
            context_efficiency: 0.75,
            discoverability: 0.80,
            reusability: 0.70,
            semantic_diversity: 0.15,
            navigational_complexity: 0.30,
            maintenance_coupling: 0.20,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let de: StructuralMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(de.cohesion, 0.85);
        assert_eq!(de.semantic_diversity, 0.15);
    }

    #[test]
    fn test_metrics_cohesion_diversity_coupling() {
        // High cohesion: fragments share concepts
        let cohesive_frags = vec![
            FragmentMetadata {
                anchor: "auth_token".to_string(),
                heading: "Authentication Token Generation".to_string(),
                start_line: 1,
                end_line: 20,
                tokens: 500,
                inbound: 2,
                outbound: 1,
            },
            FragmentMetadata {
                anchor: "auth_verify".to_string(),
                heading: "Authentication Token Verification".to_string(),
                start_line: 21,
                end_line: 40,
                tokens: 500,
                inbound: 1,
                outbound: 1,
            },
        ];
        let c = compute_cohesion(&cohesive_frags);
        let s = compute_semantic_diversity(c);
        assert!(c >= 0.5, "Cohesive fragments should yield cohesion >= 0.5, got {}", c);
        assert!(s <= 0.5, "Cohesive fragments should yield diversity <= 0.5, got {}", s);

        // Low cohesion / High diversity: unrelated topics
        let diverse_frags = vec![
            FragmentMetadata {
                anchor: "database".to_string(),
                heading: "PostgreSQL Database Schema".to_string(),
                start_line: 1,
                end_line: 20,
                tokens: 500,
                inbound: 0,
                outbound: 0,
            },
            FragmentMetadata {
                anchor: "css".to_string(),
                heading: "Frontend Styling CSS".to_string(),
                start_line: 21,
                end_line: 40,
                tokens: 500,
                inbound: 0,
                outbound: 0,
            },
        ];
        let c_div = compute_cohesion(&diverse_frags);
        let s_div = compute_semantic_diversity(c_div);
        assert!(c_div < c, "Diverse fragments should have lower cohesion");
        assert!(s_div > s, "Diverse fragments should have higher diversity");

        // Coupling
        let mut node = NoteNode::new("test.md", "/path/test.md", "Test");
        node.in_degree = 5;
        node.out_degree = 10;
        let k = compute_coupling(&node);
        assert_eq!(k, 1.0); // (5 + 10) / 15.0 = 1.0

        node.in_degree = 1;
        node.out_degree = 2;
        let k_low = compute_coupling(&node);
        assert_eq!(k_low, 0.2); // (1 + 2) / 15.0 = 0.2
    }

    #[test]
    fn test_metrics_concentration_granularity_reusability() {
        let frags = vec![
            FragmentMetadata {
                anchor: "sec1".to_string(),
                heading: "Section 1".to_string(),
                start_line: 1,
                end_line: 10,
                tokens: 500,
                inbound: 9,
                outbound: 0,
            },
            FragmentMetadata {
                anchor: "sec2".to_string(),
                heading: "Section 2".to_string(),
                start_line: 11,
                end_line: 20,
                tokens: 500,
                inbound: 1,
                outbound: 0,
            },
        ];

        // Reference Concentration: 9 / (9 + 1) = 0.90
        let r = compute_reference_concentration(&frags);
        assert_eq!(r, 0.90);

        // Granularity: avg 500 tokens / 3000 target = ~0.1667
        let mut node = NoteNode::new("test.md", "/path/test.md", "Test");
        node.tokens = 1000;
        let g = compute_granularity(&node, &frags, 3000);
        assert!((g - (500.0 / 3000.0)).abs() < 1e-4);

        // Reusability: 10 fragment inbound / (0 node inbound + 10 fragment inbound) = 1.0
        let u = compute_reusability(&node, &frags);
        assert_eq!(u, 1.0);
    }

    #[test]
    fn test_metrics_efficiency_discoverability() {
        let mut node = NoteNode::new("test.md", "/path/test.md", "Test");
        node.tokens = 2000;
        node.hop_distance = 0; // Hub
        let d_hub = compute_discoverability(&node);
        assert_eq!(d_hub, 1.0);

        node.hop_distance = 2;
        let d_hop2 = compute_discoverability(&node);
        assert_eq!(d_hop2, 0.5); // 1.0 / (1.0 + 0.5 * 2) = 0.5

        node.hop_distance = -1;
        let d_orphan = compute_discoverability(&node);
        assert_eq!(d_orphan, 0.0);

        let eff = compute_context_efficiency(&node, &[]);
        assert!(eff > 0.0 && eff <= 1.0);
    }

    #[test]
    fn test_metrics_navigational_and_maintenance() {
        let mut node = NoteNode::new("test.md", "/path/test.md", "Test");
        node.hop_distance = 3;
        node.out_degree = 6;
        node.in_degree = 9;

        let n = compute_navigational_complexity(&node);
        // 0.5 * (3 / 6) + 0.5 * (6 / 12) = 0.25 + 0.25 = 0.50
        assert_eq!(n, 0.50);

        let m = compute_maintenance_coupling(&node);
        // 9 / 15 = 0.60
        assert_eq!(m, 0.60);
    }

    #[test]
    fn test_parallel_10d_metrics_extraction() {
        let mut nodes = Vec::with_capacity(1000);
        for i in 0..1000 {
            let mut node = NoteNode::new(
                format!("doc_{}.md", i),
                format!("/path/doc_{}.md", i),
                format!("Doc {}", i),
            );
            node.tokens = 1500 + (i % 500) as u32;
            node.in_degree = (i % 10) as u32;
            node.out_degree = (i % 8) as u32;
            node.hop_distance = (i % 4) as i32;
            node.fragments = vec![
                FragmentMetadata {
                    anchor: "sec1".to_string(),
                    heading: "Overview".to_string(),
                    start_line: 1,
                    end_line: 20,
                    tokens: 500,
                    inbound: 2,
                    outbound: 1,
                },
                FragmentMetadata {
                    anchor: "sec2".to_string(),
                    heading: "Details".to_string(),
                    start_line: 21,
                    end_line: 50,
                    tokens: 1000,
                    inbound: 1,
                    outbound: 0,
                },
            ];
            nodes.push(node);
        }

        let start = std::time::Instant::now();
        let results = extract_metrics_parallel(&nodes, 3000);
        let elapsed = start.elapsed();

        assert_eq!(results.len(), 1000);
        assert!(
            elapsed <= std::time::Duration::from_millis(150),
            "Parallel extraction for 1000 files must complete in <= 150ms (NFR-003), took {:?}",
            elapsed
        );
    }

    #[test]
    fn test_raw_numeric_integrity_metric_values() {
        let mut node = NoteNode::new("test.md", "/path/test.md", "Test");
        node.tokens = 3333;
        node.in_degree = 7;
        node.out_degree = 5;
        node.hop_distance = 1;

        let metrics = extract_metrics_for_node(&node, 5000);

        // Verify values are within [0.0, 1.0] and retain floating point precision
        assert!(metrics.cohesion >= 0.0 && metrics.cohesion <= 1.0);
        assert!(metrics.coupling >= 0.0 && metrics.coupling <= 1.0);
        assert!(metrics.reference_concentration >= 0.0 && metrics.reference_concentration <= 1.0);
        assert!(metrics.granularity >= 0.0 && metrics.granularity <= 1.0);
        assert!(metrics.context_efficiency >= 0.0 && metrics.context_efficiency <= 1.0);
        assert!(metrics.discoverability >= 0.0 && metrics.discoverability <= 1.0);
        assert!(metrics.reusability >= 0.0 && metrics.reusability <= 1.0);
        assert!(metrics.semantic_diversity >= 0.0 && metrics.semantic_diversity <= 1.0);
        assert!(metrics.navigational_complexity >= 0.0 && metrics.navigational_complexity <= 1.0);
        assert!(metrics.maintenance_coupling >= 0.0 && metrics.maintenance_coupling <= 1.0);

        // Invariant IX: Complete raw values preserved across JSON roundtrip without truncation
        let json = serde_json::to_string(&metrics).unwrap();
        let de: StructuralMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(de, metrics);
    }

    #[test]
    fn test_raw_numeric_integrity_invariants() {
        test_raw_numeric_integrity_metric_values();
    }
}

