//! AI-Oriented 4-capability matrix evaluator.
//!
//! Synthesizes Entry, Discovery, Retrieval, and Reuse scores (FR-030).
//! Invariant XIV: Documentation governance compiles directly into the active
//! optimization profile with zero parallel disconnected scoring paths.

use super::{
    AIOrientedCapabilityMatrix, HarnessEntryPoint, ProcedureCandidate, RouteResolutionStatus,
    SkillDefinition, SkillValidationStatus,
};
use crate::trace::traversal::HarnessTraceResult;

pub struct AIOrientedCapabilityMatrixEvaluator;

impl AIOrientedCapabilityMatrixEvaluator {
    /// Evaluates the 4 core AI capabilities: Entry, Discovery, Retrieval, Reuse.
    pub fn evaluate(
        harness: Option<&HarnessEntryPoint>,
        skills: &[SkillDefinition],
        procedure_candidates: &[ProcedureCandidate],
        traces: &[HarnessTraceResult],
    ) -> AIOrientedCapabilityMatrix {
        let entry_score = Self::calculate_entry_score(harness);
        let discovery_score = Self::calculate_discovery_score(harness);
        let retrieval_score = Self::calculate_retrieval_score(harness, traces);
        let reuse_score = Self::calculate_reuse_score(skills, procedure_candidates);

        let overall_governance_score =
            (entry_score * 0.25 + discovery_score * 0.25 + retrieval_score * 0.25 + reuse_score * 0.25)
                .clamp(0.0, 100.0);

        AIOrientedCapabilityMatrix {
            entry_score,
            discovery_score,
            retrieval_score,
            reuse_score,
            overall_governance_score,
        }
    }

    fn calculate_entry_score(harness: Option<&HarnessEntryPoint>) -> f32 {
        let Some(h) = harness else {
            return 0.0;
        };

        let ratings = &h.quality_ratings;
        let mut score = ratings.integrity_score * 0.45 + ratings.structure_score * 0.35;

        // Bonus for clean direct routing (no secondary routing layers)
        if h.secondary_layers_detected.is_empty() {
            score += 20.0;
        } else {
            // Penalty per secondary layer
            let penalty: f32 = h.secondary_layers_detected.iter().map(|s| s.penalty_points).sum();
            score = (score - penalty).max(0.0);
        }

        score.clamp(0.0, 100.0)
    }

    fn calculate_discovery_score(harness: Option<&HarnessEntryPoint>) -> f32 {
        let Some(h) = harness else {
            return 0.0;
        };

        let Some(idx) = &h.embedded_index else {
            return (h.quality_ratings.discoverability_score * 0.5).clamp(0.0, 100.0);
        };

        let coverage = idx.domain_coverage_ratio;
        let resolved_ratio = if idx.routes.is_empty() {
            1.0
        } else {
            let resolved = idx
                .routes
                .iter()
                .filter(|r| r.resolution_status == RouteResolutionStatus::Resolved)
                .count();
            resolved as f32 / idx.routes.len() as f32
        };

        (coverage * 60.0 + resolved_ratio * 40.0).clamp(0.0, 100.0)
    }

    fn calculate_retrieval_score(
        harness: Option<&HarnessEntryPoint>,
        traces: &[HarnessTraceResult],
    ) -> f32 {
        let base_efficiency = harness
            .map(|h| h.quality_ratings.context_efficiency)
            .unwrap_or(75.0);

        if traces.is_empty() {
            return base_efficiency;
        }

        let mut hop_scores = Vec::new();
        for trace in traces {
            // Target is <= 3 hops (SC-002)
            let hop_score = match trace.total_hops {
                0..=1 => 100.0,
                2 => 90.0,
                3 => 80.0,
                4 => 60.0,
                _ => 40.0,
            };
            hop_scores.push(hop_score);
        }

        let avg_hop_score = hop_scores.iter().sum::<f32>() / hop_scores.len() as f32;
        (avg_hop_score * 0.6 + base_efficiency * 0.4).clamp(0.0, 100.0)
    }

    fn calculate_reuse_score(
        skills: &[SkillDefinition],
        procedure_candidates: &[ProcedureCandidate],
    ) -> f32 {
        let skill_component = if skills.is_empty() {
            75.0 // Neutral baseline if repository has not adopted skills yet
        } else {
            let valid_count = skills
                .iter()
                .filter(|s| s.status == SkillValidationStatus::Valid)
                .count();
            (valid_count as f32 / skills.len() as f32) * 100.0
        };

        // If unpromoted high-recurrence procedures exist, deduct slight score to encourage promotion
        let unpromoted_penalty = (procedure_candidates.len() as f32 * 4.0).min(20.0);

        (skill_component - unpromoted_penalty).clamp(0.0, 100.0)
    }
}
