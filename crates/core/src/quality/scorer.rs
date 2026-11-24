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
    let mut raw_degradation_factors: Vec<QualityDegradationFactor> = Vec::new();
    let mut eff_degradation_factors: Vec<QualityDegradationFactor> = Vec::new();

    // 1. Pillar: Integrity (25%)
    let mut raw_integrity_deductions = 0.0f32;
    let mut eff_integrity_deductions = 0.0f32;
    let mut raw_non_clickable_deductions = 0.0f32;
    let mut eff_non_clickable_deductions = 0.0f32;
    for edge in &topology.edges {
        if edge.is_broken {
            let b = topology
                .nodes
                .get(&edge.source)
                .map(|n| n.betweenness)
                .unwrap_or(0.0);
            let weight = calculate_topological_weight(b);

            let penalty = 5.0 * weight;
            raw_integrity_deductions += penalty;

            let factor = QualityDegradationFactor {
                category: "broken_link".to_string(),
                affected_node: edge.source.clone(),
                betweenness_weight: weight,
                deduction_points: penalty,
                description: format!("Broken link referencing missing target '{}'", edge.target),
            };
            raw_degradation_factors.push(factor.clone());

            let is_legacy = baseline.map(|b| {
                b.broken_links.iter().any(|bl| bl.source_path == edge.source && bl.target_ref == edge.target)
            }).unwrap_or(false);

            if !is_legacy {
                eff_integrity_deductions += penalty;
                eff_degradation_factors.push(factor);
            }
        } else if !edge.is_clickable {
            let b = topology
                .nodes
                .get(&edge.source)
                .map(|n| n.betweenness)
                .unwrap_or(0.0);
            let weight = calculate_topological_weight(b);
            let penalty = 0.5 * weight;
            raw_non_clickable_deductions = (raw_non_clickable_deductions + penalty).min(20.0);
            eff_non_clickable_deductions = (eff_non_clickable_deductions + penalty).min(20.0);

            let reason_label = format_non_clickable_reason(edge.non_clickable_reason);
            let factor = QualityDegradationFactor {
                category: "non_clickable_link".to_string(),
                affected_node: edge.source.clone(),
                betweenness_weight: weight,
                deduction_points: penalty,
                description: format!(
                    "Link to '{}' is valid but not clickable in standard Markdown renderers ({})",
                    edge.target, reason_label
                ),
            };
            raw_degradation_factors.push(factor.clone());
            eff_degradation_factors.push(factor);
        }
    }
    raw_integrity_deductions += raw_non_clickable_deductions;
    eff_integrity_deductions += eff_non_clickable_deductions;
    let raw_integrity_score = (100.0f32 - raw_integrity_deductions).clamp(0.0, 100.0);
    let eff_integrity_score = (100.0f32 - eff_integrity_deductions).clamp(0.0, 100.0);

    // 2. Pillar: Consistency (20%)
    let mut raw_consistency_deductions = 0.0f32;
    let mut eff_consistency_deductions = 0.0f32;
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
        raw_consistency_deductions += penalty;

        let factor = QualityDegradationFactor {
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
        };
        raw_degradation_factors.push(factor.clone());

        let sig = CollisionSignature::new(&c.source_id, &c.target_id);
        let is_legacy = baseline.map(|b| b.collisions.contains(&sig)).unwrap_or(false);

        if !is_legacy {
            eff_consistency_deductions += penalty;
            eff_degradation_factors.push(factor);
        }
    }
    let raw_consistency_score = (100.0f32 - raw_consistency_deductions).clamp(0.0, 100.0);
    let eff_consistency_score = (100.0f32 - eff_consistency_deductions).clamp(0.0, 100.0);

    // 3. Pillar: Discoverability (20%)
    let mut raw_discoverability_deductions = 0.0f32;
    let mut eff_discoverability_deductions = 0.0f32;
    let mut sorted_nodes_disc: Vec<_> = topology.nodes.iter().collect();
    sorted_nodes_disc.sort_by(|a, b| a.0.cmp(b.0));
    for (id, node) in sorted_nodes_disc {
        if node.in_degree == 0 && node.out_degree == 0 {
            // Orphan node
            let penalty = 8.0;
            raw_discoverability_deductions += penalty;
            let factor = QualityDegradationFactor {
                category: "orphan_node".to_string(),
                affected_node: id.clone(),
                betweenness_weight: 1.0,
                deduction_points: penalty,
                description: "Isolated orphan node without incoming or outgoing links".to_string(),
            };
            raw_degradation_factors.push(factor.clone());

            let is_legacy = baseline.map(|b| b.orphan_nodes.contains(id)).unwrap_or(false);
            if !is_legacy {
                eff_discoverability_deductions += penalty;
                eff_degradation_factors.push(factor);
            }
        } else if node.hop_distance > 3 {
            let weight = calculate_topological_weight(node.betweenness);
            let penalty = 3.0 * (node.hop_distance - 3) as f32 * weight;
            raw_discoverability_deductions += penalty;
            let factor = QualityDegradationFactor {
                category: "high_hop_distance".to_string(),
                affected_node: id.clone(),
                betweenness_weight: weight,
                deduction_points: penalty,
                description: format!(
                    "Node is {} hops away from designated central hub",
                    node.hop_distance
                ),
            };
            raw_degradation_factors.push(factor.clone());

            let is_legacy = baseline.is_some() && baseline.map(|b| b.total_debt > 0).unwrap_or(false);
            if !is_legacy {
                eff_discoverability_deductions += penalty;
                eff_degradation_factors.push(factor);
            }
        } else if node.hop_distance == -1 {
            // Disconnected island
            let penalty = 5.0;
            raw_discoverability_deductions += penalty;
            let factor = QualityDegradationFactor {
                category: "disconnected_subgraph".to_string(),
                affected_node: id.clone(),
                betweenness_weight: 1.0,
                deduction_points: penalty,
                description: "Node is part of a disconnected subgraph unreachable from hub"
                    .to_string(),
            };
            raw_degradation_factors.push(factor.clone());

            let is_legacy = baseline.is_some() && baseline.map(|b| b.total_debt > 0).unwrap_or(false);
            if !is_legacy {
                eff_discoverability_deductions += penalty;
                eff_degradation_factors.push(factor);
            }
        }
    }
    let raw_discoverability_score = (100.0f32 - raw_discoverability_deductions).clamp(0.0, 100.0);
    let eff_discoverability_score = (100.0f32 - eff_discoverability_deductions).clamp(0.0, 100.0);

    // 4. Pillar: Freshness (15%)
    let mut raw_freshness_deductions = 0.0f32;
    let mut eff_freshness_deductions = 0.0f32;
    for d in drifts {
        let b = topology
            .nodes
            .get(&d.doc_id)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let penalty = 8.0 * weight;
        raw_freshness_deductions += penalty;

        let factor = QualityDegradationFactor {
            category: "technical_drift".to_string(),
            affected_node: d.doc_id.clone(),
            betweenness_weight: weight,
            deduction_points: penalty,
            description: format!(
                "Cites modified/deleted code '{}' ({})",
                d.code_target, d.explanation
            ),
        };
        raw_degradation_factors.push(factor.clone());

        let is_legacy = baseline.map(|b| {
            b.technical_drifts.iter().any(|td| td.doc_path == d.doc_id && td.referenced_code_path == d.code_target)
        }).unwrap_or(false);

        if !is_legacy {
            eff_freshness_deductions += penalty;
            eff_degradation_factors.push(factor);
        }
    }

    let mut sorted_nodes_fresh: Vec<_> = topology.nodes.iter().collect();
    sorted_nodes_fresh.sort_by(|a, b| a.0.cmp(b.0));
    for (path, node) in sorted_nodes_fresh {
        if let Some(ref f) = node.freshness {
            let weight = calculate_topological_weight(node.betweenness);
            for missing in &f.missing_citations {
                let penalty = 15.0 * weight;
                raw_freshness_deductions += penalty;
                let factor = QualityDegradationFactor {
                    category: "missing_code_citation".to_string(),
                    affected_node: path.clone(),
                    betweenness_weight: weight,
                    deduction_points: penalty,
                    description: format!("Cites non-existent code file '{}'", missing),
                };
                raw_degradation_factors.push(factor.clone());

                let is_legacy = baseline.map(|b| {
                    b.technical_drifts.iter().any(|td| td.doc_path == *path && td.referenced_code_path == *missing)
                }).unwrap_or(false);

                if !is_legacy {
                    eff_freshness_deductions += penalty;
                    eff_degradation_factors.push(factor);
                }
            }
            if !f.code_modified_after_doc.is_empty() {
                let penalty = 10.0 * weight;
                raw_freshness_deductions += penalty;
                let factor = QualityDegradationFactor {
                    category: "code_modified_after_doc".to_string(),
                    affected_node: path.clone(),
                    betweenness_weight: weight,
                    deduction_points: penalty,
                    description: format!(
                        "Cited code files {:?} modified after document commit",
                        f.code_modified_after_doc
                    ),
                };
                raw_degradation_factors.push(factor.clone());

                let is_legacy = baseline.is_some() && baseline.map(|b| b.total_debt > 0).unwrap_or(false);
                if !is_legacy {
                    eff_freshness_deductions += penalty;
                    eff_degradation_factors.push(factor);
                }
            }
        }
    }
    let raw_freshness_score = (100.0f32 - raw_freshness_deductions).clamp(0.0, 100.0);
    let eff_freshness_score = (100.0f32 - eff_freshness_deductions).clamp(0.0, 100.0);

    // Pillar 2: Structure
    let structure_findings = crate::quality::structure::detect_structure_debt(topology);
    let raw_structure_score =
        crate::quality::structure::calculate_structure_score(&structure_findings, topology);
    let mut eff_structure_deductions = 0.0f32;

    for sf in &structure_findings {
        let b = topology
            .get_node(&sf.document_path)
            .map(|n| n.betweenness)
            .unwrap_or(0.0);
        let weight = calculate_topological_weight(b);
        let penalty = sf.penalty * weight;
        let factor = QualityDegradationFactor {
            category: "structure_debt".to_string(),
            affected_node: sf.document_path.clone(),
            betweenness_weight: weight,
            deduction_points: penalty,
            description: sf.guidance.clone(),
        };
        raw_degradation_factors.push(factor.clone());

        let is_legacy = if sf.debt_type == crate::quality::structure::StructureDebtType::UnanchoredReference {
            baseline.is_some() && baseline.map(|b| b.total_debt > 0).unwrap_or(false)
        } else {
            false
        };
        if !is_legacy {
            eff_structure_deductions += penalty;
            eff_degradation_factors.push(factor);
        }
    }
    let eff_structure_score = (100.0f32 - eff_structure_deductions).clamp(0.0, 100.0);

    // Baseline debt accounting integration
    let (legacy_debt_count, new_regressions_count, resolved_debt_count) =
        if let Some(base) = baseline {
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

    let all_issues = extract_all_issues_with_baseline(topology, collisions, drifts, baseline);
    let mut broken_references = 0u32;
    let mut invalid_anchors = 0u32;
    for edge in &topology.edges {
        if edge.is_broken {
            if edge.target_fragment.is_some() || edge.anchor.is_some() {
                invalid_anchors += 1;
            } else {
                broken_references += 1;
            }
        }
    }
    let total_issues_count = all_issues.len() as u32;
    let active_issues_count_calc = all_issues.iter().filter(|i| !i.is_legacy).count() as u32;

    let raw_overall = (0.25 * raw_integrity_score
        + 0.20 * raw_structure_score
        + 0.20 * raw_consistency_score
        + 0.20 * raw_discoverability_score
        + 0.15 * raw_freshness_score).clamp(0.0, 100.0);

    let eff_overall = (0.25 * eff_integrity_score
        + 0.20 * eff_structure_score
        + 0.20 * eff_consistency_score
        + 0.20 * eff_discoverability_score
        + 0.15 * eff_freshness_score).clamp(0.0, 100.0);

    let (
        overall_score,
        integrity_score,
        structure_score,
        consistency_score,
        discoverability_score,
        freshness_score,
        mut degradation_factors,
        findings_count,
        raw_score,
        is_effective,
        active_issues_count,
    ) = if baseline.is_some() {
        (
            eff_overall,
            eff_integrity_score,
            eff_structure_score,
            eff_consistency_score,
            eff_discoverability_score,
            eff_freshness_score,
            eff_degradation_factors,
            active_issues_count_calc,
            Some((raw_overall * 10.0).round() / 10.0),
            true,
            active_issues_count_calc,
        )
    } else {
        (
            raw_overall,
            raw_integrity_score,
            raw_structure_score,
            raw_consistency_score,
            raw_discoverability_score,
            raw_freshness_score,
            raw_degradation_factors,
            total_issues_count,
            None,
            false,
            total_issues_count,
        )
    };

    degradation_factors.sort_by(|a, b| {
        b.deduction_points
            .partial_cmp(&a.deduction_points)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.affected_node.cmp(&b.affected_node))
            .then_with(|| a.category.cmp(&b.category))
    });
    let top_degradation_factors = degradation_factors.into_iter().take(5).collect();

    KnowledgeQualityScore {
        overall_score: (overall_score * 10.0).round() / 10.0,
        integrity_score: (integrity_score * 10.0).round() / 10.0,
        structure_score: (structure_score * 10.0).round() / 10.0,
        consistency_score: (consistency_score * 10.0).round() / 10.0,
        discoverability_score: (discoverability_score * 10.0).round() / 10.0,
        freshness_score: (freshness_score * 10.0).round() / 10.0,
        traceability_score: (freshness_score * 10.0).round() / 10.0,
        legacy_debt_count,
        new_regressions_count,
        resolved_debt_count,
        central_hub_id,
        top_degradation_factors,
        total_issues: total_issues_count,
        findings_count,
        broken_references,
        invalid_anchors,
        raw_score,
        is_effective,
        active_issues_count,
    }
}

fn format_non_clickable_reason(reason: Option<crate::model::NonClickableReason>) -> &'static str {
    match reason {
        Some(crate::model::NonClickableReason::WikilinkSyntax) => {
            "Obsidian wikilink syntax not clickable in VS Code or standard Markdown renderers"
        }
        Some(crate::model::NonClickableReason::MissingExtension) => {
            "missing explicit .md file extension"
        }
        Some(crate::model::NonClickableReason::NonRelativePath) => {
            "path is not relative to source file directory"
        }
        Some(crate::model::NonClickableReason::CaseMismatch) => {
            "path casing does not match target filename on disk"
        }
        Some(crate::model::NonClickableReason::UnencodedSpaces) => {
            "unencoded whitespace in Markdown link destination"
        }
        Some(crate::model::NonClickableReason::UnslugifiedAnchor) => {
            "fragment anchor is not a canonical GFM heading slug"
        }
        None => "non-portable Markdown link format",
    }
}

/// Extracts full categorized list of knowledge issues across all 4 pillars.
pub fn extract_all_issues(
    topology: &GraphTopology,
    collisions: &[SemanticCollision],
    drifts: &[TechnicalDriftFinding],
) -> Vec<KnowledgeIssue> {
    let mut issues = Vec::new();

    // 1. Integrity Issues (broken references + non-clickable valid links)
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
                id: format!(
                    "integrity:{}:{}:{}",
                    edge.source,
                    edge.target,
                    edge.line.unwrap_or(0)
                ),
                pillar: "integrity".to_string(),
                category: "broken_link".to_string(),
                file_path: edge.source.clone(),
                line_number: edge.line,
                target: Some(edge.target.clone()),
                deduction_points: deduction,
                weight: (weight * 100.0).round() / 100.0,
                description: format!("Broken reference to missing target '{}'", edge.target),
                remediation_hint: format!(
                    "Create target file '{}' or update link in '{}'",
                    edge.target, edge.source
                ),
                is_legacy: false,
                is_excluded: false,
            });
        } else if !edge.is_clickable {
            let b = topology
                .nodes
                .get(&edge.source)
                .map(|n| n.betweenness)
                .unwrap_or(0.0);
            let weight = calculate_topological_weight(b);
            let deduction = (0.5 * weight * 10.0).round() / 10.0;
            let reason_label = format_non_clickable_reason(edge.non_clickable_reason);
            let suggested_dest = edge
                .clickable_target
                .as_deref()
                .unwrap_or(&edge.target);
            let label = if edge.raw_text.trim().is_empty() {
                edge.target.as_str()
            } else {
                edge.raw_text.as_str()
            };
            issues.push(KnowledgeIssue {
                id: format!(
                    "integrity:non_clickable:{}:{}:{}",
                    edge.source,
                    edge.target,
                    edge.line.unwrap_or(0)
                ),
                pillar: "integrity".to_string(),
                category: "non_clickable_link".to_string(),
                file_path: edge.source.clone(),
                line_number: edge.line,
                target: Some(edge.target.clone()),
                deduction_points: deduction,
                weight: (weight * 100.0).round() / 100.0,
                description: format!(
                    "Link to '{}' exists, but is not clickable when rendered ({})",
                    edge.target, reason_label
                ),
                remediation_hint: format!(
                    "Convert to standard clickable Markdown link: [{}]({})",
                    label, suggested_dest
                ),
                is_legacy: false,
                is_excluded: false,
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
            description: format!(
                "Semantic collision with '{}': {:.1}% similarity ({:?})",
                c.target_id,
                c.composite_similarity * 100.0,
                c.classification
            ),
            remediation_hint: format!(
                "Consolidate duplicate concepts into canonical note '{}'",
                c.canonical_candidate.as_deref().unwrap_or(&c.target_id)
            ),
            is_legacy: false,
            is_excluded: false,
        });
    }

    let mut sorted_nodes_disc: Vec<_> = topology.nodes.iter().collect();
    sorted_nodes_disc.sort_by(|a, b| a.0.cmp(b.0));
    for (id, node) in sorted_nodes_disc {
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
                remediation_hint: "Add wiki links to connect this note to the knowledge graph"
                    .to_string(),
                is_legacy: false,
                is_excluded: false,
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
                description: format!(
                    "Node is {} hops away from designated central hub",
                    node.hop_distance
                ),
                remediation_hint:
                    "Add cross-links from higher-level hub notes to improve discovery".to_string(),
                is_legacy: false,
                is_excluded: false,
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
                description: "Node is part of a disconnected subgraph unreachable from hub"
                    .to_string(),
                remediation_hint: "Connect this cluster to the primary knowledge graph".to_string(),
                is_legacy: false,
                is_excluded: false,
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
            id: format!(
                "freshness:{}:{}:{}",
                d.doc_id, d.code_target, d.citation_line
            ),
            pillar: "freshness".to_string(),
            category: "technical_drift".to_string(),
            file_path: d.doc_id.clone(),
            line_number: Some(d.citation_line),
            target: Some(d.code_target.clone()),
            deduction_points: deduction,
            weight: (weight * 100.0).round() / 100.0,
            description: format!(
                "Cites modified/deleted code '{}' ({})",
                d.code_target, d.explanation
            ),
            remediation_hint: format!(
                "Update documentation to reflect changes in '{}'",
                d.code_target
            ),
            is_legacy: false,
            is_excluded: false,
        });
    }

    let mut sorted_nodes_fresh: Vec<_> = topology.nodes.iter().collect();
    sorted_nodes_fresh.sort_by(|a, b| a.0.cmp(b.0));
    for (path, node) in sorted_nodes_fresh {
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
                    description: format!(
                        "Cites code file '{}' which does not exist on disk",
                        missing
                    ),
                    remediation_hint: format!(
                        "Create source file '{}' or update code citation in '{}'",
                        missing, path
                    ),
                    is_legacy: false,
                is_excluded: false,
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
                    description: format!(
                        "Cited code '{}' was modified in Git after document's last commit",
                        mod_code
                    ),
                    remediation_hint: format!(
                        "Review recent code changes in '{}' and update documentation accordingly",
                        mod_code
                    ),
                    is_legacy: false,
                is_excluded: false,
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
        let hint = match sf.debt_type {
            crate::quality::structure::StructureDebtType::HarnessDrift => "Synchronize route definitions across configured AI harness files",
            crate::quality::structure::StructureDebtType::SecondaryRoutingLayer => "Consolidate routes into primary AI Harness Entry Point",
            crate::quality::structure::StructureDebtType::UnanchoredReference => "Add section fragment anchor (#section) to wikilink",
            crate::quality::structure::StructureDebtType::MissingFragmentMetadata => "Materialize fragment metadata with 'knowframe-core materialize'",
            crate::quality::structure::StructureDebtType::OversizedDocument => "Decompose document into focused topical modules",
            crate::quality::structure::StructureDebtType::HighReferenceConcentration => "Distribute inbound links across specific section anchors",
            crate::quality::structure::StructureDebtType::UnfragmentedCentralHub => "Add section headings and anchors to central hub",
        };
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
            remediation_hint: hint.to_string(),
            is_legacy: false,
            is_excluded: false,
        });
    }

    // Sort issues by deduction points descending
    issues.sort_by(|a, b| {
        b.deduction_points
            .partial_cmp(&a.deduction_points)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.id.cmp(&b.id))
            .then_with(|| a.file_path.cmp(&b.file_path))
    });

    issues
}

/// Determines whether an issue is already recorded in the brownfield debt baseline.
pub fn is_issue_baselined(issue: &KnowledgeIssue, baseline: &KnowledgeDebtBaseline) -> bool {
    match issue.pillar.as_str() {
        "integrity" => {
            let target = issue.target.as_deref().unwrap_or("");
            baseline.broken_links.iter().any(|bl| {
                bl.source_path == issue.file_path && (bl.target_ref == target || target.starts_with(&bl.target_ref))
            })
        }
        "consistency" => {
            if let Some(target) = &issue.target {
                let sig = CollisionSignature::new(&issue.file_path, target);
                baseline.collisions.contains(&sig)
            } else {
                false
            }
        }
        "discoverability" => {
            if issue.category == "orphan_node" {
                baseline.orphan_nodes.contains(&issue.file_path)
            } else if issue.category == "high_hop_distance" || issue.category == "disconnected_subgraph" {
                baseline.total_debt > 0
            } else {
                false
            }
        }
        "freshness" | "traceability" => {
            let target = issue.target.as_deref().unwrap_or("");
            if issue.category == "technical_drift" || issue.category == "missing_code_citation" {
                baseline.technical_drifts.iter().any(|td| {
                    td.doc_path == issue.file_path && (td.referenced_code_path == target || target.ends_with(&td.referenced_code_path))
                })
            } else if issue.category == "code_modified_after_doc" {
                baseline.technical_drifts.iter().any(|td| {
                    td.doc_path == issue.file_path && (td.referenced_code_path == target || target.ends_with(&td.referenced_code_path))
                }) || baseline.total_debt > 0
            } else {
                false
            }
        }
        "structure" => {
            if issue.category == "unanchored_reference" || issue.category == "unanchoredreference" {
                let target = issue.target.as_deref().unwrap_or("");
                baseline.broken_links.iter().any(|bl| {
                    bl.source_path == issue.file_path && (bl.target_ref == target || target.starts_with(&bl.target_ref))
                }) || baseline.total_debt > 0
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Extracts all knowledge issues, tagging legacy items according to the debt baseline.
pub fn extract_all_issues_with_baseline(
    topology: &GraphTopology,
    collisions: &[SemanticCollision],
    drifts: &[TechnicalDriftFinding],
    baseline: Option<&KnowledgeDebtBaseline>,
) -> Vec<KnowledgeIssue> {
    let mut issues = extract_all_issues(topology, collisions, drifts);
    for issue in &mut issues {
        if let Some(node) = topology.nodes.get(&issue.file_path) {
            issue.is_excluded = node.status == crate::model::NodeStatus::Excluded;
        }
        if let Some(base) = baseline {
            issue.is_legacy = is_issue_baselined(issue, base);
        }
    }
    issues
}
