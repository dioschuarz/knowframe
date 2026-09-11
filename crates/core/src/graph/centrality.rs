//! Degree and Brandes betweenness centrality algorithms.

use std::collections::{HashMap, VecDeque};

/// Computes betweenness centrality for a directed graph represented as an adjacency list.
/// Implements Brandes' algorithm: O(V * E) time, O(V + E) space.
pub fn compute_brandes_betweenness(
    node_ids: &[String],
    adj: &HashMap<String, Vec<String>>,
) -> HashMap<String, f64> {
    let mut cb: HashMap<String, f64> = node_ids.iter().map(|id| (id.clone(), 0.0)).collect();
    let n = node_ids.len();
    if n <= 2 {
        return cb;
    }

    for s in node_ids {
        let mut stack = Vec::new();
        let mut predecessors: HashMap<String, Vec<String>> = HashMap::new();
        let mut sigma: HashMap<String, f64> = HashMap::new();
        let mut distance: HashMap<String, i32> = HashMap::new();

        for id in node_ids {
            predecessors.insert(id.clone(), Vec::new());
            sigma.insert(id.clone(), 0.0);
            distance.insert(id.clone(), -1);
        }

        *sigma.get_mut(s).unwrap() = 1.0;
        *distance.get_mut(s).unwrap() = 0;

        let mut queue = VecDeque::new();
        queue.push_back(s.clone());

        while let Some(v) = queue.pop_front() {
            stack.push(v.clone());
            let v_dist = *distance.get(&v).unwrap();
            let v_sigma = *sigma.get(&v).unwrap();

            if let Some(neighbors) = adj.get(&v) {
                for w in neighbors {
                    // w found for the first time?
                    if *distance.get(w).unwrap() < 0 {
                        *distance.get_mut(w).unwrap() = v_dist + 1;
                        queue.push_back(w.clone());
                    }

                    // shortest path to w via v?
                    if *distance.get(w).unwrap() == v_dist + 1 {
                        *sigma.get_mut(w).unwrap() += v_sigma;
                        predecessors.get_mut(w).unwrap().push(v.clone());
                    }
                }
            }
        }

        let mut delta: HashMap<String, f64> = node_ids.iter().map(|id| (id.clone(), 0.0)).collect();

        // S returns vertices in order of non-increasing distance from s
        while let Some(w) = stack.pop() {
            let w_sigma = *sigma.get(&w).unwrap();
            let w_delta = *delta.get(&w).unwrap();

            for v in predecessors.get(&w).unwrap() {
                let v_sigma = *sigma.get(v).unwrap();
                let coeff = (v_sigma / w_sigma) * (1.0 + w_delta);
                *delta.get_mut(v).unwrap() += coeff;
            }

            if w != *s {
                *cb.get_mut(&w).unwrap() += w_delta;
            }
        }
    }

    // Directed normalization scale: 1.0 / ((N - 1) * (N - 2))
    let scale = 1.0 / (((n - 1) * (n - 2)) as f64);
    for val in cb.values_mut() {
        *val *= scale;
    }

    cb
}
