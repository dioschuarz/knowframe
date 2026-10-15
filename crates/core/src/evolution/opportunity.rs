//! Structural Opportunities and 10-Dimensional Evidence Analytics.
//! Implements FR-022 through FR-036, Invariants XIII, XIV, XV, and XVI.

use serde::{Deserialize, Serialize};
use std::path::Path;

use super::profile::OptimizationProfile;
use super::signatures::{detect_all_signatures, ProblemSignatureType, StructuralProblemSignature};
use super::strategies::{
    calculate_fitness, generate_candidate_strategies, generate_trade_off_matrix,
    KnowledgeOrganizationStrategy,
};
use super::transformation::{TransformationAction, TransformationEngine, TransformationPlan};
use crate::model::NoteNode;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructuralOpportunity {
    pub id: String,
    pub target_document: String,
    pub signature: ProblemSignature,
    /// Confidence rating (0.0..1.0): High >= 0.80, Medium 0.50..0.79, Low < 0.50
    pub confidence: f64,
    /// Narrative explanation of WHY this opportunity was detected
    pub narrative_why: String,
    /// Multi-dimensional evidence metrics covering all 10 canonical dimensions
    pub evidence: OpportunityEvidence,
    /// Candidate strategies evaluated for this problem
    pub candidate_strategies: Vec<StrategyRecommendation>,
    /// Multi-strategy recommendation set with primary, supporting, and alternatives (Feature 007)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_set: Option<super::strategies::StrategySet>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProblemSignature {
    /// High context waste / structurally overloaded document
    StructurallyOverloadedDocument,
    /// Multi-document duplication or fragmentation
    KnowledgeFragmentation,
    /// Excessively long traversal path to access relevant knowledge
    WeakDiscoveryPath,
    /// Disconnected cluster of knowledge
    IsolatedKnowledgeSilo,
}

/// 10-Dimensional structural analytics evidence metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpportunityEvidence {
    pub document_size_tokens: u32,
    pub reference_concentration: f64,
    pub cohesion: f64,
    pub coupling: f64,
    pub granularity: f64,
    pub context_efficiency: f64,
    pub discoverability: f64,
    pub knowledge_reuse: f64,
    pub semantic_diversity: f64,
    pub navigational_complexity: f64,
    pub maintenance_coupling: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StrategyRecommendation {
    pub strategy: EvolutionStrategy,
    /// Mathematical fit score (0.0..1.0)
    pub fit_score: f64,
    pub confidence: f64,
    pub why: String,
    pub expected_benefits: Vec<BenefitItem>,
    pub countervailing_trade_offs: Vec<TradeOffItem>,
    pub effort: EffortLevel,
    pub is_hybrid: bool,
    pub hybrid_partner: Option<EvolutionStrategy>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvolutionStrategy {
    SectionAddressing,
    TopicDecomposition,
    LightweightIndex,
    FacetedClassification,
    PreserveDocument,
    Hybrid,
}

impl From<KnowledgeOrganizationStrategy> for EvolutionStrategy {
    fn from(s: KnowledgeOrganizationStrategy) -> Self {
        match s {
            KnowledgeOrganizationStrategy::SectionAddressing => EvolutionStrategy::SectionAddressing,
            KnowledgeOrganizationStrategy::TopicDecomposition => EvolutionStrategy::TopicDecomposition,
            KnowledgeOrganizationStrategy::LightweightIndex => EvolutionStrategy::LightweightIndex,
            KnowledgeOrganizationStrategy::FacetedClassification => {
                EvolutionStrategy::FacetedClassification
            }
            KnowledgeOrganizationStrategy::PreserveDocument => EvolutionStrategy::PreserveDocument,
            KnowledgeOrganizationStrategy::Hybrid => EvolutionStrategy::Hybrid,
        }
    }
}

impl From<EvolutionStrategy> for KnowledgeOrganizationStrategy {
    fn from(s: EvolutionStrategy) -> Self {
        match s {
            EvolutionStrategy::SectionAddressing => KnowledgeOrganizationStrategy::SectionAddressing,
            EvolutionStrategy::TopicDecomposition => KnowledgeOrganizationStrategy::TopicDecomposition,
            EvolutionStrategy::LightweightIndex => KnowledgeOrganizationStrategy::LightweightIndex,
            EvolutionStrategy::FacetedClassification => {
                KnowledgeOrganizationStrategy::FacetedClassification
            }
            EvolutionStrategy::PreserveDocument => KnowledgeOrganizationStrategy::PreserveDocument,
            EvolutionStrategy::Hybrid => KnowledgeOrganizationStrategy::Hybrid,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenefitItem {
    pub metric: String,
    pub direction: MetricDirection,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeOffItem {
    pub metric: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetricDirection {
    Increase,
    Decrease,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricDeltaComparison {
    pub metric_name: String,
    pub before_value: f64,
    pub after_value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProposedActionPreview {
    pub action_type: String,
    pub file_path: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformationPlanPreview {
    pub plan_id: String,
    pub target_document: String,
    pub strategy: EvolutionStrategy,
    pub hybrid_partner: Option<EvolutionStrategy>,
    pub proposed_actions: Vec<ProposedActionPreview>,
    pub expected_impact: Vec<MetricDeltaComparison>,
    pub trade_offs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReMeasurementReport {
    pub context_efficiency_delta_pct: f64,
    pub trace_depth_delta_pct: f64,
    pub maintenance_coupling_delta_pct: f64,
    pub document_count_delta: i32,
    pub reference_count_delta: i32,
    pub narrative_summary: String,
}

fn build_strategy_recommendation(
    strategy: KnowledgeOrganizationStrategy,
    signature: &StructuralProblemSignature,
    profile: &OptimizationProfile,
) -> StrategyRecommendation {
    let fit_score = calculate_fitness(strategy, signature, profile) as f64;
    let confidence = signature.confidence as f64;
    let matrix = generate_trade_off_matrix(strategy, signature);

    let (expected_benefits, countervailing_trade_offs, effort, is_hybrid, hybrid_partner) =
        match strategy {
            KnowledgeOrganizationStrategy::SectionAddressing => (
                vec![
                    BenefitItem {
                        metric: "Context Efficiency".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Enables zero-explosion section-level routing via Markdown heading anchors".to_string(),
                    },
                    BenefitItem {
                        metric: "Discoverability".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Provides granular deep links without file explosion".to_string(),
                    },
                ],
                vec![
                    TradeOffItem {
                        metric: "Frontmatter Overhead".to_string(),
                        description: "Requires maintaining section anchor metadata in frontmatter".to_string(),
                    },
                    TradeOffItem {
                        metric: "File Length".to_string(),
                        description: "Document remains monolithic in physical file system".to_string(),
                    },
                ],
                EffortLevel::Low,
                false,
                None,
            ),
            KnowledgeOrganizationStrategy::TopicDecomposition => (
                vec![
                    BenefitItem {
                        metric: "Cohesion".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Isolates distinct topic boundaries into cohesive standalone documents".to_string(),
                    },
                    BenefitItem {
                        metric: "Coupling".to_string(),
                        direction: MetricDirection::Decrease,
                        description: "Reduces change coupling across distinct concerns".to_string(),
                    },
                ],
                vec![
                    TradeOffItem {
                        metric: "Document Count".to_string(),
                        description: "Increases repository node count and requires reference rewrites".to_string(),
                    },
                    TradeOffItem {
                        metric: "Narrative Coherence".to_string(),
                        description: "Splits narrative flow across separate files".to_string(),
                    },
                ],
                EffortLevel::High,
                false,
                None,
            ),
            KnowledgeOrganizationStrategy::LightweightIndex => (
                vec![
                    BenefitItem {
                        metric: "Discoverability".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Establishes a central router document with curated discovery paths".to_string(),
                    },
                    BenefitItem {
                        metric: "Navigational Complexity".to_string(),
                        direction: MetricDirection::Decrease,
                        description: "Reduces graph diameter and hop distance".to_string(),
                    },
                ],
                vec![
                    TradeOffItem {
                        metric: "Maintenance Simplicity".to_string(),
                        description: "Index routes must be maintained as documents change".to_string(),
                    },
                ],
                EffortLevel::Medium,
                false,
                None,
            ),
            KnowledgeOrganizationStrategy::FacetedClassification => (
                vec![
                    BenefitItem {
                        metric: "Knowledge Reuse".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Enables multidimensional discovery through structured taxonomy metadata".to_string(),
                    },
                ],
                vec![
                    TradeOffItem {
                        metric: "Metadata Overhead".to_string(),
                        description: "Requires strict taxonomy compliance across documents".to_string(),
                    },
                ],
                EffortLevel::Medium,
                false,
                None,
            ),
            KnowledgeOrganizationStrategy::PreserveDocument => (
                vec![
                    BenefitItem {
                        metric: "Narrative Coherence".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Preserves unbroken narrative flow and reading continuity".to_string(),
                    },
                ],
                vec![
                    TradeOffItem {
                        metric: "Context Efficiency".to_string(),
                        description: "Retrieving document loads complete token surface into context window".to_string(),
                    },
                ],
                EffortLevel::Low,
                false,
                None,
            ),
            KnowledgeOrganizationStrategy::Hybrid => (
                vec![
                    BenefitItem {
                        metric: "Context Efficiency & Navigation".to_string(),
                        direction: MetricDirection::Increase,
                        description: "Combines section addressing with lightweight index routing for balanced retrieval".to_string(),
                    },
                ],
                vec![
                    TradeOffItem {
                        metric: "Operational Maintenance".to_string(),
                        description: "Requires managing both anchor metadata and index entries".to_string(),
                    },
                ],
                EffortLevel::Medium,
                true,
                Some(EvolutionStrategy::LightweightIndex),
            ),
        };

    let why = format!(
        "Strategy {:?} achieves a {:.0}% fit with active {:?} profile. Expected benefits include {}.",
        strategy,
        fit_score * 100.0,
        profile.orientation,
        matrix.expected_benefits.first().cloned().unwrap_or_default()
    );

    StrategyRecommendation {
        strategy: strategy.into(),
        fit_score,
        confidence,
        why,
        expected_benefits,
        countervailing_trade_offs,
        effort,
        is_hybrid,
        hybrid_partner,
    }
}

/// Builds all structural opportunities evaluated across the workspace.
/// Invariant XIII: Healthy monoliths (>50k tokens) are NEVER included as opportunities to split.
pub fn build_structural_opportunities(
    nodes: &[NoteNode],
    profile: &OptimizationProfile,
) -> Vec<StructuralOpportunity> {
    let signatures = detect_all_signatures(nodes, profile.fragment_size_target);

    signatures
        .into_iter()
        .filter_map(|sig| {
            let (signature_type, narrative_why) = match sig.problem_type {
                ProblemSignatureType::GodDocument => (
                    ProblemSignature::StructurallyOverloadedDocument,
                    format!(
                        "Document exhibits structural overload: {:.2} semantic diversity and {:.2} reference concentration with low context efficiency ({:.2}).",
                        sig.evidence.semantic_diversity,
                        sig.evidence.reference_concentration,
                        sig.evidence.context_efficiency
                    ),
                ),
                ProblemSignatureType::ImplicitRouter => (
                    ProblemSignature::WeakDiscoveryPath,
                    format!(
                        "Document acts as an structural traffic hub with high betweenness ({:.2}) but sparse prose, causing weak context discovery.",
                        sig.evidence.navigational_complexity
                    ),
                ),
                ProblemSignatureType::FragmentedConcept => (
                    ProblemSignature::KnowledgeFragmentation,
                    format!(
                        "Related concepts are fragmented across multiple weakly connected documents (coupling {:.2}).",
                        sig.evidence.coupling
                    ),
                ),
                ProblemSignatureType::IsolatedSilo => (
                    ProblemSignature::IsolatedKnowledgeSilo,
                    format!(
                        "Document is an isolated knowledge silo with zero incoming references (discoverability {:.2}).",
                        sig.evidence.discoverability
                    ),
                ),
                ProblemSignatureType::HealthyMonolith => {
                    // Protected by Invariant XIII: healthy monoliths are NEVER opportunities to split
                    return None;
                }
            };

            let node = nodes
                .iter()
                .find(|n| n.file_path == sig.document_path || n.id == sig.document_path);
            let document_size_tokens = node.map(|n| n.tokens).unwrap_or(0);

            let evidence = OpportunityEvidence {
                document_size_tokens,
                reference_concentration: sig.evidence.reference_concentration as f64,
                cohesion: sig.evidence.cohesion as f64,
                coupling: sig.evidence.coupling as f64,
                granularity: sig.evidence.granularity as f64,
                context_efficiency: sig.evidence.context_efficiency as f64,
                discoverability: sig.evidence.discoverability as f64,
                knowledge_reuse: sig.evidence.reusability as f64,
                semantic_diversity: sig.evidence.semantic_diversity as f64,
                navigational_complexity: sig.evidence.navigational_complexity as f64,
                maintenance_coupling: sig.evidence.maintenance_coupling as f64,
            };

            let candidate_strategies = generate_candidate_strategies(&sig)
                .into_iter()
                .map(|strat| build_strategy_recommendation(strat, &sig, profile))
                .collect();

            let strategy_set = super::strategies::select_strategy_set(
                &sig.signature_id,
                &sig.document_path,
                &sig,
                profile,
            );

            Some(StructuralOpportunity {
                id: sig.signature_id,
                target_document: sig.document_path,
                signature: signature_type,
                confidence: sig.confidence as f64,
                narrative_why,
                evidence,
                candidate_strategies,
                strategy_set: Some(strategy_set),
            })
        })
        .collect()
}

/// Builds detailed opportunity information for a specific opportunity ID.
pub fn build_opportunity_detail(
    opp_id: &str,
    nodes: &[NoteNode],
    profile: &OptimizationProfile,
) -> Option<StructuralOpportunity> {
    let opportunities = build_structural_opportunities(nodes, profile);
    opportunities.into_iter().find(|opp| opp.id == opp_id)
}

/// Builds the StrategySet for a given opportunity ID (T011).
pub fn build_opportunity_strategy_set(
    opp_id: &str,
    nodes: &[NoteNode],
    profile: &OptimizationProfile,
) -> Option<super::strategies::StrategySet> {
    let signatures = detect_all_signatures(nodes, profile.fragment_size_target);
    if let Some(sig) = signatures.iter().find(|s| s.signature_id == opp_id) {
        return Some(super::strategies::select_strategy_set(
            &sig.signature_id,
            &sig.document_path,
            sig,
            profile,
        ));
    }
    if let Some(opp) = build_opportunity_detail(opp_id, nodes, profile) {
        if let Some(set) = opp.strategy_set {
            return Some(set);
        }
    }
    None
}

/// Generates a preview for a selected transformation plan along with the internal TransformationPlan (FR-037, Invariant XV).
pub fn build_transformation_preview_and_plan(
    opp_id: &str,
    strategy: EvolutionStrategy,
    hybrid_partner: Option<EvolutionStrategy>,
    nodes: &[NoteNode],
    workspace_root: &Path,
    profile: &OptimizationProfile,
) -> Option<(TransformationPlanPreview, TransformationPlan)> {
    let opp = build_opportunity_detail(opp_id, nodes, profile)?;
    let node = nodes
        .iter()
        .find(|n| n.file_path == opp.target_document || n.id == opp.target_document)?;

    let rec = opp
        .candidate_strategies
        .iter()
        .find(|s| s.strategy == strategy)?;

    let k_strat: KnowledgeOrganizationStrategy = strategy.into();
    let strat_rec = super::strategies::StructuralRecommendation {
        recommendation_id: format!("rec_{}", opp.id),
        signature_id: opp.id.clone(),
        strategy: k_strat,
        confidence: opp.confidence as f32,
        expected_benefits: rec
            .expected_benefits
            .iter()
            .map(|b| b.description.clone())
            .collect(),
        trade_offs: rec
            .countervailing_trade_offs
            .iter()
            .map(|t| t.description.clone())
            .collect(),
        transformation_plan: None,
    };

    let plan = TransformationEngine::build_plan(&strat_rec, node, workspace_root);

    let proposed_actions = plan
        .actions
        .iter()
        .cloned()
        .map(|a| match a {
            TransformationAction::CreateDocument { file_path, .. } => ProposedActionPreview {
                action_type: "create_document".to_string(),
                file_path,
                details: "Creates a new standalone document".to_string(),
            },
            TransformationAction::UpdateDocumentContent { file_path, .. } => ProposedActionPreview {
                action_type: "update_document_links".to_string(),
                file_path,
                details: "Updates references and anchors in document".to_string(),
            },
            TransformationAction::UpdateDocumentTags { file_path, tags } => ProposedActionPreview {
                action_type: "inject_frontmatter".to_string(),
                file_path,
                details: format!("Updates taxonomy tags: {:?}", tags),
            },
            TransformationAction::AppendDocumentContent { file_path, .. } => ProposedActionPreview {
                action_type: "update_document_links".to_string(),
                file_path,
                details: "Appends structured links to document".to_string(),
            },
            TransformationAction::RewriteLinkReferences {
                source_path,
                old_target,
                new_target,
            } => ProposedActionPreview {
                action_type: "update_document_links".to_string(),
                file_path: source_path,
                details: format!("Rewrites reference from '{}' to '{}'", old_target, new_target),
            },
            TransformationAction::InjectFragmentFrontmatter {
                file_path,
                fragments,
            } => ProposedActionPreview {
                action_type: "inject_frontmatter".to_string(),
                file_path,
                details: format!("Injects {} heading section anchors into frontmatter", fragments.len()),
            },
            TransformationAction::DeleteDocument { file_path } => ProposedActionPreview {
                action_type: "delete_document".to_string(),
                file_path,
                details: "Safely removes obsolete document after route consolidation".to_string(),
            },
        })
        .collect();

    let expected_impact = vec![
        MetricDeltaComparison {
            metric_name: "Context Efficiency".to_string(),
            before_value: opp.evidence.context_efficiency,
            after_value: (opp.evidence.context_efficiency + plan.projected_delta.efficiency_delta as f64).clamp(0.0, 1.0),
            unit: "ratio".to_string(),
        },
        MetricDeltaComparison {
            metric_name: "Trace Depth".to_string(),
            before_value: opp.evidence.navigational_complexity,
            after_value: (opp.evidence.navigational_complexity + plan.projected_delta.trace_depth_delta as f64).max(0.0),
            unit: "hops".to_string(),
        },
        MetricDeltaComparison {
            metric_name: "Coupling".to_string(),
            before_value: opp.evidence.coupling,
            after_value: (opp.evidence.coupling + plan.projected_delta.coupling_delta as f64).clamp(0.0, 1.0),
            unit: "ratio".to_string(),
        },
    ];

    let trade_offs = rec
        .countervailing_trade_offs
        .iter()
        .map(|t| format!("{}: {}", t.metric, t.description))
        .collect();

    let preview = TransformationPlanPreview {
        plan_id: plan.plan_id.clone(),
        target_document: opp.target_document,
        strategy,
        hybrid_partner,
        proposed_actions,
        expected_impact,
        trade_offs,
    };

    Some((preview, plan))
}

/// Generates a preview for a selected transformation plan (FR-037, Invariant XV).
pub fn build_transformation_preview(
    opp_id: &str,
    strategy: EvolutionStrategy,
    hybrid_partner: Option<EvolutionStrategy>,
    nodes: &[NoteNode],
    workspace_root: &Path,
    profile: &OptimizationProfile,
) -> Option<TransformationPlanPreview> {
    build_transformation_preview_and_plan(
        opp_id,
        strategy,
        hybrid_partner,
        nodes,
        workspace_root,
        profile,
    )
    .map(|(preview, _)| preview)
}

/// Generates an atomic transformation preview and plan for multiple selected strategies (T028, T029, SC-007).
pub fn build_multi_strategy_transformation_preview_and_plan(
    opp_id: &str,
    selected_strategies: &[KnowledgeOrganizationStrategy],
    nodes: &[NoteNode],
    workspace_root: &Path,
    profile: &OptimizationProfile,
) -> Option<(TransformationPlanPreview, TransformationPlan)> {
    let opp = build_opportunity_detail(opp_id, nodes, profile)?;
    let node = nodes
        .iter()
        .find(|n| n.file_path == opp.target_document || n.id == opp.target_document)?;

    let primary_strat = selected_strategies
        .first()
        .copied()
        .unwrap_or(KnowledgeOrganizationStrategy::PreserveDocument);
    let plan = TransformationEngine::build_multi_strategy_plan(
        &format!("plan_{}", opp.id),
        &format!("rec_{}", opp.id),
        selected_strategies,
        node,
        workspace_root,
    );

    let proposed_actions = plan
        .actions
        .iter()
        .cloned()
        .map(|a| match a {
            TransformationAction::CreateDocument { file_path, .. } => ProposedActionPreview {
                action_type: "create_document".to_string(),
                file_path,
                details: "Creates a new standalone document".to_string(),
            },
            TransformationAction::UpdateDocumentContent { file_path, .. } => ProposedActionPreview {
                action_type: "update_document_links".to_string(),
                file_path,
                details: "Updates references and anchors in document".to_string(),
            },
            TransformationAction::UpdateDocumentTags { file_path, tags } => ProposedActionPreview {
                action_type: "inject_frontmatter".to_string(),
                file_path,
                details: format!("Updates taxonomy tags: {:?}", tags),
            },
            TransformationAction::AppendDocumentContent { file_path, .. } => ProposedActionPreview {
                action_type: "update_document_links".to_string(),
                file_path,
                details: "Appends structured links to document".to_string(),
            },
            TransformationAction::RewriteLinkReferences {
                source_path,
                old_target,
                new_target,
            } => ProposedActionPreview {
                action_type: "update_document_links".to_string(),
                file_path: source_path,
                details: format!("Rewrites reference from '{}' to '{}'", old_target, new_target),
            },
            TransformationAction::InjectFragmentFrontmatter {
                file_path,
                fragments,
            } => ProposedActionPreview {
                action_type: "inject_frontmatter".to_string(),
                file_path,
                details: format!("Injects {} heading section anchors into frontmatter", fragments.len()),
            },
            TransformationAction::DeleteDocument { file_path } => ProposedActionPreview {
                action_type: "delete_document".to_string(),
                file_path,
                details: "Safely removes obsolete document after route consolidation".to_string(),
            },
        })
        .collect();

    let expected_impact = vec![
        MetricDeltaComparison {
            metric_name: "Context Efficiency".to_string(),
            before_value: opp.evidence.context_efficiency,
            after_value: (opp.evidence.context_efficiency + plan.projected_delta.efficiency_delta as f64).clamp(0.0, 1.0),
            unit: "ratio".to_string(),
        },
        MetricDeltaComparison {
            metric_name: "Trace Depth".to_string(),
            before_value: opp.evidence.navigational_complexity,
            after_value: (opp.evidence.navigational_complexity + plan.projected_delta.trace_depth_delta as f64).max(0.0),
            unit: "hops".to_string(),
        },
        MetricDeltaComparison {
            metric_name: "Coupling".to_string(),
            before_value: opp.evidence.coupling,
            after_value: (opp.evidence.coupling + plan.projected_delta.coupling_delta as f64).clamp(0.0, 1.0),
            unit: "ratio".to_string(),
        },
    ];

    let mut trade_offs = Vec::new();
    for strat in selected_strategies {
        let tm = super::strategies::generate_trade_off_matrix(*strat, &StructuralProblemSignature {
            signature_id: opp.id.clone(),
            document_path: opp.target_document.clone(),
            problem_type: match opp.signature {
                ProblemSignature::StructurallyOverloadedDocument => ProblemSignatureType::GodDocument,
                ProblemSignature::KnowledgeFragmentation => ProblemSignatureType::FragmentedConcept,
                ProblemSignature::WeakDiscoveryPath => ProblemSignatureType::ImplicitRouter,
                ProblemSignature::IsolatedKnowledgeSilo => ProblemSignatureType::IsolatedSilo,
            },
            confidence: opp.confidence as f32,
            evidence: Default::default(),
            rationale: opp.narrative_why.clone(),
        });
        for to in tm.trade_offs {
            trade_offs.push(format!("{:?}: {}", strat, to));
        }
    }

    let preview = TransformationPlanPreview {
        plan_id: plan.plan_id.clone(),
        target_document: opp.target_document,
        strategy: primary_strat.into(),
        hybrid_partner: selected_strategies.get(1).map(|&s| s.into()),
        proposed_actions,
        expected_impact,
        trade_offs,
    };

    Some((preview, plan))
}

