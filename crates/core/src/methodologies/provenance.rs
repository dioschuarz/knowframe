//! Domain Analysis and Provenance KOS evaluators (W3C PROV / Git).
//! Implements FR-018 and FR-025.

use std::collections::HashSet;

/// Computes Git commit co-change coupling between two files.
/// C_prov(A, B) = |Commits(A ∩ B)| / |Commits(A ∪ B)|
pub fn compute_co_change_coupling(
    commits_a: &HashSet<String>,
    commits_b: &HashSet<String>,
) -> f64 {
    let intersection_count = commits_a.intersection(commits_b).count();
    let union_count = commits_a.union(commits_b).count();

    if union_count > 0 {
        intersection_count as f64 / union_count as f64
    } else {
        0.0
    }
}
