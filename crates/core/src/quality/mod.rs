pub mod scorer;
pub mod structure;

pub use scorer::{
    calculate_quality_score, calculate_topological_weight, extract_all_issues,
    extract_all_issues_with_baseline, is_issue_baselined,
};
pub use structure::*;

use crate::debt::DebtSummary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn default_100() -> f32 {
    100.0
}

/// 5-pillar knowledge quality score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeQualityScore {
    pub overall_score: f32,   // 0.0 to 100.0
    pub integrity_score: f32, // 0.0 to 100.0
    #[serde(default = "default_100")]
    pub structure_score: f32, // 0.0 to 100.0 (Pillar 2)
    pub consistency_score: f32, // 0.0 to 100.0
    pub discoverability_score: f32, // 0.0 to 100.0
    pub freshness_score: f32, // 0.0 to 100.0
    pub legacy_debt_count: u32,
    pub new_regressions_count: u32,
    pub resolved_debt_count: u32,
    pub central_hub_id: Option<String>,
    pub top_degradation_factors: Vec<QualityDegradationFactor>,
    #[serde(default)]
    pub total_issues: u32,
    #[serde(default)]
    pub findings_count: u32,
    #[serde(default)]
    pub broken_references: u32,
    #[serde(default)]
    pub invalid_anchors: u32,
    #[serde(default)]
    pub raw_score: Option<f32>,
    #[serde(default)]
    pub is_effective: bool,
    #[serde(default)]
    pub active_issues_count: u32,
}

impl Default for KnowledgeQualityScore {
    fn default() -> Self {
        Self {
            overall_score: 100.0,
            integrity_score: 100.0,
            structure_score: 100.0,
            consistency_score: 100.0,
            discoverability_score: 100.0,
            freshness_score: 100.0,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            resolved_debt_count: 0,
            central_hub_id: None,
            top_degradation_factors: Vec::new(),
            total_issues: 0,
            findings_count: 0,
            broken_references: 0,
            invalid_anchors: 0,
            raw_score: None,
            is_effective: false,
            active_issues_count: 0,
        }
    }
}

/// The 5 core dimensions of knowledge quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualityPillar {
    Integrity,
    Structure,
    Consistency,
    Discoverability,
    Freshness,
}

/// Formal 5-pillar composite quality score with explainable reasons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FivePillarQualityScore {
    pub overall_score: f64,
    pub integrity_score: f64,
    pub structure_score: f64,
    pub consistency_score: f64,
    pub discoverability_score: f64,
    pub freshness_score: f64,
    pub reasons: HashMap<QualityPillar, Vec<String>>,
    pub debt_summary: DebtSummary,
}

impl Default for FivePillarQualityScore {
    fn default() -> Self {
        Self {
            overall_score: 100.0,
            integrity_score: 100.0,
            structure_score: 100.0,
            consistency_score: 100.0,
            discoverability_score: 100.0,
            freshness_score: 100.0,
            reasons: HashMap::new(),
            debt_summary: DebtSummary::default(),
        }
    }
}

/// Topological defect penalty item explaining a deduction on a specific node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityDegradationFactor {
    pub category: String,
    pub affected_node: String,
    pub betweenness_weight: f32,
    pub deduction_points: f32,
    pub description: String,
}

/// Detailed knowledge issue finding categorized by pillar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeIssue {
    pub id: String,
    pub pillar: String, // "integrity" | "consistency" | "discoverability" | "freshness"
    pub category: String,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub target: Option<String>,
    pub deduction_points: f32,
    pub weight: f32,
    pub description: String,
    pub remediation_hint: String,
    #[serde(default)]
    pub is_legacy: bool,
    #[serde(default)]
    pub is_excluded: bool,
}

/// Comprehensive issues report with counts per pillar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeIssuesReport {
    pub overall_score: f32,
    pub total_issues: usize,
    pub integrity_count: usize,
    #[serde(default)]
    pub structure_count: usize,
    pub consistency_count: usize,
    pub discoverability_count: usize,
    pub freshness_count: usize,
    pub issues: Vec<KnowledgeIssue>,
    #[serde(default)]
    pub active_issues_count: usize,
    #[serde(default)]
    pub legacy_debt_count: usize,
    #[serde(default)]
    pub is_quiet_default: bool,
}
