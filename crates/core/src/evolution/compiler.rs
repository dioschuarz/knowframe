use super::profile::{ConstraintRule, DocumentationOrientation, OptimizationProfile};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectiveLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tier1Objectives {
    pub ai_discoverability: ObjectiveLevel,
    pub context_efficiency: ObjectiveLevel,
    pub human_readability: ObjectiveLevel,
    pub knowledge_reuse: ObjectiveLevel,
    pub narrative_coherence: ObjectiveLevel,
    pub maintenance_simplicity: ObjectiveLevel,
}

impl Default for Tier1Objectives {
    fn default() -> Self {
        Self {
            ai_discoverability: ObjectiveLevel::High,
            context_efficiency: ObjectiveLevel::High,
            human_readability: ObjectiveLevel::Medium,
            knowledge_reuse: ObjectiveLevel::Medium,
            narrative_coherence: ObjectiveLevel::Low,
            maintenance_simplicity: ObjectiveLevel::Medium,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tier2Parameters {
    pub cohesion_threshold: f32,
    pub semantic_diversity_threshold: f32,
    pub reference_concentration_threshold: f32,
    pub fragment_size_target: u32,
    pub max_trace_depth: u32,
    pub candidate_expansion_factor: u32,
    #[serde(alias = "max_context_surface")]
    pub max_context_surface_tokens: u32,
    pub split_sensitivity: f32,
    pub canonical_ownership_sensitivity: f32,
    pub duplicate_tolerance: f32,
    pub fragmentation_tolerance: f32,
}

impl Default for Tier2Parameters {
    fn default() -> Self {
        Self {
            cohesion_threshold: 0.70,
            semantic_diversity_threshold: 0.75,
            reference_concentration_threshold: 0.70,
            fragment_size_target: 3000,
            max_trace_depth: 3,
            candidate_expansion_factor: 3,
            max_context_surface_tokens: 12000,
            split_sensitivity: 0.70,
            canonical_ownership_sensitivity: 0.80,
            duplicate_tolerance: 0.10,
            fragmentation_tolerance: 0.20,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterMappingRelation {
    pub tier1_key: String,
    pub affected_tier2_keys: Vec<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, Default)]
pub struct ProfileCompiler;

impl ProfileCompiler {
    pub fn new() -> Self {
        Self
    }

    /// Compile a documentation orientation preset into a unified OptimizationProfile.
    /// Invariant XIV: All orientations compile to a single internal OptimizationProfile struct.
    pub fn compile_preset(&self, orientation: DocumentationOrientation) -> OptimizationProfile {
        OptimizationProfile::preset(orientation)
    }

    /// Maps Tier 1 discrete knobs (low, medium, high) into continuous Tier 2 engine variables.
    pub fn map_tier1_to_tier2(&self, tier1: &Tier1Objectives) -> Tier2Parameters {
        let (max_trace_depth, candidate_expansion_factor) = match tier1.ai_discoverability {
            ObjectiveLevel::Low => (5, 2),
            ObjectiveLevel::Medium => (3, 3),
            ObjectiveLevel::High => (2, 5),
        };

        let (max_context_surface_tokens, fragment_size_target) = match tier1.context_efficiency {
            ObjectiveLevel::Low => (25000, 15000),
            ObjectiveLevel::Medium => (12000, 8000),
            ObjectiveLevel::High => (5000, 3000),
        };

        let (cohesion_threshold, split_sensitivity) = match tier1.human_readability {
            ObjectiveLevel::Low => (0.40, 0.80),
            ObjectiveLevel::Medium => (0.70, 0.50),
            ObjectiveLevel::High => (0.90, 0.20),
        };

        let (canonical_ownership_sensitivity, duplicate_tolerance) = match tier1.knowledge_reuse {
            ObjectiveLevel::Low => (0.50, 0.30),
            ObjectiveLevel::Medium => (0.75, 0.10),
            ObjectiveLevel::High => (0.95, 0.02),
        };

        let fragmentation_tolerance = match tier1.narrative_coherence {
            ObjectiveLevel::Low => 0.50,
            ObjectiveLevel::Medium => 0.30,
            ObjectiveLevel::High => 0.10,
        };

        let reference_concentration_threshold = match tier1.maintenance_simplicity {
            ObjectiveLevel::Low => 0.50,
            ObjectiveLevel::Medium => 0.70,
            ObjectiveLevel::High => 0.85,
        };

        Tier2Parameters {
            cohesion_threshold,
            semantic_diversity_threshold: 0.75,
            reference_concentration_threshold,
            fragment_size_target,
            max_trace_depth,
            candidate_expansion_factor,
            max_context_surface_tokens,
            split_sensitivity,
            canonical_ownership_sensitivity,
            duplicate_tolerance,
            fragmentation_tolerance,
        }
    }

    /// Compile a custom profile translating Tier 1 objectives and optional manual Tier 2 overrides
    /// into an OptimizationProfile.
    pub fn compile_custom(
        &self,
        tier1: &Tier1Objectives,
        manual_tier2: Option<&Tier2Parameters>,
        constraints: Vec<ConstraintRule>,
    ) -> OptimizationProfile {
        let mapped = self.map_tier1_to_tier2(tier1);
        let tier2 = manual_tier2.unwrap_or(&mapped);

        let cohesion_weight = match tier1.human_readability {
            ObjectiveLevel::Low => 0.40,
            ObjectiveLevel::Medium => 0.70,
            ObjectiveLevel::High => 0.90,
        };

        let context_efficiency_weight = match tier1.context_efficiency {
            ObjectiveLevel::Low => 0.50,
            ObjectiveLevel::Medium => 0.80,
            ObjectiveLevel::High => 1.00,
        };

        let discoverability_weight = match tier1.ai_discoverability {
            ObjectiveLevel::Low => 0.50,
            ObjectiveLevel::Medium => 0.80,
            ObjectiveLevel::High => 1.00,
        };

        let narrative_coherence_weight = match tier1.narrative_coherence {
            ObjectiveLevel::Low => 0.40,
            ObjectiveLevel::Medium => 0.70,
            ObjectiveLevel::High => 0.95,
        };

        let maintenance_simplicity_weight = match tier1.maintenance_simplicity {
            ObjectiveLevel::Low => 0.30,
            ObjectiveLevel::Medium => 0.60,
            ObjectiveLevel::High => 0.90,
        };

        OptimizationProfile {
            orientation: DocumentationOrientation::Custom,
            cohesion_weight,
            coupling_weight: 0.65,
            concentration_weight: 0.70,
            context_efficiency_weight,
            discoverability_weight,
            narrative_coherence_weight,
            maintenance_simplicity_weight,
            max_trace_depth: tier2.max_trace_depth,
            max_context_surface_tokens: tier2.max_context_surface_tokens,
            fragment_size_target: tier2.fragment_size_target,
            split_sensitivity: tier2.split_sensitivity,
            canonical_ownership_sensitivity: tier2.canonical_ownership_sensitivity,
            constraints,
        }
    }

    /// Introspection API returning explicit parameter dependency relationships between Tier 1 and Tier 2.
    pub fn get_parameter_relationships(&self) -> Vec<ParameterMappingRelation> {
        vec![
            ParameterMappingRelation {
                tier1_key: "ai_discoverability".to_string(),
                affected_tier2_keys: vec![
                    "max_trace_depth".to_string(),
                    "candidate_expansion_factor".to_string(),
                ],
                explanation: "Controls search beam breadth and search depth for agent traversal.".to_string(),
            },
            ParameterMappingRelation {
                tier1_key: "context_efficiency".to_string(),
                affected_tier2_keys: vec![
                    "max_context_surface_tokens".to_string(),
                    "fragment_size_target".to_string(),
                ],
                explanation: "Favors focused knowledge units, smaller fragments, and stricter context budgets.".to_string(),
            },
            ParameterMappingRelation {
                tier1_key: "human_readability".to_string(),
                affected_tier2_keys: vec![
                    "cohesion_threshold".to_string(),
                    "split_sensitivity".to_string(),
                ],
                explanation: "Protects long-form conceptual narratives and reduces aggressive splitting.".to_string(),
            },
            ParameterMappingRelation {
                tier1_key: "knowledge_reuse".to_string(),
                affected_tier2_keys: vec![
                    "canonical_ownership_sensitivity".to_string(),
                    "duplicate_tolerance".to_string(),
                ],
                explanation: "Enforces single canonical sources and penalizes redundant content duplication.".to_string(),
            },
            ParameterMappingRelation {
                tier1_key: "narrative_coherence".to_string(),
                affected_tier2_keys: vec!["fragmentation_tolerance".to_string()],
                explanation: "Penalizes excessive decomposition that shatters linear document reading flow.".to_string(),
            },
            ParameterMappingRelation {
                tier1_key: "maintenance_simplicity".to_string(),
                affected_tier2_keys: vec!["reference_concentration_threshold".to_string()],
                explanation: "Detects overloaded sections absorbing disproportionate maintenance links.".to_string(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier1_tier2_models() {
        let t1 = Tier1Objectives::default();
        let serialized_t1 = serde_json::to_string(&t1).unwrap();
        let de_t1: Tier1Objectives = serde_json::from_str(&serialized_t1).unwrap();
        assert_eq!(de_t1.ai_discoverability, ObjectiveLevel::High);

        let t2 = Tier2Parameters::default();
        let serialized_t2 = serde_json::to_string(&t2).unwrap();
        let de_t2: Tier2Parameters = serde_json::from_str(&serialized_t2).unwrap();
        assert_eq!(de_t2.fragment_size_target, 3000);
        assert_eq!(de_t2.max_context_surface_tokens, 12000);
    }

    #[test]
    fn test_compile_preset_profiles() {
        let compiler = ProfileCompiler::new();
        let ai = compiler.compile_preset(DocumentationOrientation::AiOriented);
        assert_eq!(ai.orientation, DocumentationOrientation::AiOriented);
        assert_eq!(ai.fragment_size_target, 3000);

        let human = compiler.compile_preset(DocumentationOrientation::HumanOriented);
        assert_eq!(human.orientation, DocumentationOrientation::HumanOriented);
        assert_eq!(human.fragment_size_target, 8000);

        let balanced = compiler.compile_preset(DocumentationOrientation::Balanced);
        assert_eq!(balanced.orientation, DocumentationOrientation::Balanced);
        assert_eq!(balanced.fragment_size_target, 5000);
    }

    #[test]
    fn test_profile_compilation_determinism() {
        let compiler = ProfileCompiler::new();
        let p1 = compiler.compile_preset(DocumentationOrientation::AiOriented);
        let p2 = compiler.compile_preset(DocumentationOrientation::AiOriented);
        assert_eq!(p1.context_efficiency_weight, p2.context_efficiency_weight);
        assert_eq!(p1.max_trace_depth, p2.max_trace_depth);
        assert_eq!(p1.fragment_size_target, p2.fragment_size_target);
    }

    #[test]
    fn test_tier1_to_tier2_mapping_matrix() {
        let compiler = ProfileCompiler::new();
        let mut t1 = Tier1Objectives::default();
        t1.context_efficiency = ObjectiveLevel::High;
        t1.ai_discoverability = ObjectiveLevel::High;
        t1.human_readability = ObjectiveLevel::Low;

        let mapped = compiler.map_tier1_to_tier2(&t1);
        assert_eq!(mapped.max_trace_depth, 2);
        assert_eq!(mapped.candidate_expansion_factor, 5);
        assert_eq!(mapped.max_context_surface_tokens, 5000);
        assert_eq!(mapped.fragment_size_target, 3000);
        assert_eq!(mapped.split_sensitivity, 0.80);
    }

    #[test]
    fn test_tier2_manual_override_preservation() {
        let compiler = ProfileCompiler::new();
        let t1 = Tier1Objectives::default();
        let mut manual_tier2 = compiler.map_tier1_to_tier2(&t1);
        manual_tier2.max_context_surface_tokens = 9999; // Manual override

        let custom_profile = compiler.compile_custom(&t1, Some(&manual_tier2), vec![]);
        assert_eq!(custom_profile.max_context_surface_tokens, 9999);
    }

    #[test]
    fn test_parameter_mapping_introspection() {
        let compiler = ProfileCompiler::new();
        let relations = compiler.get_parameter_relationships();
        assert_eq!(relations.len(), 6);
        assert!(relations
            .iter()
            .any(|r| r.tier1_key == "ai_discoverability"));
        assert!(relations
            .iter()
            .any(|r| r.tier1_key == "context_efficiency"));
    }
}
