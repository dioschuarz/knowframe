use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DocumentationOrientation {
    #[default]
    AiOriented,
    HumanOriented,
    Balanced,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintLevel {
    Preference,
    Constraint,
    HardConstraint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintRule {
    pub id: String,
    pub level: ConstraintLevel,
    pub rule_key: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintConflict {
    pub conflicting_rules: Vec<String>,
    pub reason: String,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationProfile {
    pub orientation: DocumentationOrientation,
    pub cohesion_weight: f32,
    pub coupling_weight: f32,
    pub concentration_weight: f32,
    pub context_efficiency_weight: f32,
    pub discoverability_weight: f32,
    pub narrative_coherence_weight: f32,
    pub maintenance_simplicity_weight: f32,
    pub max_trace_depth: u32,
    pub max_context_surface_tokens: u32,
    pub fragment_size_target: u32,
    pub split_sensitivity: f32,
    pub canonical_ownership_sensitivity: f32,
    pub constraints: Vec<ConstraintRule>,
}

impl OptimizationProfile {
    /// 🤖 AI-Oriented preset: optimizes knowledge discovery, canonical routing, section addressability,
    /// and Context Trace traversal for AI agents.
    pub fn ai_oriented() -> Self {
        Self {
            orientation: DocumentationOrientation::AiOriented,
            cohesion_weight: 0.80,
            coupling_weight: 0.70,
            concentration_weight: 0.80,
            context_efficiency_weight: 0.90, // >= 0.85 per FR-004
            discoverability_weight: 0.85,    // >= 0.80 per FR-004
            narrative_coherence_weight: 0.40,
            maintenance_simplicity_weight: 0.60,
            max_trace_depth: 3,
            max_context_surface_tokens: 12000,
            fragment_size_target: 3000, // Canonical 3k tokens for AI
            split_sensitivity: 0.75,
            canonical_ownership_sensitivity: 0.80,
            constraints: Vec::new(),
        }
    }

    /// Compiles an AIOrientedCapabilityMatrix into an overall governance composite score (Invariant XIV).
    pub fn compile_governance_score(
        &self,
        matrix: &crate::harness::AIOrientedCapabilityMatrix,
    ) -> f32 {
        if self.orientation == DocumentationOrientation::AiOriented {
            matrix.overall_governance_score
        } else {
            (matrix.overall_governance_score * 0.5 + matrix.entry_score * 0.5).clamp(0.0, 100.0)
        }
    }

    /// 👤 Human-Oriented preset: prioritizes narrative continuity, readability, conceptual explanation,
    /// human navigation, and maintainability.
    pub fn human_oriented() -> Self {
        Self {
            orientation: DocumentationOrientation::HumanOriented,
            cohesion_weight: 0.85,
            coupling_weight: 0.50,
            concentration_weight: 0.40,
            context_efficiency_weight: 0.45,
            discoverability_weight: 0.60,
            narrative_coherence_weight: 0.90, // >= 0.85 per FR-005
            maintenance_simplicity_weight: 0.80, // >= 0.75 per FR-005
            max_trace_depth: 5,
            max_context_surface_tokens: 25000,
            fragment_size_target: 8000, // Canonical 8k tokens for Human
            split_sensitivity: 0.25,
            canonical_ownership_sensitivity: 0.60,
            constraints: Vec::new(),
        }
    }

    /// ⚖️ Balanced preset: multi-objective compromise balancing AI retrieval efficiency and human comprehension.
    pub fn balanced() -> Self {
        Self {
            orientation: DocumentationOrientation::Balanced,
            cohesion_weight: 0.80,
            coupling_weight: 0.60,
            concentration_weight: 0.60,
            context_efficiency_weight: 0.60,
            discoverability_weight: 0.70,
            narrative_coherence_weight: 0.60,
            maintenance_simplicity_weight: 0.70,
            max_trace_depth: 4,
            max_context_surface_tokens: 18000,
            fragment_size_target: 5000, // Canonical 5k tokens for Balanced
            split_sensitivity: 0.50,
            canonical_ownership_sensitivity: 0.70,
            constraints: Vec::new(),
        }
    }

    /// Create default profile for any DocumentationOrientation.
    pub fn preset(orientation: DocumentationOrientation) -> Self {
        match orientation {
            DocumentationOrientation::AiOriented => Self::ai_oriented(),
            DocumentationOrientation::HumanOriented => Self::human_oriented(),
            DocumentationOrientation::Balanced => Self::balanced(),
            DocumentationOrientation::Custom => Self::ai_oriented(), // fallback base for custom
        }
    }
}

impl Default for OptimizationProfile {
    fn default() -> Self {
        Self::ai_oriented()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution_enums_and_profile_serde() {
        let orientation = DocumentationOrientation::AiOriented;
        let serialized = serde_json::to_string(&orientation).unwrap();
        assert_eq!(serialized, "\"ai_oriented\"");
        let deserialized: DocumentationOrientation = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, DocumentationOrientation::AiOriented);

        let profile = OptimizationProfile {
            orientation: DocumentationOrientation::AiOriented,
            cohesion_weight: 0.8,
            coupling_weight: 0.7,
            concentration_weight: 0.7,
            context_efficiency_weight: 0.9,
            discoverability_weight: 0.85,
            narrative_coherence_weight: 0.5,
            maintenance_simplicity_weight: 0.6,
            max_trace_depth: 3,
            max_context_surface_tokens: 12000,
            fragment_size_target: 3000,
            split_sensitivity: 0.7,
            canonical_ownership_sensitivity: 0.8,
            constraints: vec![ConstraintRule {
                id: "c1".to_string(),
                level: ConstraintLevel::HardConstraint,
                rule_key: "never_split_canonical_docs".to_string(),
                description: "Do not split canonical documents".to_string(),
            }],
        };

        let json = serde_json::to_string(&profile).unwrap();
        let de: OptimizationProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(de.orientation, DocumentationOrientation::AiOriented);
        assert_eq!(de.fragment_size_target, 3000);
        assert_eq!(de.constraints.len(), 1);
        assert_eq!(de.constraints[0].level, ConstraintLevel::HardConstraint);
    }

    #[test]
    fn test_orientation_profile_defaults() {
        let ai = OptimizationProfile::ai_oriented();
        assert_eq!(ai.orientation, DocumentationOrientation::AiOriented);
        assert!(ai.context_efficiency_weight >= 0.85);
        assert!(ai.discoverability_weight >= 0.80);
        assert_eq!(ai.fragment_size_target, 3000);

        let human = OptimizationProfile::human_oriented();
        assert_eq!(human.orientation, DocumentationOrientation::HumanOriented);
        assert!(human.narrative_coherence_weight >= 0.85);
        assert!(human.maintenance_simplicity_weight >= 0.75);
        assert_eq!(human.fragment_size_target, 8000);

        let balanced = OptimizationProfile::balanced();
        assert_eq!(balanced.orientation, DocumentationOrientation::Balanced);
        assert_eq!(balanced.fragment_size_target, 5000);
        // Multi-objective compromise: neither exceeds 60/40 ratio
        let ai_efficiency = balanced.context_efficiency_weight;
        let human_readability = balanced.narrative_coherence_weight;
        let total = ai_efficiency + human_readability;
        assert!(ai_efficiency / total <= 0.60);
        assert!(human_readability / total <= 0.60);
    }
}
