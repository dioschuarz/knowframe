//! Knowledge Organization Systems, Structural Evidence & Trade-Offs,
//! Documentation Orientations & User-Controlled Downstream Automation.

pub mod compiler;
pub mod conflict;
pub mod metrics;
pub mod opportunity;
pub mod plan_value;
pub mod profile;
pub mod re_measure;
pub mod signatures;
pub mod strategies;
pub mod summary;
pub mod transformation;

pub use compiler::*;
pub use conflict::*;
pub use metrics::*;
pub use opportunity::*;
pub use plan_value::*;
pub use profile::*;
pub use re_measure::*;
pub use signatures::*;
pub use strategies::*;
pub use summary::*;
pub use transformation::*;

use crate::config::CustomProfileConfig;

#[derive(Debug, Clone)]
pub struct EvolutionEngine {
    pub orientation: DocumentationOrientation,
    pub profile: OptimizationProfile,
    pub custom_profile: Option<CustomProfileConfig>,
}

impl EvolutionEngine {
    pub fn new(
        orientation: DocumentationOrientation,
        custom_profile: Option<CustomProfileConfig>,
    ) -> Self {
        let profile = Self::compile_active_profile(orientation, custom_profile.as_ref());
        Self {
            orientation,
            profile,
            custom_profile,
        }
    }

    pub fn compile_active_profile(
        orientation: DocumentationOrientation,
        custom: Option<&CustomProfileConfig>,
    ) -> OptimizationProfile {
        match orientation {
            DocumentationOrientation::Custom => {
                let default_custom = CustomProfileConfig::default();
                let custom_cfg = custom.unwrap_or(&default_custom);
                let compiler = ProfileCompiler::new();
                let tier1 = custom_cfg.tier1.clone().unwrap_or_default();
                let tier2 = custom_cfg.tier2.clone();
                let constraints = custom_cfg.constraints.clone();
                compiler.compile_custom(&tier1, tier2.as_ref(), constraints)
            }
            preset => {
                let compiler = ProfileCompiler::new();
                compiler.compile_preset(preset)
            }
        }
    }

    /// Dynamically switches the orientation and recompiles the optimization profile in <= 20 ms without restart (NFR-002, FR-008).
    pub fn switch_orientation(
        &mut self,
        new_orientation: DocumentationOrientation,
    ) -> std::time::Duration {
        let start = std::time::Instant::now();
        self.orientation = new_orientation;
        self.profile = Self::compile_active_profile(new_orientation, self.custom_profile.as_ref());
        start.elapsed()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_dynamic_orientation_switch_latency() {
        let mut engine = EvolutionEngine::new(DocumentationOrientation::AiOriented, None);
        assert_eq!(engine.orientation, DocumentationOrientation::AiOriented);
        assert_eq!(engine.profile.fragment_size_target, 3000);

        // Switch to HumanOriented
        let dur1 = engine.switch_orientation(DocumentationOrientation::HumanOriented);
        assert_eq!(engine.orientation, DocumentationOrientation::HumanOriented);
        assert_eq!(engine.profile.fragment_size_target, 8000);
        assert!(
            dur1 <= std::time::Duration::from_millis(20),
            "Switch to HumanOriented must complete in <= 20ms, took {:?}",
            dur1
        );

        // Switch to Balanced
        let dur2 = engine.switch_orientation(DocumentationOrientation::Balanced);
        assert_eq!(engine.orientation, DocumentationOrientation::Balanced);
        assert_eq!(engine.profile.fragment_size_target, 5000);
        assert!(
            dur2 <= std::time::Duration::from_millis(20),
            "Switch to Balanced must complete in <= 20ms, took {:?}",
            dur2
        );

        // Switch to Custom
        let dur3 = engine.switch_orientation(DocumentationOrientation::Custom);
        assert_eq!(engine.orientation, DocumentationOrientation::Custom);
        assert!(
            dur3 <= std::time::Duration::from_millis(20),
            "Switch to Custom must complete in <= 20ms, took {:?}",
            dur3
        );
    }
}
