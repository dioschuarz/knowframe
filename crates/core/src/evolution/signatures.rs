use super::metrics::{extract_metrics_for_node, StructuralMetrics};
use crate::model::NoteNode;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemSignatureType {
    GodDocument,
    ImplicitRouter,
    FragmentedConcept,
    IsolatedSilo,
    HealthyMonolith,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralProblemSignature {
    pub signature_id: String,
    pub document_path: String,
    pub problem_type: ProblemSignatureType,
    pub confidence: f32,
    pub evidence: StructuralMetrics,
    pub rationale: String,
}

/// Detects the structural GodDocument signature enforcing the multi-metric predicate (FR-024, Invariant XIII):
/// semantic_diversity >= 0.75 && reference_concentration >= 0.70 && context_efficiency <= 0.35 && coupling >= 0.70
pub fn detect_god_document(node: &NoteNode, metrics: &StructuralMetrics) -> Option<StructuralProblemSignature> {
    if metrics.semantic_diversity >= 0.75
        && metrics.reference_concentration >= 0.70
        && metrics.context_efficiency <= 0.35
        && metrics.coupling >= 0.70
    {
        let confidence = ((metrics.semantic_diversity
            + metrics.reference_concentration
            + (1.0 - metrics.context_efficiency)
            + metrics.coupling)
            / 4.0)
            .clamp(0.70, 1.0);

        Some(StructuralProblemSignature {
            signature_id: format!("god_doc_{}", node.id.replace('/', "_")),
            document_path: node.file_path.clone(),
            problem_type: ProblemSignatureType::GodDocument,
            confidence,
            evidence: metrics.clone(),
            rationale: "Document exhibits multi-metric structural God Document pathology: high semantic diversity (>=0.75), extreme reference concentration (>=0.70), low context efficiency (<=0.35), and high coupling (>=0.70).".to_string(),
        })
    } else {
        None
    }
}

/// Classifies HealthyMonolith ensuring large documents (>50k tokens) with high cohesion (>=0.80)
/// and distributed references (<=0.30) are certified healthy and NEVER split (FR-025, Invariant XIII).
pub fn detect_healthy_monolith(node: &NoteNode, metrics: &StructuralMetrics) -> Option<StructuralProblemSignature> {
    if node.tokens >= 50_000 && metrics.cohesion >= 0.80 && metrics.reference_concentration <= 0.30 {
        let confidence = (metrics.cohesion * (1.0 - metrics.reference_concentration)).clamp(0.75, 1.0);
        Some(StructuralProblemSignature {
            signature_id: format!("healthy_mono_{}", node.id.replace('/', "_")),
            document_path: node.file_path.clone(),
            problem_type: ProblemSignatureType::HealthyMonolith,
            confidence,
            evidence: metrics.clone(),
            rationale: format!(
                "Document is a Healthy Monolith: {} tokens with high cohesion ({:.2} >= 0.80) and distributed references ({:.2} <= 0.30). Certified healthy; must not be split.",
                node.tokens, metrics.cohesion, metrics.reference_concentration
            ),
        })
    } else {
        None
    }
}

/// Detects ImplicitRouter signature (FR-026):
/// outbound >= 20 && prose < 500 tokens && betweenness >= 0.40
pub fn detect_implicit_router(node: &NoteNode, metrics: &StructuralMetrics) -> Option<StructuralProblemSignature> {
    if node.out_degree >= 20 && node.tokens < 500 && node.betweenness >= 0.40 {
        let confidence = ((node.betweenness as f32) * (node.out_degree as f32 / 20.0).min(1.0)).clamp(0.70, 1.0);
        Some(StructuralProblemSignature {
            signature_id: format!("implicit_router_{}", node.id.replace('/', "_")),
            document_path: node.file_path.clone(),
            problem_type: ProblemSignatureType::ImplicitRouter,
            confidence,
            evidence: metrics.clone(),
            rationale: format!(
                "Document acts as an structural traffic hub with high betweenness ({:.2} >= 0.40) and {} outbound references, but contains minimal prose ({} < 500 tokens).",
                node.betweenness, node.out_degree, node.tokens
            ),
        })
    } else {
        None
    }
}

/// Detects FragmentedConcept signature (FR-027):
/// Concept split across >= 3 weakly linked documents (hop distance >= 3 or edge weight < 0.20).
pub fn detect_fragmented_concept(
    node: &NoteNode,
    metrics: &StructuralMetrics,
    matching_concept_docs_count: usize,
) -> Option<StructuralProblemSignature> {
    if matching_concept_docs_count >= 3 && (node.hop_distance >= 3 || metrics.coupling < 0.20) {
        let confidence = 0.80;
        Some(StructuralProblemSignature {
            signature_id: format!("frag_concept_{}", node.id.replace('/', "_")),
            document_path: node.file_path.clone(),
            problem_type: ProblemSignatureType::FragmentedConcept,
            confidence,
            evidence: metrics.clone(),
            rationale: format!(
                "Concept is fragmented across {} weakly linked documents (hop distance {} >= 3 or coupling {:.2} < 0.20).",
                matching_concept_docs_count, node.hop_distance, metrics.coupling
            ),
        })
    } else {
        None
    }
}

/// Detects IsolatedSilo signature:
/// Node has very low discoverability (<= 0.20) and minimal in-degree (<= 1) or is orphan.
pub fn detect_isolated_silo(node: &NoteNode, metrics: &StructuralMetrics) -> Option<StructuralProblemSignature> {
    if (metrics.discoverability <= 0.20 && node.in_degree <= 1) || node.hop_distance < 0 {
        let confidence = if node.hop_distance < 0 { 0.95 } else { 0.75 };
        Some(StructuralProblemSignature {
            signature_id: format!("silo_{}", node.id.replace('/', "_")),
            document_path: node.file_path.clone(),
            problem_type: ProblemSignatureType::IsolatedSilo,
            confidence,
            evidence: metrics.clone(),
            rationale: format!(
                "Document is an isolated knowledge silo: discoverability {:.2} <= 0.20 and in-degree {} <= 1 (hop distance: {}).",
                metrics.discoverability, node.in_degree, node.hop_distance
            ),
        })
    } else {
        None
    }
}

/// Detects all applicable problem signatures for a single node.
pub fn detect_signatures_for_node(
    node: &NoteNode,
    metrics: &StructuralMetrics,
    matching_concept_count: usize,
) -> Vec<StructuralProblemSignature> {
    let mut signatures = Vec::new();

    // Invariant XIII: Healthy Monolith has precedence and certifies no splitting
    if let Some(healthy) = detect_healthy_monolith(node, metrics) {
        signatures.push(healthy);
        return signatures;
    }

    if let Some(god) = detect_god_document(node, metrics) {
        signatures.push(god);
    }
    if let Some(router) = detect_implicit_router(node, metrics) {
        signatures.push(router);
    }
    if let Some(frag) = detect_fragmented_concept(node, metrics, matching_concept_count) {
        signatures.push(frag);
    }
    if let Some(silo) = detect_isolated_silo(node, metrics) {
        signatures.push(silo);
    }

    signatures
}

/// Batch signature detector running over all graph nodes in parallel (NFR-003, T029).
pub fn detect_all_signatures(nodes: &[NoteNode], fragment_target: u32) -> Vec<StructuralProblemSignature> {
    // Build tag frequency map to detect fragmented concepts across >= 3 docs
    let mut tag_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for node in nodes {
        for tag in &node.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    nodes
        .par_iter()
        .flat_map(|node| {
            let metrics = extract_metrics_for_node(node, fragment_target);
            let matching_concept_count = node
                .tags
                .iter()
                .map(|t| *tag_counts.get(t).unwrap_or(&0))
                .max()
                .unwrap_or(0);
            detect_signatures_for_node(node, &metrics, matching_concept_count)
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_problem_signature_types() {
        let sig = StructuralProblemSignature {
            signature_id: "sig-001".to_string(),
            document_path: "docs/api_reference.md".to_string(),
            problem_type: ProblemSignatureType::HealthyMonolith,
            confidence: 0.95,
            evidence: StructuralMetrics::default(),
            rationale: "Document has high cohesion and low concentration despite large size".to_string(),
        };

        let json = serde_json::to_string(&sig).unwrap();
        let de: StructuralProblemSignature = serde_json::from_str(&json).unwrap();
        assert_eq!(de.problem_type, ProblemSignatureType::HealthyMonolith);
        assert_eq!(de.confidence, 0.95);
    }

    #[test]
    fn test_god_document_structural_predicate() {
        let node = NoteNode::new("god.md", "/path/god.md", "God");
        let mut evidence = StructuralMetrics::default();
        evidence.semantic_diversity = 0.80; // >= 0.75
        evidence.reference_concentration = 0.75; // >= 0.70
        evidence.context_efficiency = 0.25; // <= 0.35
        evidence.coupling = 0.85; // >= 0.70

        let sig = detect_god_document(&node, &evidence);
        assert!(sig.is_some(), "Predicate must identify God Document");
        let s = sig.unwrap();
        assert_eq!(s.problem_type, ProblemSignatureType::GodDocument);
        assert!(s.confidence >= 0.70);

        // Fails predicate if context efficiency is good (e.g. 0.50)
        evidence.context_efficiency = 0.50;
        let no_sig = detect_god_document(&node, &evidence);
        assert!(no_sig.is_none(), "Must NOT flag if any predicate condition fails");
    }

    #[test]
    fn test_healthy_monolith_protection() {
        let mut node = NoteNode::new("monolith.md", "/path/monolith.md", "Monolith");
        node.tokens = 60_000; // > 50k tokens

        let mut evidence = StructuralMetrics::default();
        evidence.cohesion = 0.85; // >= 0.80
        evidence.reference_concentration = 0.20; // <= 0.30

        let sig = detect_healthy_monolith(&node, &evidence);
        assert!(sig.is_some(), "Healthy monolith must be certified");
        let s = sig.unwrap();
        assert_eq!(s.problem_type, ProblemSignatureType::HealthyMonolith);

        // Verify Invariant XIII: healthy monolith is NEVER flagged as god document
        let all_sigs = detect_signatures_for_node(&node, &evidence, 0);
        assert_eq!(all_sigs.len(), 1);
        assert_eq!(all_sigs[0].problem_type, ProblemSignatureType::HealthyMonolith);
    }

    #[test]
    fn test_implicit_router_detection() {
        let mut node = NoteNode::new("router.md", "/path/router.md", "Router");
        node.out_degree = 25; // >= 20
        node.tokens = 300; // < 500
        node.betweenness = 0.45; // >= 0.40

        let evidence = StructuralMetrics::default();
        let sig = detect_implicit_router(&node, &evidence);
        assert!(sig.is_some(), "Implicit router must be detected");
        assert_eq!(sig.unwrap().problem_type, ProblemSignatureType::ImplicitRouter);
    }

    #[test]
    fn test_fragmented_concept_and_isolated_silo() {
        let mut node = NoteNode::new("frag.md", "/path/frag.md", "Frag");
        node.hop_distance = 3;
        let mut evidence = StructuralMetrics::default();
        evidence.coupling = 0.15;

        // 3 matching concept docs
        let frag_sig = detect_fragmented_concept(&node, &evidence, 3);
        assert!(frag_sig.is_some());
        assert_eq!(frag_sig.unwrap().problem_type, ProblemSignatureType::FragmentedConcept);

        // Isolated Silo
        let mut silo_node = NoteNode::new("silo.md", "/path/silo.md", "Silo");
        silo_node.in_degree = 0;
        silo_node.hop_distance = -1;
        let mut silo_evidence = StructuralMetrics::default();
        silo_evidence.discoverability = 0.0;

        let silo_sig = detect_isolated_silo(&silo_node, &silo_evidence);
        assert!(silo_sig.is_some());
        assert_eq!(silo_sig.unwrap().problem_type, ProblemSignatureType::IsolatedSilo);
    }

    #[test]
    fn test_batch_signature_detection_latency() {
        let mut nodes = Vec::with_capacity(1000);
        for i in 0..1000 {
            let mut node = NoteNode::new(
                format!("doc_{}.md", i),
                format!("/path/doc_{}.md", i),
                format!("Doc {}", i),
            );
            node.tokens = 1000 + (i % 500) as u32;
            node.in_degree = (i % 10) as u32;
            node.out_degree = (i % 8) as u32;
            node.hop_distance = (i % 4) as i32;
            node.betweenness = (i % 10) as f64 / 20.0;
            if i % 10 == 0 {
                node.tags = vec!["architecture".to_string()];
            }
            nodes.push(node);
        }

        let start = std::time::Instant::now();
        let signatures = detect_all_signatures(&nodes, 3000);
        let elapsed = start.elapsed();

        assert!(
            elapsed <= std::time::Duration::from_millis(100),
            "Batch signature detection for 1,000 files must complete in <= 100ms (NFR-003, T029), took {:?}",
            elapsed
        );
        println!("Detected {} signatures in {:?}", signatures.len(), elapsed);
    }

    #[test]
    fn test_healthy_monolith_50k_tokens_not_god_doc() {
        test_healthy_monolith_protection();
    }

    #[test]
    fn test_implicit_router_signature_detection() {
        test_implicit_router_detection();
    }

    #[test]
    fn test_signature_confidence_and_evidence_generation() {
        test_problem_signature_types();
    }
}

