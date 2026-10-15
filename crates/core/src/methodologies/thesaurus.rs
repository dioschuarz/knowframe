//! Controlled Vocabulary and Thesaurus KOS evaluators (ISO 25964 / SKOS).
//! Implements FR-013, FR-014, and SC-008.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strsim::levenshtein;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearDuplicate {
    pub observed: String,
    pub suggested: String,
    pub distance: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledVocabularyResult {
    pub total_observed: usize,
    pub uncontrolled_count: usize,
    pub uncontrolled_ratio: f64,
    pub near_duplicates: Vec<NearDuplicate>,
}

/// Evaluates observed terms against an authorized controlled vocabulary.
pub fn evaluate_controlled_vocabulary(
    authorized: &[String],
    observed: &[String],
) -> ControlledVocabularyResult {
    let auth_set: HashSet<_> = authorized.iter().cloned().collect();
    let mut uncontrolled_count = 0;
    let mut near_duplicates = Vec::new();

    for term in observed {
        if !auth_set.contains(term) {
            uncontrolled_count += 1;
            // Check for close typos (distance <= 2)
            let mut best_match: Option<(&String, usize)> = None;
            for auth in authorized {
                let dist = levenshtein(term, auth);
                if dist <= 2 {
                    if best_match.as_ref().map_or(true, |(_, d)| dist < *d) {
                        best_match = Some((auth, dist));
                    }
                }
            }
            if let Some((suggested, distance)) = best_match {
                near_duplicates.push(NearDuplicate {
                    observed: term.clone(),
                    suggested: suggested.clone(),
                    distance,
                });
            }
        }
    }

    let uncontrolled_ratio = if !observed.is_empty() {
        uncontrolled_count as f64 / observed.len() as f64
    } else {
        0.0
    };

    ControlledVocabularyResult {
        total_observed: observed.len(),
        uncontrolled_count,
        uncontrolled_ratio,
        near_duplicates,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThesaurusRelation {
    BroaderTerm,
    NarrowerTerm,
    RelatedTerm,
    Use,
    UsedFor,
}

#[derive(Debug, Clone, Default)]
pub struct ThesaurusGraph {
    pub relations: Vec<(String, String, ThesaurusRelation)>,
}

impl ThesaurusGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_relation(&mut self, source: &str, target: &str, relation: ThesaurusRelation) {
        self.relations.push((source.to_string(), target.to_string(), relation));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReciprocalViolation {
    pub source: String,
    pub target: String,
    pub expected_relation: ThesaurusRelation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesaurusReciprocityReport {
    pub total_relations: usize,
    pub reciprocal_violations: Vec<ReciprocalViolation>,
    pub violation_ratio: f64,
}

/// Evaluates thesaurus reciprocity according to ISO 25964 standards.
pub fn evaluate_thesaurus_reciprocity(graph: &ThesaurusGraph) -> ThesaurusReciprocityReport {
    let mut relation_map: HashMap<(String, String), HashSet<ThesaurusRelation>> = HashMap::new();

    for (s, t, r) in &graph.relations {
        relation_map.entry((s.clone(), t.clone())).or_default().insert(*r);
    }

    let mut violations = Vec::new();

    for (s, t, r) in &graph.relations {
        match r {
            ThesaurusRelation::BroaderTerm => {
                let has_nt = relation_map
                    .get(&(t.clone(), s.clone()))
                    .map_or(false, |set| set.contains(&ThesaurusRelation::NarrowerTerm));
                if !has_nt {
                    violations.push(ReciprocalViolation {
                        source: s.clone(),
                        target: t.clone(),
                        expected_relation: ThesaurusRelation::NarrowerTerm,
                    });
                }
            }
            ThesaurusRelation::NarrowerTerm => {
                let has_bt = relation_map
                    .get(&(t.clone(), s.clone()))
                    .map_or(false, |set| set.contains(&ThesaurusRelation::BroaderTerm));
                if !has_bt {
                    violations.push(ReciprocalViolation {
                        source: s.clone(),
                        target: t.clone(),
                        expected_relation: ThesaurusRelation::BroaderTerm,
                    });
                }
            }
            ThesaurusRelation::RelatedTerm => {
                let has_rt = relation_map
                    .get(&(t.clone(), s.clone()))
                    .map_or(false, |set| set.contains(&ThesaurusRelation::RelatedTerm));
                if !has_rt {
                    violations.push(ReciprocalViolation {
                        source: s.clone(),
                        target: t.clone(),
                        expected_relation: ThesaurusRelation::RelatedTerm,
                    });
                }
            }
            ThesaurusRelation::Use => {
                let has_uf = relation_map
                    .get(&(t.clone(), s.clone()))
                    .map_or(false, |set| set.contains(&ThesaurusRelation::UsedFor));
                if !has_uf {
                    violations.push(ReciprocalViolation {
                        source: s.clone(),
                        target: t.clone(),
                        expected_relation: ThesaurusRelation::UsedFor,
                    });
                }
            }
            ThesaurusRelation::UsedFor => {
                let has_use = relation_map
                    .get(&(t.clone(), s.clone()))
                    .map_or(false, |set| set.contains(&ThesaurusRelation::Use));
                if !has_use {
                    violations.push(ReciprocalViolation {
                        source: s.clone(),
                        target: t.clone(),
                        expected_relation: ThesaurusRelation::Use,
                    });
                }
            }
        }
    }

    let total = graph.relations.len();
    let violation_ratio = if total > 0 {
        violations.len() as f64 / total as f64
    } else {
        0.0
    };

    ThesaurusReciprocityReport {
        total_relations: total,
        reciprocal_violations: violations,
        violation_ratio,
    }
}
