use crate::collision::SemanticCollision;
use crate::debt::baseline::{diff_against_baseline, extract_findings_from_topology};
use crate::debt::{CollisionSignature, DriftSignature, KnowledgeDebtBaseline};
use crate::drift::TechnicalDriftFinding;
use crate::graph::builder::GraphTopology;
use crate::quality::{KnowledgeIssue, KnowledgeQualityScore, QualityDegradationFactor};

/// Calculates betweenness centrality topological defect weight multiplier:
/// W(v) = 1.0 + 3.0 * B(v), range [1.0, 4.0]
pub fn calculate_topological_weight(betweenness: f64) -> f32 {
    let b = (betweenness as f32).clamp(0.0, 1.0);
    1.0 + 3.0 * b
}

/// Evaluates 4-pillar Knowledge Quality Score from in-memory GraphTopology.
pub fn calculate_quality_score(
    topology: &GraphTopology,
    baseline: Option<&KnowledgeDebtBaseline>,
    collisions: &[SemanticCollision],
    drifts: &[TechnicalDriftFinding],
) -> KnowledgeQualityScore {
    let mut degradation_factors: Vec<QualityDegradationFactor> = Vec::new();

    // 1. Pillar: Integrity (30%)
    let mut integrity_deductions = 0.0f32;
    for edge in &topology.edges {
        if edge.is_broken {
            let b = topology
                .nodes
                .get(&edge.source)
                .map(|n| n.betweenness)
                .unwrap_or(0.0);
            let weight = calculate_topological_weight(b);

            let penalty = 5.0 * weight;
            integrity_deductions += penalty;

            degradation_factors.push(QualityDegradationFactor {
                category: "broken_link".to_string(),
                affected_node: edge.source.clone(),
                betweenness_weight: weight,
                deduction_points: penalty,
                description: format!("Broken link referencing missing target '{}'", edge.target),
            });
        }
    }
    let integrity_score = (100.0f32 - integrity_deductions).clamp(0.0, 100.0);

    // 2. Pillar: Consistency (25%)
    let mut consistency_deductions = 0.0f32;
    for c in collisions {
        let b = topology
            .nodes
            .get(&c.target_id)
            .or_else(|| topology.nodes.get(&c.source_id))
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);

        let base_penalty = match c.classification {
            crate::collision::CollisionClassification::PotentialDuplicate => 10.0,
            crate::collision::CollisionClassification::PotentialConflict => 8.0,
            crate::collision::CollisionClassification::PotentialFragmentation => 4.0,
            crate::collision::CollisionClassification::PotentialSpecialization => 2.0,
            crate::collision::CollisionClassification::PotentialSummary => 2.0,
        };
        let penalty = base_penalty * weight;
        consistency_deductions += penalty;

        degradation_factors.push(QualityDegradationFactor {
            category: format!("collision_{:?}", c.classification).to_lowercase(),
            affected_node: c.source_id.clone(),
            betweenness_weight: weight,
            deduction_points: penalty,
            description: format!(
                "Collision with '{}': {:.1}% similarity ({:?})",
                c.target_id,
                c.composite_similarity * 100.0,
                c.classification
            ),
        });
    }
    let consistency_score = (100.0f32 - consistency_deductions).clamp(0.0, 100.0);

    // 3. Pillar: Discoverability (25%)
    let mut discoverability_deductions = 0.0f32;
    for (id, node) in &topology.nodes {
        if node.in_degree == 0 && node.out_degree == 0 {
            // Orphan node
            let penalty = 8.0;
            discoverability_deductions += penalty;
            degradation_factors.push(QualityDegradationFactor {
                category: "orphan_node".to_string(),
                affected_node: id.clone(),
                betweenness_weight: 1.0,
                deduction_points: penalty,
                description: "Isolated orphan node without incoming or outgoing links".to_string(),
            });
        } else if node.hop_distance > 3 {
            let weight = calculate_topological_weight(node.betweenness);
            let penalty = 3.0 * (node.hop_distance - 3) as f32 * weight;
            discoverability_deductions += penalty;
            degradation_factors.push(QualityDegradationFactor {
                category: "high_hop_distance".to_string(),
                affected_node: id.clone(),
                betweenness_weight: weight,
                deduction_points: penalty,
                description: format!("Node is {} hops away from designated central hub", node.hop_distance),
            });
        } else if node.hop_distance == -1 {
            // Disconnected island
            let penalty = 5.0;
            discoverability_deductions += penalty;
            degradation_factors.push(QualityDegradationFactor {
                category: "disconnected_subgraph".to_string(),
                affected_node: id.clone(),
                betweenness_weight: 1.0,
                deduction_points: penalty,
                description: "Node is part of a disconnected subgraph unreachable from hub".to_string(),
            });
        }
    }
    let discoverability_score = (100.0f32 - discoverability_deductions).clamp(0.0, 100.0);

    // 4. Pillar: Freshness (20%)
    let mut freshness_deductions = 0.0f32;
    for d in drifts {
        let b = topology
            .nodes
            .get(&d.doc_id)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let penalty = 8.0 * weight;
        freshness_deductions += penalty;

        degradation_factors.push(QualityDegradationFactor {
            category: "technical_drift".to_string(),
            affected_node: d.doc_id.clone(),
            betweenness_weight: weight,
            deduction_points: penalty,
            description: format!("Cites modified/deleted code '{}' ({})", d.code_target, d.explanation),
        });
    }

    for (path, node) in &topology.nodes {
        if let Some(ref f) = node.freshness {
            let weight = calculate_topological_weight(node.betweenness);
            for missing in &f.missing_citations {
                let penalty = 15.0 * weight;
                freshness_deductions += penalty;
                degradation_factors.push(QualityDegradationFactor {
                    category: "missing_code_citation".to_string(),
                    affected_node: path.clone(),
                    betweenness_weight: weight,
                    deduction_points: penalty,
                    description: format!("Cites non-existent code file '{}'", missing),
                });
            }
            if !f.code_modified_after_doc.is_empty() {
                let penalty = 10.0 * weight;
                freshness_deductions += penalty;
                degradation_factors.push(QualityDegradationFactor {
                    category: "code_modified_after_doc".to_string(),
                    affected_node: path.clone(),
                    betweenness_weight: weight,
                    deduction_points: penalty,
                    description: format!("Cited code files {:?} modified after document commit", f.code_modified_after_doc),
                });
            }
        }
    }
    let freshness_score = (100.0f32 - freshness_deductions).clamp(0.0, 100.0);

    // Pillar 2: Structure
    let structure_findings = crate::quality::structure::detect_structure_debt(topology);
    let structure_score = crate::quality::structure::calculate_structure_score(&structure_findings, topology);

    for sf in &structure_findings {
        let b = topology
            .get_node(&sf.document_path)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let penalty = sf.penalty * weight;
        degradation_factors.push(QualityDegradationFactor {
            category: "structure_debt".to_string(),
            affected_node: sf.document_path.clone(),
            betweenness_weight: weight,
            deduction_points: penalty,
            description: sf.guidance.clone(),
        });
    }

    // 5-pillar composite calculation: 25% / 20% / 20% / 20% / 15%
    let overall_score = 0.25 * integrity_score
        + 0.20 * structure_score
        + 0.20 * consistency_score
        + 0.20 * discoverability_score
        + 0.15 * freshness_score;
    let overall_score = overall_score.clamp(0.0, 100.0);

    // Top degradation factors: sort descending by deduction_points
    degradation_factors.sort_by(|a, b| {
        b.deduction_points
            .partial_cmp(&a.deduction_points)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_degradation_factors = degradation_factors.into_iter().take(5).collect();

    // Baseline debt accounting integration
    let (legacy_debt_count, new_regressions_count, resolved_debt_count) = if let Some(base) = baseline {
        let col_sigs: Vec<CollisionSignature> = collisions
            .iter()
            .map(|c| CollisionSignature::new(&c.source_id, &c.target_id))
            .collect();
        let drift_sigs: Vec<DriftSignature> = drifts
            .iter()
            .map(|d| DriftSignature {
                doc_path: d.doc_id.clone(),
                referenced_code_path: d.code_target.clone(),
                symbol: d.symbol_name.clone().unwrap_or_default(),
            })
            .collect();
        let findings = extract_findings_from_topology(topology, col_sigs, drift_sigs);
        let diff = diff_against_baseline(&findings, base);

        let new_cnt = (diff.new_broken_links.len()
            + diff.new_orphans.len()
            + diff.new_collisions.len()
            + diff.new_technical_drifts.len()) as u32;
        let res_cnt = (diff.resolved_broken_links.len()
            + diff.resolved_orphans.len()
            + diff.resolved_collisions.len()
            + diff.resolved_technical_drifts.len()) as u32;

        (base.total_debt, new_cnt, res_cnt)
    } else {
        (0, 0, 0)
    };

    let central_hub_id = topology.suggest_hub();

    KnowledgeQualityScore {
        overall_score: (overall_score * 10.0).round() / 10.0,
        integrity_score: (integrity_score * 10.0).round() / 10.0,
        structure_score: (structure_score * 10.0).round() / 10.0,
        consistency_score: (consistency_score * 10.0).round() / 10.0,
        discoverability_score: (discoverability_score * 10.0).round() / 10.0,
        freshness_score: (freshness_score * 10.0).round() / 10.0,
        legacy_debt_count,
        new_regressions_count,
        resolved_debt_count,
        central_hub_id,
        top_degradation_factors,
    }
}

/// Extracts full categorized list of knowledge issues across all 4 pillars.
pub fn extract_all_issues(
    topology: &GraphTopology,
    collisions: &[SemanticCollision],
    drifts: &[TechnicalDriftFinding],
) -> Vec<KnowledgeIssue> {
    let mut issues = Vec::new();

    // 1. Integrity Issues (broken references)
    for edge in &topology.edges {
        if edge.is_broken {
            let b = topology
                .nodes
                .get(&edge.source)
                .map(|n| n.betweenness)
                .unwrap_or(0.0);
            let weight = calculate_topological_weight(b);
            let deduction = (5.0 * weight * 10.0).round() / 10.0;
            issues.push(KnowledgeIssue {
                id: format!("integrity:{}:{}:{}", edge.source, edge.target, edge.line.unwrap_or(0)),
                pillar: "integrity".to_string(),
                category: "broken_link".to_string(),
                file_path: edge.source.clone(),
                line_number: edge.line,
                target: Some(edge.target.clone()),
                deduction_points: deduction,
                weight: (weight * 100.0).round() / 100.0,
                description: format!("Broken reference to missing target '{}'", edge.target),
                remediation_hint: format!("Create target file '{}' or update link in '{}'", edge.target, edge.source),
            });
        }
    }

    // 2. Consistency Issues (collisions and duplicates)
    for c in collisions {
        let b = topology
            .nodes
            .get(&c.target_id)
            .or_else(|| topology.nodes.get(&c.source_id))
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let base_penalty = match c.classification {
            crate::collision::CollisionClassification::PotentialDuplicate => 10.0,
            crate::collision::CollisionClassification::PotentialConflict => 8.0,
            crate::collision::CollisionClassification::PotentialFragmentation => 4.0,
            crate::collision::CollisionClassification::PotentialSpecialization => 2.0,
            crate::collision::CollisionClassification::PotentialSummary => 2.0,
        };
        let deduction = (base_penalty * weight * 10.0).round() / 10.0;
        issues.push(KnowledgeIssue {
            id: format!("consistency:{}:{}", c.source_id, c.target_id),
            pillar: "consistency".to_string(),
            category: format!("collision_{:?}", c.classification).to_lowercase(),
            file_path: c.source_id.clone(),
            line_number: None,
            target: Some(c.target_id.clone()),
            deduction_points: deduction,
            weight: (weight * 100.0).round() / 100.0,
            description: format!("Semantic collision with '{}': {:.1}% similarity ({:?})", c.target_id, c.composite_similarity * 100.0, c.classification),
            remediation_hint: format!("Consolidate duplicate concepts into canonical note '{}'", c.canonical_candidate.as_deref().unwrap_or(&c.target_id)),
        });
    }

    // 3. Discoverability Issues (orphans, high hop distances, disconnected subgraphs)
    for (id, node) in &topology.nodes {
        if node.in_degree == 0 && node.out_degree == 0 {
            issues.push(KnowledgeIssue {
                id: format!("discoverability:orphan:{}", id),
                pillar: "discoverability".to_string(),
                category: "orphan_node".to_string(),
                file_path: id.clone(),
                line_number: None,
                target: None,
                deduction_points: 8.0,
                weight: 1.0,
                description: "Isolated orphan note without incoming or outgoing links".to_string(),
                remediation_hint: "Add wiki links to connect this note to the knowledge graph".to_string(),
            });
        } else if node.hop_distance > 3 {
            let weight = calculate_topological_weight(node.betweenness);
            let deduction = ((3.0 * (node.hop_distance - 3) as f32 * weight) * 10.0).round() / 10.0;
            issues.push(KnowledgeIssue {
                id: format!("discoverability:high_hop:{}", id),
                pillar: "discoverability".to_string(),
                category: "high_hop_distance".to_string(),
                file_path: id.clone(),
                line_number: None,
                target: None,
                deduction_points: deduction,
                weight: (weight * 100.0).round() / 100.0,
                description: format!("Node is {} hops away from designated central hub", node.hop_distance),
                remediation_hint: "Add cross-links from higher-level hub notes to improve discovery".to_string(),
            });
        } else if node.hop_distance == -1 {
            issues.push(KnowledgeIssue {
                id: format!("discoverability:disconnected:{}", id),
                pillar: "discoverability".to_string(),
                category: "disconnected_subgraph".to_string(),
                file_path: id.clone(),
                line_number: None,
                target: None,
                deduction_points: 5.0,
                weight: 1.0,
                description: "Node is part of a disconnected subgraph unreachable from hub".to_string(),
                remediation_hint: "Connect this cluster to the primary knowledge graph".to_string(),
            });
        }
    }

    // 4. Freshness Issues (technical drift)
    for d in drifts {
        let b = topology
            .nodes
            .get(&d.doc_id)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let deduction = (8.0 * weight * 10.0).round() / 10.0;
        issues.push(KnowledgeIssue {
            id: format!("freshness:{}:{}:{}", d.doc_id, d.code_target, d.citation_line),
            pillar: "freshness".to_string(),
            category: "technical_drift".to_string(),
            file_path: d.doc_id.clone(),
            line_number: Some(d.citation_line),
            target: Some(d.code_target.clone()),
            deduction_points: deduction,
            weight: (weight * 100.0).round() / 100.0,
            description: format!("Cites modified/deleted code '{}' ({})", d.code_target, d.explanation),
            remediation_hint: format!("Update documentation to reflect changes in '{}'", d.code_target),
        });
    }

    for (path, node) in &topology.nodes {
        if let Some(ref f) = node.freshness {
            let weight = calculate_topological_weight(node.betweenness);
            for missing in &f.missing_citations {
                let deduction = (15.0 * weight * 10.0).round() / 10.0;
                issues.push(KnowledgeIssue {
                    id: format!("freshness:missing:{}:{}", path, missing),
                    pillar: "freshness".to_string(),
                    category: "missing_code_citation".to_string(),
                    file_path: path.clone(),
                    line_number: None,
                    target: Some(missing.clone()),
                    deduction_points: deduction,
                    weight: (weight * 100.0).round() / 100.0,
                    description: format!("Cites code file '{}' which does not exist on disk", missing),
                    remediation_hint: format!("Create source file '{}' or update code citation in '{}'", missing, path),
                });
            }
            for mod_code in &f.code_modified_after_doc {
                let deduction = (10.0 * weight * 10.0).round() / 10.0;
                issues.push(KnowledgeIssue {
                    id: format!("freshness:modified:{}:{}", path, mod_code),
                    pillar: "freshness".to_string(),
                    category: "code_modified_after_doc".to_string(),
                    file_path: path.clone(),
                    line_number: None,
                    target: Some(mod_code.clone()),
                    deduction_points: deduction,
                    weight: (weight * 100.0).round() / 100.0,
                    description: format!("Cited code '{}' was modified in Git after document's last commit", mod_code),
                    remediation_hint: format!("Review recent code changes in '{}' and update documentation accordingly", mod_code),
                });
            }
        }
    }

    // 5. Structure Issues (Pillar 2)
    let structure_findings = crate::quality::structure::detect_structure_debt(topology);
    for sf in structure_findings {
        let b = topology
            .nodes
            .get(&sf.document_path)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let deduction = (sf.penalty * weight * 10.0).round() / 10.0;
        issues.push(KnowledgeIssue {
            id: format!("structure:{}:{:?}", sf.document_path, sf.debt_type),
            pillar: "structure".to_string(),
            category: format!("{:?}", sf.debt_type).to_lowercase(),
            file_path: sf.document_path.clone(),
            line_number: None,
            target: sf.fragment_anchor.clone(),
            deduction_points: deduction,
            weight: (weight * 100.0).round() / 100.0,
            description: sf.guidance.clone(),
            remediation_hint: "Restructure document or materialize fragment metadata with 'llm-wiki-core materialize'".to_string(),
        });
    }

    // Sort issues by deduction points descending
    issues.sort_by(|a, b| b.deduction_points.partial_cmp(&a.deduction_points).unwrap_or(std::cmp::Ordering::Equal));

    issues
}

