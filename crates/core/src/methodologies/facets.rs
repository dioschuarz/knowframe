//! Faceted Classification KOS evaluator.
//! Implements FR-012: Pairwise Mutual Information (PMI) and Facet Orthogonality.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFacets {
    pub document_id: String,
    pub facets: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacetOrthogonalityResult {
    pub orthogonality: f64,
    pub facet_pairs: Vec<(String, String, f64)>,
}

/// Computes facet orthogonality across document faceted metadata.
pub fn compute_facet_orthogonality(docs: &[DocumentFacets]) -> FacetOrthogonalityResult {
    if docs.is_empty() {
        return FacetOrthogonalityResult {
            orthogonality: 1.0,
            facet_pairs: Vec::new(),
        };
    }

    // Collect all facet keys
    let mut facet_keys: HashSet<String> = HashSet::new();
    for doc in docs {
        for (k, _) in &doc.facets {
            facet_keys.insert(k.clone());
        }
    }

    let mut keys: Vec<String> = facet_keys.into_iter().collect();
    keys.sort();

    if keys.len() < 2 {
        return FacetOrthogonalityResult {
            orthogonality: 1.0,
            facet_pairs: Vec::new(),
        };
    }

    let n = docs.len() as f64;
    let mut facet_pairs = Vec::new();
    let mut sum_normalized_pmi = 0.0;
    let mut pair_count = 0;

    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            let k1 = &keys[i];
            let k2 = &keys[j];

            let mut count_k1: HashMap<String, usize> = HashMap::new();
            let mut count_k2: HashMap<String, usize> = HashMap::new();
            let mut joint_counts: HashMap<(String, String), usize> = HashMap::new();

            for doc in docs {
                let v1 = doc.facets.iter().find(|(k, _)| k == k1).map(|(_, v)| v.as_str());
                let v2 = doc.facets.iter().find(|(k, _)| k == k2).map(|(_, v)| v.as_str());

                if let (Some(val1), Some(val2)) = (v1, v2) {
                    *count_k1.entry(val1.to_string()).or_default() += 1;
                    *count_k2.entry(val2.to_string()).or_default() += 1;
                    *joint_counts.entry((val1.to_string(), val2.to_string())).or_default() += 1;
                }
            }

            let mut mi = 0.0;
            for ((val1, val2), joint_c) in &joint_counts {
                let p_joint = *joint_c as f64 / n;
                let p_k1 = count_k1[val1] as f64 / n;
                let p_k2 = count_k2[val2] as f64 / n;

                if p_joint > 0.0 && p_k1 > 0.0 && p_k2 > 0.0 {
                    mi += p_joint * (p_joint / (p_k1 * p_k2)).log2();
                }
            }

            // Calculate entropy of k1 and k2 for normalization
            let mut h1 = 0.0;
            for c in count_k1.values() {
                let p = *c as f64 / n;
                if p > 0.0 {
                    h1 -= p * p.log2();
                }
            }
            let mut h2 = 0.0;
            for c in count_k2.values() {
                let p = *c as f64 / n;
                if p > 0.0 {
                    h2 -= p * p.log2();
                }
            }

            let min_h = h1.min(h2);
            let normalized_mi = if min_h > 0.0 {
                (mi / min_h).clamp(0.0, 1.0)
            } else {
                0.0
            };

            facet_pairs.push((k1.clone(), k2.clone(), normalized_mi));
            sum_normalized_pmi += normalized_mi;
            pair_count += 1;
        }
    }

    let mean_normalized_pmi = if pair_count > 0 {
        sum_normalized_pmi / pair_count as f64
    } else {
        0.0
    };

    let orthogonality = (1.0 - mean_normalized_pmi).clamp(0.0, 1.0);

    FacetOrthogonalityResult {
        orthogonality,
        facet_pairs,
    }
}
