//! Degree and Brandes betweenness centrality algorithms.
//! High-performance index-based implementation with zero-allocation inner loops.

use std::collections::{HashMap, VecDeque};

/// Computes betweenness centrality for a directed graph represented as an adjacency list.
/// Implements Brandes' algorithm: O(V * E) time, O(V + E) space.
pub fn compute_brandes_betweenness(
    node_ids: &[String],
    adj: &HashMap<String, Vec<String>>,
) -> HashMap<String, f64> {
    let n = node_ids.len();
    if n <= 2 {
        return node_ids.iter().map(|id| (id.clone(), 0.0)).collect();
    }

    // 1. Build index map: id -> usize
    let id_to_idx: HashMap<&str, usize> = node_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| (id.as_str(), idx))
        .collect();

    // 2. Build index-based adjacency list: Vec<Vec<usize>>
    let mut adj_indexed: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (src, neighbors) in adj {
        if let Some(&src_idx) = id_to_idx.get(src.as_str()) {
            for tgt in neighbors {
                if let Some(&tgt_idx) = id_to_idx.get(tgt.as_str()) {
                    adj_indexed[src_idx].push(tgt_idx);
                }
            }
        }
    }
    for list in &mut adj_indexed {
        list.sort();
    }

    // 3. Preallocate reusable data structures across BFS runs
    let mut cb = vec![0.0f64; n];
    let mut stack = Vec::with_capacity(n);
    let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut sigma = vec![0.0f64; n];
    let mut distance = vec![-1i32; n];
    let mut delta = vec![0.0f64; n];
    let mut queue = VecDeque::with_capacity(n);

    // 4. Run Brandes BFS from each source node
    for s in 0..n {
        stack.clear();
        queue.clear();

        for i in 0..n {
            predecessors[i].clear();
            sigma[i] = 0.0;
            distance[i] = -1;
            delta[i] = 0.0;
        }

        sigma[s] = 1.0;
        distance[s] = 0;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let v_dist = distance[v];
            let v_sigma = sigma[v];

            for &w in &adj_indexed[v] {
                // First time discovering w?
                if distance[w] < 0 {
                    distance[w] = v_dist + 1;
                    queue.push_back(w);
                }

                // Shortest path to w via v?
                if distance[w] == v_dist + 1 {
                    sigma[w] += v_sigma;
                    predecessors[w].push(v);
                }
            }
        }

        // Accumulate dependency deltas in reverse topological order
        while let Some(w) = stack.pop() {
            let w_sigma = sigma[w];
            let w_delta = delta[w];

            if w_sigma > 0.0 {
                let factor = (1.0 + w_delta) / w_sigma;
                for &v in &predecessors[w] {
                    delta[v] += sigma[v] * factor;
                }
            }

            if w != s {
                cb[w] += w_delta;
            }
        }
    }

    // 5. Directed normalization scale: 1.0 / ((N - 1) * (N - 2))
    let scale = 1.0 / (((n - 1) * (n - 2)) as f64);
    for val in &mut cb {
        *val *= scale;
    }

    // 6. Map back to node IDs
    node_ids
        .iter()
        .enumerate()
        .map(|(idx, id)| (id.clone(), cb[idx]))
        .collect()
}
