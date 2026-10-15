//! Composite Plan Value & Trade-Off Valuation Engine.
//! Implements Invariants IX, X, XV, and FR-008, FR-014.

use super::strategies::KnowledgeOrganizationStrategy;
use serde::{Deserialize, Serialize};

/// Composite Plan Value evaluating the multi-factor architectural utility
/// of an active strategy selection set (FR-008, FR-014, Invariant XV).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompositePlanValue {
    pub base_fit_sum: u32,
    pub synergy_bonus: u32,
    pub overlap_penalty: u32,
    pub transformation_cost: u32,
    pub net_value: i32,
    pub narrative: String,
}

impl Default for CompositePlanValue {
    fn default() -> Self {
        Self {
            base_fit_sum: 0,
            synergy_bonus: 0,
            overlap_penalty: 0,
            transformation_cost: 0,
            net_value: 0,
            narrative: "No strategies selected.".to_string(),
        }
    }
}

/// Returns the transformation effort cost for a strategy.
pub fn get_transformation_cost(strategy: KnowledgeOrganizationStrategy) -> u32 {
    match strategy {
        KnowledgeOrganizationStrategy::PreserveDocument => 0,
        KnowledgeOrganizationStrategy::SectionAddressing => 5,
        KnowledgeOrganizationStrategy::LightweightIndex => 10,
        KnowledgeOrganizationStrategy::FacetedClassification => 15,
        KnowledgeOrganizationStrategy::TopicDecomposition => 25,
        KnowledgeOrganizationStrategy::Hybrid => 15,
    }
}

/// Returns the pairwise synergy bonus for two distinct strategies.
pub fn get_pairwise_synergy(
    a: KnowledgeOrganizationStrategy,
    b: KnowledgeOrganizationStrategy,
) -> u32 {
    use KnowledgeOrganizationStrategy::*;
    match (a, b) {
        (SectionAddressing, LightweightIndex) | (LightweightIndex, SectionAddressing) => 15,
        (FacetedClassification, LightweightIndex) | (LightweightIndex, FacetedClassification) => 12,
        _ => 0,
    }
}

/// Returns the pairwise conceptual overlap penalty for two distinct strategies.
pub fn get_pairwise_overlap(
    a: KnowledgeOrganizationStrategy,
    b: KnowledgeOrganizationStrategy,
) -> u32 {
    use KnowledgeOrganizationStrategy::*;
    match (a, b) {
        (SectionAddressing, LightweightIndex) | (LightweightIndex, SectionAddressing) => 5,
        (FacetedClassification, LightweightIndex) | (LightweightIndex, FacetedClassification) => 3,
        _ => 0,
    }
}

/// Calculates the composite plan value for an active selection of strategies.
pub fn calculate_composite_plan_value(
    selected: &[KnowledgeOrganizationStrategy],
    fit_scores: &[(KnowledgeOrganizationStrategy, u32)],
) -> CompositePlanValue {
    if selected.is_empty() {
        return CompositePlanValue::default();
    }

    let mut base_fit_sum = 0u32;
    let mut transformation_cost = 0u32;

    for &strat in selected {
        let fit = fit_scores
            .iter()
            .find(|(s, _)| *s == strat)
            .map(|(_, score)| *score)
            .unwrap_or(0);
        base_fit_sum += fit;
        transformation_cost += get_transformation_cost(strat);
    }

    let mut synergy_bonus = 0u32;
    let mut overlap_penalty = 0u32;

    for i in 0..selected.len() {
        for j in (i + 1)..selected.len() {
            synergy_bonus += get_pairwise_synergy(selected[i], selected[j]);
            overlap_penalty += get_pairwise_overlap(selected[i], selected[j]);
        }
    }

    let net_value = (base_fit_sum as i32) + (synergy_bonus as i32)
        - (overlap_penalty as i32)
        - (transformation_cost as i32);

    let narrative = if selected.len() > 1 && synergy_bonus > overlap_penalty {
        format!(
            "Combined plan yields net value {} with +{} synergy bonus exceeding {} overlap overhead.",
            net_value, synergy_bonus, overlap_penalty
        )
    } else {
        format!(
            "Active plan yields net value {} (fit: {}, cost: {}).",
            net_value, base_fit_sum, transformation_cost
        )
    };

    CompositePlanValue {
        base_fit_sum,
        synergy_bonus,
        overlap_penalty,
        transformation_cost,
        net_value,
        narrative,
    }
}
