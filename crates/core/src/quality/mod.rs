pub mod scorer;

pub use scorer::{calculate_quality_score, calculate_topological_weight, extract_all_issues};

use serde::{Deserialize, Serialize};

/// 4-pillar knowledge quality score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeQualityScore {
    pub overall_score: f32, // 0.0 to 100.0
    pub integrity_score: f32, // 0.0 to 100.0
    pub consistency_score: f32, // 0.0 to 100.0
    pub discoverability_score: f32, // 0.0 to 100.0
    pub freshness_score: f32, // 0.0 to 100.0
    pub legacy_debt_count: u32,
    pub new_regressions_count: u32,
    pub resolved_debt_count: u32,
    pub central_hub_id: Option<String>,
    pub top_degradation_factors: Vec<QualityDegradationFactor>,
}

impl Default for KnowledgeQualityScore {
    fn default() -> Self {
        Self {
            overall_score: 100.0,
            integrity_score: 100.0,
            consistency_score: 100.0,
            discoverability_score: 100.0,
            freshness_score: 100.0,
            legacy_debt_count: 0,
            new_regressions_count: 0,
            resolved_debt_count: 0,
            central_hub_id: None,
            top_degradation_factors: Vec::new(),
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
}

/// Comprehensive issues report with counts per pillar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeIssuesReport {
    pub overall_score: f32,
    pub total_issues: usize,
    pub integrity_count: usize,
    pub consistency_count: usize,
    pub discoverability_count: usize,
    pub freshness_count: usize,
    pub issues: Vec<KnowledgeIssue>,
}

