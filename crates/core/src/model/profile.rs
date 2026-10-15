//! Unified OptimizationProfile compiler and presets.
//! Implements FR-006, FR-007, FR-008, FR-009, and Constitution Invariant XIV (Unified Profile).

use crate::model::{
    EvolutionMode, OptimizationProfile, ProfileConstraints, ProfileThresholds, ProfileWeights,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ProfileValidationError {
    #[error("Pillar weights must sum to a positive value")]
    ZeroOrNegativePillarWeights,
    #[error("Threshold parameter '{param}' out of valid range [{min}, {max}]")]
    ThresholdOutOfRange {
        param: String,
        min: f64,
        max: f64,
        actual: f64,
    },
}

impl OptimizationProfile {
    /// Compiles the canonical AI-Oriented preset (default).
    /// Optimized for LLM context packing, sub-3-hop navigation, and fine-grained fragment retrieval.
    pub fn ai_oriented() -> Self {
        Self {
            mode: EvolutionMode::AiOriented,
            profile_version: "1.0.0".to_string(),
            weights: ProfileWeights {
                integrity: 0.15,
                structure: 0.25,
                consistency: 0.15,
                discoverability: 0.25,
                freshness: 0.20,
                context_efficiency: 0.30,
                navigation_depth: 0.25,
                vocabulary_adherence: 0.15,
            },
            thresholds: ProfileThresholds {
                max_document_tokens: 3000,
                max_fragment_tokens: 800,
                max_navigation_depth: 3,
                min_scent_similarity: 0.65,
                max_betweenness_centrality: 0.35,
                min_category_coverage: 0.70,
                max_taxonomy_imbalance: 0.25,
            },
            constraints: ProfileConstraints {
                never_auto_split_monoliths: false,
                enforce_controlled_tags: true,
                require_dcap_frontmatter: true,
                prohibit_circular_thesaurus: true,
                isolated_directories: Vec::new(),
            },
        }
    }

    /// Compiles the canonical Human-Oriented preset.
    /// Optimized for deep reading flow, comprehensive document narratives, and hierarchical taxonomies.
    pub fn human_oriented() -> Self {
        Self {
            mode: EvolutionMode::HumanOriented,
            profile_version: "1.0.0".to_string(),
            weights: ProfileWeights {
                integrity: 0.20,
                structure: 0.25,
                consistency: 0.25,
                discoverability: 0.15,
                freshness: 0.15,
                context_efficiency: 0.10,
                navigation_depth: 0.10,
                vocabulary_adherence: 0.25,
            },
            thresholds: ProfileThresholds {
                max_document_tokens: 8000,
                max_fragment_tokens: 2000,
                max_navigation_depth: 5,
                min_scent_similarity: 0.50,
                max_betweenness_centrality: 0.50,
                min_category_coverage: 0.60,
                max_taxonomy_imbalance: 0.40,
            },
            constraints: ProfileConstraints {
                never_auto_split_monoliths: true,
                enforce_controlled_tags: true,
                require_dcap_frontmatter: false,
                prohibit_circular_thesaurus: true,
                isolated_directories: Vec::new(),
            },
        }
    }

    /// Compiles the canonical Balanced preset.
    /// Pragmatic synthesis between human long-form documentation and AI retrieval efficiency.
    pub fn balanced() -> Self {
        Self {
            mode: EvolutionMode::Balanced,
            profile_version: "1.0.0".to_string(),
            weights: ProfileWeights {
                integrity: 0.20,
                structure: 0.20,
                consistency: 0.20,
                discoverability: 0.20,
                freshness: 0.20,
                context_efficiency: 0.20,
                navigation_depth: 0.15,
                vocabulary_adherence: 0.15,
            },
            thresholds: ProfileThresholds {
                max_document_tokens: 5000,
                max_fragment_tokens: 1200,
                max_navigation_depth: 4,
                min_scent_similarity: 0.55,
                max_betweenness_centrality: 0.40,
                min_category_coverage: 0.65,
                max_taxonomy_imbalance: 0.30,
            },
            constraints: ProfileConstraints {
                never_auto_split_monoliths: true,
                enforce_controlled_tags: true,
                require_dcap_frontmatter: false,
                prohibit_circular_thesaurus: true,
                isolated_directories: Vec::new(),
            },
        }
    }

    /// Constructs a profile for the designated EvolutionMode.
    pub fn for_mode(mode: EvolutionMode) -> Self {
        match mode {
            EvolutionMode::AiOriented => Self::ai_oriented(),
            EvolutionMode::HumanOriented => Self::human_oriented(),
            EvolutionMode::Balanced => Self::balanced(),
            EvolutionMode::Custom => {
                let mut profile = Self::balanced();
                profile.mode = EvolutionMode::Custom;
                profile
            }
        }
    }

    /// Validates and normalizes weights and thresholds.
    pub fn validate(&self) -> Result<(), ProfileValidationError> {
        let pillar_sum = self.weights.integrity
            + self.weights.structure
            + self.weights.consistency
            + self.weights.discoverability
            + self.weights.freshness;

        if pillar_sum <= 0.0 {
            return Err(ProfileValidationError::ZeroOrNegativePillarWeights);
        }

        if !(0.0..=1.0).contains(&self.thresholds.min_scent_similarity) {
            return Err(ProfileValidationError::ThresholdOutOfRange {
                param: "min_scent_similarity".to_string(),
                min: 0.0,
                max: 1.0,
                actual: self.thresholds.min_scent_similarity,
            });
        }

        if !(0.0..=1.0).contains(&self.thresholds.max_betweenness_centrality) {
            return Err(ProfileValidationError::ThresholdOutOfRange {
                param: "max_betweenness_centrality".to_string(),
                min: 0.0,
                max: 1.0,
                actual: self.thresholds.max_betweenness_centrality,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_presets_validation() {
        let ai = OptimizationProfile::ai_oriented();
        assert!(ai.validate().is_ok());
        assert_eq!(ai.mode, EvolutionMode::AiOriented);
        assert_eq!(ai.thresholds.max_document_tokens, 3000);

        let human = OptimizationProfile::human_oriented();
        assert!(human.validate().is_ok());
        assert_eq!(human.mode, EvolutionMode::HumanOriented);
        assert_eq!(human.thresholds.max_document_tokens, 8000);

        let balanced = OptimizationProfile::balanced();
        assert!(balanced.validate().is_ok());
        assert_eq!(balanced.mode, EvolutionMode::Balanced);
    }

    #[test]
    fn test_profile_validation_rejects_invalid_weights() {
        let mut profile = OptimizationProfile::balanced();
        profile.weights.integrity = 0.0;
        profile.weights.structure = 0.0;
        profile.weights.consistency = 0.0;
        profile.weights.discoverability = 0.0;
        profile.weights.freshness = 0.0;

        assert_eq!(
            profile.validate(),
            Err(ProfileValidationError::ZeroOrNegativePillarWeights)
        );
    }
}
