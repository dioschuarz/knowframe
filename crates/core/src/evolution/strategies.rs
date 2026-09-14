use super::profile::{ConstraintLevel, OptimizationProfile};
use super::signatures::{ProblemSignatureType, StructuralProblemSignature};
use super::transformation::TransformationPlan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeOrganizationStrategy {
    SectionAddressing,
    TopicDecomposition,
    LightweightIndex,
    FacetedClassification,
    PreserveDocument,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeOffMatrix {
    pub expected_benefits: Vec<String>,
    pub trade_offs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuralRecommendation {
    pub recommendation_id: String,
    pub signature_id: String,
    pub strategy: KnowledgeOrganizationStrategy,
    pub confidence: f32,
    pub expected_benefits: Vec<String>,
    pub trade_offs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformation_plan: Option<TransformationPlan>,
}

/// Generates applicable candidate strategies for a problem signature (FR-030).
pub fn generate_candidate_strategies(signature: &StructuralProblemSignature) -> Vec<KnowledgeOrganizationStrategy> {
    match signature.problem_type {
        ProblemSignatureType::GodDocument => vec![
            KnowledgeOrganizationStrategy::SectionAddressing,
            KnowledgeOrganizationStrategy::TopicDecomposition,
            KnowledgeOrganizationStrategy::LightweightIndex,
            KnowledgeOrganizationStrategy::Hybrid,
            KnowledgeOrganizationStrategy::PreserveDocument,
        ],
        ProblemSignatureType::HealthyMonolith => vec![
            KnowledgeOrganizationStrategy::PreserveDocument,
            KnowledgeOrganizationStrategy::SectionAddressing,
        ],
        ProblemSignatureType::ImplicitRouter => vec![
            KnowledgeOrganizationStrategy::LightweightIndex,
            KnowledgeOrganizationStrategy::SectionAddressing,
            KnowledgeOrganizationStrategy::PreserveDocument,
        ],
        ProblemSignatureType::FragmentedConcept => vec![
            KnowledgeOrganizationStrategy::FacetedClassification,
            KnowledgeOrganizationStrategy::LightweightIndex,
            KnowledgeOrganizationStrategy::TopicDecomposition,
            KnowledgeOrganizationStrategy::PreserveDocument,
        ],
        ProblemSignatureType::IsolatedSilo => vec![
            KnowledgeOrganizationStrategy::LightweightIndex,
            KnowledgeOrganizationStrategy::FacetedClassification,
            KnowledgeOrganizationStrategy::SectionAddressing,
            KnowledgeOrganizationStrategy::PreserveDocument,
        ],
    }
}

/// Computes deterministic fitness score for candidate strategy against optimization profile (FR-031).
pub fn calculate_fitness(
    strategy: KnowledgeOrganizationStrategy,
    signature: &StructuralProblemSignature,
    profile: &OptimizationProfile,
) -> f32 {
    let base_score: f32 = match strategy {
        KnowledgeOrganizationStrategy::SectionAddressing => {
            0.40 * profile.context_efficiency_weight
                + 0.30 * profile.maintenance_simplicity_weight
                + 0.30 * (1.0 - profile.narrative_coherence_weight * 0.2)
        }
        KnowledgeOrganizationStrategy::TopicDecomposition => {
            0.35 * profile.cohesion_weight
                + 0.35 * profile.discoverability_weight
                + 0.30 * profile.split_sensitivity
                - 0.20 * profile.narrative_coherence_weight
        }
        KnowledgeOrganizationStrategy::LightweightIndex => {
            0.40 * profile.discoverability_weight
                + 0.30 * profile.narrative_coherence_weight
                + 0.30 * profile.maintenance_simplicity_weight
        }
        KnowledgeOrganizationStrategy::FacetedClassification => {
            0.45 * profile.discoverability_weight
                + 0.35 * profile.coupling_weight
                + 0.20 * profile.maintenance_simplicity_weight
        }
        KnowledgeOrganizationStrategy::PreserveDocument => {
            0.50 * profile.narrative_coherence_weight
                + 0.30 * profile.maintenance_simplicity_weight
                + 0.20 * (1.0 - profile.split_sensitivity)
        }
        KnowledgeOrganizationStrategy::Hybrid => {
            0.30 * profile.context_efficiency_weight
                + 0.30 * profile.discoverability_weight
                + 0.20 * profile.split_sensitivity
                + 0.20 * profile.narrative_coherence_weight
        }
    };

    // Modulate with signature confidence and evidence
    let modulated = base_score * (0.5 + 0.5 * signature.confidence);
    modulated.clamp(0.05, 1.0)
}

/// Generates explicit Trade-Off Matrix itemizing expected benefits and countervailing costs (FR-034).
pub fn generate_trade_off_matrix(
    strategy: KnowledgeOrganizationStrategy,
    signature: &StructuralProblemSignature,
) -> TradeOffMatrix {
    match strategy {
        KnowledgeOrganizationStrategy::SectionAddressing => TradeOffMatrix {
            expected_benefits: vec![
                "Enables zero-explosion section-level routing via Markdown heading anchors".to_string(),
                "Improves Context Trace retrieval efficiency by loading targeted subsections".to_string(),
            ],
            trade_offs: vec![
                "Requires frontmatter materialization metadata maintenance".to_string(),
                "Preserves existing monolithic file layout without topic separation".to_string(),
            ],
        },
        KnowledgeOrganizationStrategy::TopicDecomposition => TradeOffMatrix {
            expected_benefits: vec![
                "Eliminates structural God Document by isolating distinct topics into standalone notes".to_string(),
                "Maximizes modular cohesion and independent maintainability".to_string(),
            ],
            trade_offs: vec![
                "Increases repository file count and creates new document nodes".to_string(),
                "Splits long-form narrative coherence and requires link rewrites".to_string(),
            ],
        },
        KnowledgeOrganizationStrategy::LightweightIndex => TradeOffMatrix {
            expected_benefits: vec![
                "Formalizes informal routing behavior into a structured navigation index".to_string(),
                "Enhances graph discoverability from entry hubs without altering existing documents".to_string(),
            ],
            trade_offs: vec![
                "Introduces an extra navigational hub node to maintain".to_string(),
            ],
        },
        KnowledgeOrganizationStrategy::FacetedClassification => TradeOffMatrix {
            expected_benefits: vec![
                "Unifies fragmented concepts across multi-dimensional category frontmatter tags".to_string(),
                "Enables orthogonal thematic queries without altering file hierarchy".to_string(),
            ],
            trade_offs: vec![
                "Requires maintaining consistent taxonomy tags across documents".to_string(),
            ],
        },
        KnowledgeOrganizationStrategy::PreserveDocument => TradeOffMatrix {
            expected_benefits: vec![
                "Preserves 100% narrative coherence and original authorial structure".to_string(),
                "Zero risk of link breakage or file system churn".to_string(),
            ],
            trade_offs: vec![
                format!(
                    "Incurs context inefficiency overhead: document remains at {} tokens",
                    signature.evidence.granularity
                ),
            ],
        },
        KnowledgeOrganizationStrategy::Hybrid => TradeOffMatrix {
            expected_benefits: vec![
                "Combines section addressing for large chapters with lightweight index routing".to_string(),
                "Balanced trade-off between narrative preservation and retrieval efficiency".to_string(),
            ],
            trade_offs: vec![
                "Moderate frontmatter metadata overhead and index file addition".to_string(),
            ],
        },
    }
}

/// Applies constraint filtering, penalizing soft constraints and disqualifying hard constraints (FR-017 - FR-020, T035).
pub fn filter_and_rank_strategies(
    candidates: &[KnowledgeOrganizationStrategy],
    signature: &StructuralProblemSignature,
    profile: &OptimizationProfile,
) -> Vec<(KnowledgeOrganizationStrategy, f32)> {
    let mut ranked = Vec::new();

    for &strategy in candidates {
        let mut fitness = calculate_fitness(strategy, signature, profile);
        let mut disqualified = false;

        for constraint in &profile.constraints {
            // Check splitting constraints
            let is_split_strategy = strategy == KnowledgeOrganizationStrategy::TopicDecomposition;
            if is_split_strategy {
                if constraint.rule_key == "never_split_canonical_docs"
                    || constraint.rule_key == "never_split_documents"
                {
                    match constraint.level {
                        ConstraintLevel::HardConstraint => {
                            disqualified = true;
                            break;
                        }
                        ConstraintLevel::Constraint => {
                            fitness -= 0.35;
                        }
                        ConstraintLevel::Preference => {
                            fitness -= 0.10;
                        }
                    }
                }
            }
        }

        if !disqualified && fitness > 0.0 {
            ranked.push((strategy, fitness.clamp(0.0, 1.0)));
        }
    }

    // Sort descending by fitness
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked
}

/// Generates recommendations for a signature against the active optimization profile (FR-030 - FR-034).
pub fn evaluate_recommendations_for_signature(
    signature: &StructuralProblemSignature,
    profile: &OptimizationProfile,
) -> Vec<StructuralRecommendation> {
    let candidates = generate_candidate_strategies(signature);
    let ranked = filter_and_rank_strategies(&candidates, signature, profile);

    ranked
        .into_iter()
        .enumerate()
        .map(|(idx, (strategy, fitness))| {
            let trade_off = generate_trade_off_matrix(strategy, signature);
            let mut trade_offs = trade_off.trade_offs;

            // FR-033: Label low-confidence (< 0.70) as advisory suggestions
            if fitness < 0.70 {
                trade_offs.insert(0, "[Advisory Suggestion (Low Confidence)] Fitness score is below 0.70 threshold; evaluate carefully against vault goals.".to_string());
            }

            StructuralRecommendation {
                recommendation_id: format!("rec_{}_{}", signature.signature_id, idx + 1),
                signature_id: signature.signature_id.clone(),
                strategy,
                confidence: fitness,
                expected_benefits: trade_off.expected_benefits,
                trade_offs,
                transformation_plan: None,
            }
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::super::profile::ConstraintRule;

    #[test]
    fn test_recommendation_and_strategy_models() {
        let strategy = KnowledgeOrganizationStrategy::FacetedClassification;
        let serialized = serde_json::to_string(&strategy).unwrap();
        assert_eq!(serialized, "\"faceted_classification\"");

        let rec = StructuralRecommendation {
            recommendation_id: "rec-001".to_string(),
            signature_id: "sig-001".to_string(),
            strategy: KnowledgeOrganizationStrategy::SectionAddressing,
            confidence: 0.88,
            expected_benefits: vec!["Improves context efficiency".to_string()],
            trade_offs: vec!["Increases frontmatter metadata overhead".to_string()],
            transformation_plan: None,
        };

        let json = serde_json::to_string(&rec).unwrap();
        let de: StructuralRecommendation = serde_json::from_str(&json).unwrap();
        assert_eq!(de.strategy, KnowledgeOrganizationStrategy::SectionAddressing);
        assert_eq!(de.confidence, 0.88);
    }

    #[test]
    fn test_candidate_strategy_generation() {
        let god_sig = StructuralProblemSignature {
            signature_id: "god_1".to_string(),
            document_path: "docs/monolith.md".to_string(),
            problem_type: ProblemSignatureType::GodDocument,
            confidence: 0.90,
            evidence: Default::default(),
            rationale: "God doc".to_string(),
        };
        let candidates = generate_candidate_strategies(&god_sig);
        assert!(candidates.contains(&KnowledgeOrganizationStrategy::SectionAddressing));
        assert!(candidates.contains(&KnowledgeOrganizationStrategy::TopicDecomposition));
        assert!(candidates.contains(&KnowledgeOrganizationStrategy::LightweightIndex));
        assert!(candidates.contains(&KnowledgeOrganizationStrategy::PreserveDocument));
        assert!(candidates.contains(&KnowledgeOrganizationStrategy::Hybrid));

        let mono_sig = StructuralProblemSignature {
            signature_id: "mono_1".to_string(),
            document_path: "docs/api.md".to_string(),
            problem_type: ProblemSignatureType::HealthyMonolith,
            confidence: 0.95,
            evidence: Default::default(),
            rationale: "Healthy".to_string(),
        };
        let mono_candidates = generate_candidate_strategies(&mono_sig);
        assert!(mono_candidates.contains(&KnowledgeOrganizationStrategy::PreserveDocument));
        assert!(!mono_candidates.contains(&KnowledgeOrganizationStrategy::TopicDecomposition));
    }

    #[test]
    fn test_strategy_fitness_scoring() {
        let sig = StructuralProblemSignature {
            signature_id: "god_1".to_string(),
            document_path: "docs/monolith.md".to_string(),
            problem_type: ProblemSignatureType::GodDocument,
            confidence: 0.90,
            evidence: Default::default(),
            rationale: "God doc".to_string(),
        };

        let ai_profile = OptimizationProfile::ai_oriented();
        let human_profile = OptimizationProfile::human_oriented();

        let ai_score = calculate_fitness(KnowledgeOrganizationStrategy::SectionAddressing, &sig, &ai_profile);
        let human_score = calculate_fitness(KnowledgeOrganizationStrategy::PreserveDocument, &sig, &human_profile);

        assert!(ai_score > 0.0 && ai_score <= 1.0);
        assert!(human_score > 0.0 && human_score <= 1.0);
    }

    #[test]
    fn test_recommendation_confidence_labeling() {
        let sig = StructuralProblemSignature {
            signature_id: "silo_1".to_string(),
            document_path: "docs/orphan.md".to_string(),
            problem_type: ProblemSignatureType::IsolatedSilo,
            confidence: 0.50,
            evidence: Default::default(),
            rationale: "Silo".to_string(),
        };
        let mut profile = OptimizationProfile::balanced();
        // Force very low weights to test < 0.70 threshold
        profile.discoverability_weight = 0.1;
        profile.coupling_weight = 0.1;
        profile.maintenance_simplicity_weight = 0.1;

        let recs = evaluate_recommendations_for_signature(&sig, &profile);
        let low_conf = recs.iter().find(|r| r.confidence < 0.70);
        if let Some(r) = low_conf {
            assert!(
                r.trade_offs.iter().any(|t| t.contains("Advisory Suggestion")),
                "Recommendations with confidence < 0.70 must have advisory label (FR-033)"
            );
        }
    }

    #[test]
    fn test_trade_off_matrix_generation() {
        let sig = StructuralProblemSignature {
            signature_id: "god_1".to_string(),
            document_path: "docs/monolith.md".to_string(),
            problem_type: ProblemSignatureType::GodDocument,
            confidence: 0.90,
            evidence: Default::default(),
            rationale: "God doc".to_string(),
        };

        let matrix = generate_trade_off_matrix(KnowledgeOrganizationStrategy::SectionAddressing, &sig);
        assert!(!matrix.expected_benefits.is_empty());
        assert!(!matrix.trade_offs.is_empty());
    }

    #[test]
    fn test_constraint_aware_strategy_filtering() {
        let sig = StructuralProblemSignature {
            signature_id: "god_1".to_string(),
            document_path: "docs/monolith.md".to_string(),
            problem_type: ProblemSignatureType::GodDocument,
            confidence: 0.90,
            evidence: Default::default(),
            rationale: "God doc".to_string(),
        };

        let mut profile = OptimizationProfile::ai_oriented();
        // Add HardConstraint never_split_canonical_docs
        profile.constraints.push(ConstraintRule {
            id: "hard_no_split".to_string(),
            level: ConstraintLevel::HardConstraint,
            rule_key: "never_split_canonical_docs".to_string(),
            description: "Strictly prevent splitting".to_string(),
        });

        let recs = evaluate_recommendations_for_signature(&sig, &profile);
        // TopicDecomposition must be disqualified by HardConstraint
        assert!(
            !recs.iter().any(|r| r.strategy == KnowledgeOrganizationStrategy::TopicDecomposition),
            "Hard constraint must strictly disqualify TopicDecomposition (FR-020, T035)"
        );
        // SectionAddressing must still be available
        assert!(recs.iter().any(|r| r.strategy == KnowledgeOrganizationStrategy::SectionAddressing));
    }

    #[test]
    fn test_strategy_profile_fitness_scoring() {
        test_strategy_fitness_scoring();
    }

    #[test]
    fn test_strategy_confidence_and_advisory_label() {
        test_recommendation_confidence_labeling();
    }

    #[test]
    fn test_hard_constraint_strategy_disqualification() {
        test_constraint_aware_strategy_filtering();
    }
}

