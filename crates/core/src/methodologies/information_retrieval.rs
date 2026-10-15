//! Information Architecture and BM25F Information Retrieval evaluators.
//! Implements FR-020 and FR-021.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationArchitectureEvaluation {
    pub max_depth: usize,
    pub mean_scent_score: f64,
    pub dead_ends: Vec<String>,
}

fn tokenize_words(text: &str) -> HashSet<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect()
}

fn jaccard_similarity(set_a: &HashSet<String>, set_b: &HashSet<String>) -> f64 {
    let intersection = set_a.intersection(set_b).count();
    let union = set_a.union(set_b).count();
    if union > 0 {
        intersection as f64 / union as f64
    } else {
        0.0
    }
}

/// Evaluates Information Architecture metrics: navigation depth, scent proxy, and dead-ends.
pub fn evaluate_information_architecture(
    root_id: &str,
    edges: &[(String, String, String)], // (source, target, label)
    node_titles: &[(String, String)],   // (node_id, title)
) -> InformationArchitectureEvaluation {
    let title_map: HashMap<String, String> = node_titles.iter().cloned().collect();
    let mut adj: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut all_nodes = HashSet::new();

    for (s, t, label) in edges {
        all_nodes.insert(s.clone());
        all_nodes.insert(t.clone());
        adj.entry(s.clone()).or_default().push((t.clone(), label.clone()));
    }

    // BFS for navigation depth from root
    let mut depth_map: HashMap<String, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    depth_map.insert(root_id.to_string(), 0);
    queue.push_back(root_id.to_string());

    while let Some(curr) = queue.pop_front() {
        let d = depth_map[&curr];
        if let Some(neighbors) = adj.get(&curr) {
            for (nxt, _) in neighbors {
                if !depth_map.contains_key(nxt) {
                    depth_map.insert(nxt.clone(), d + 1);
                    queue.push_back(nxt.clone());
                }
            }
        }
    }

    let max_depth = depth_map.values().copied().max().unwrap_or(0);

    // Compute scent proxy for each edge
    let mut scent_scores = Vec::new();
    for (_, target, label) in edges {
        let label_tokens = tokenize_words(label);
        let target_title = title_map.get(target).cloned().unwrap_or_else(|| target.clone());
        let title_tokens = tokenize_words(&target_title);
        let scent = jaccard_similarity(&label_tokens, &title_tokens);
        scent_scores.push(scent);
    }

    let mean_scent_score = if !scent_scores.is_empty() {
        scent_scores.iter().sum::<f64>() / scent_scores.len() as f64
    } else {
        0.0
    };

    // Detect dead-ends (nodes reached with 0 outgoing edges)
    let mut dead_ends = Vec::new();
    for node in all_nodes {
        if !adj.contains_key(&node) || adj[&node].is_empty() {
            dead_ends.push(node);
        }
    }
    dead_ends.sort();

    InformationArchitectureEvaluation {
        max_depth,
        mean_scent_score,
        dead_ends,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFieldContent {
    pub id: String,
    pub title: String,
    pub headings: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BM25SearchResult {
    pub document_id: String,
    pub score: f64,
}

/// Evaluates multi-field BM25F ranking across documents.
pub fn evaluate_bm25f(query: &str, docs: &[DocumentFieldContent]) -> Vec<BM25SearchResult> {
    let query_terms: Vec<String> = query
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect();

    if query_terms.is_empty() || docs.is_empty() {
        return Vec::new();
    }

    let n = docs.len() as f64;
    let k1 = 1.2;

    // Field weights
    let w_title = 3.0;
    let w_headings = 2.0;
    let w_body = 1.0;

    // Field b parameters
    let b_title = 0.75;
    let b_headings = 0.75;
    let b_body = 0.75;

    // Compute average field lengths
    let mut avg_len_title = 0.0;
    let mut avg_len_headings = 0.0;
    let mut avg_len_body = 0.0;

    for d in docs {
        avg_len_title += tokenize_words(&d.title).len() as f64;
        let h_text = d.headings.join(" ");
        avg_len_headings += tokenize_words(&h_text).len() as f64;
        avg_len_body += tokenize_words(&d.body).len() as f64;
    }
    avg_len_title = (avg_len_title / n).max(1.0);
    avg_len_headings = (avg_len_headings / n).max(1.0);
    avg_len_body = (avg_len_body / n).max(1.0);

    // Compute document frequency (DF) for each query term
    let mut df: HashMap<String, usize> = HashMap::new();
    for term in &query_terms {
        for d in docs {
            let combined = format!("{} {} {}", d.title, d.headings.join(" "), d.body).to_lowercase();
            if combined.contains(term) {
                *df.entry(term.clone()).or_default() += 1;
            }
        }
    }

    let mut results = Vec::new();

    for d in docs {
        let title_words: Vec<_> = d.title.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(String::from).collect();
        let h_words: Vec<_> = d.headings.join(" ").to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(String::from).collect();
        let body_words: Vec<_> = d.body.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(String::from).collect();

        let len_title = title_words.len() as f64;
        let len_headings = h_words.len() as f64;
        let len_body = body_words.len() as f64;

        let mut doc_score = 0.0;

        for term in &query_terms {
            let n_t = *df.get(term).unwrap_or(&0) as f64;
            let idf = ((n - n_t + 0.5) / (n_t + 0.5) + 1.0).ln();

            let tf_title = title_words.iter().filter(|w| *w == term).count() as f64;
            let tf_headings = h_words.iter().filter(|w| *w == term).count() as f64;
            let tf_body = body_words.iter().filter(|w| *w == term).count() as f64;

            let c_title = tf_title / (1.0 - b_title + b_title * (len_title / avg_len_title));
            let c_headings = tf_headings / (1.0 - b_headings + b_headings * (len_headings / avg_len_headings));
            let c_body = tf_body / (1.0 - b_body + b_body * (len_body / avg_len_body));

            let c_tilde = w_title * c_title + w_headings * c_headings + w_body * c_body;

            if c_tilde > 0.0 {
                doc_score += idf * (c_tilde / (k1 + c_tilde));
            }
        }

        if doc_score > 0.0 {
            results.push(BM25SearchResult {
                document_id: d.id.clone(),
                score: doc_score,
            });
        }
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results
}
