//! Multi-signal document fingerprint extraction and inverted index for candidate pruning.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::DocumentFingerprint;

const STOP_WORDS: &[&str] = &[
    "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by",
    "from", "as", "is", "was", "are", "were", "be", "been", "being", "have", "has", "had",
    "do", "does", "did", "this", "that", "these", "those", "it", "its", "we", "you", "they",
    "can", "will", "would", "should", "could", "not",
];

/// Extracts cleaned, lowercased word tokens excluding stop words.
pub fn tokenize(text: &str) -> Vec<String> {
    let stop_set: HashSet<&'static str> = STOP_WORDS.iter().copied().collect();
    let mut tokens = Vec::new();

    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        let cleaned = raw.trim().to_lowercase();
        if cleaned.len() >= 2 && !stop_set.contains(cleaned.as_str()) {
            tokens.push(cleaned);
        }
    }

    tokens
}

/// Extracts character n-grams (default 3-grams) from text.
pub fn extract_char_ngrams(text: &str, n: usize) -> HashSet<String> {
    let mut ngrams = HashSet::new();
    let filtered: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();

    if filtered.len() < n {
        return ngrams;
    }

    for i in 0..=(filtered.len() - n) {
        let shingle: String = filtered[i..i + n].iter().collect();
        ngrams.insert(shingle);
    }

    ngrams
}

/// Extracts a deterministic multi-signal `DocumentFingerprint` from a document.
pub fn extract_fingerprint(
    doc_id: &str,
    content: &str,
    headings: &[String],
    tags: &[String],
) -> DocumentFingerprint {
    // 1. Title tokens
    let file_stem = Path::new(doc_id)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| doc_id.to_string());
    let title_tokens: HashSet<String> = tokenize(&file_stem).into_iter().collect();

    // 2. Heading tokens
    let mut heading_tokens = HashSet::new();
    for h in headings {
        for t in tokenize(h) {
            heading_tokens.insert(t);
        }
    }

    // 3. Normalized Term Frequencies (TF)
    let body_tokens = tokenize(content);
    let total_tokens = body_tokens.len().max(1) as f32;
    let mut raw_counts: HashMap<String, u32> = HashMap::new();
    for t in &body_tokens {
        *raw_counts.entry(t.clone()).or_insert(0) += 1;
    }

    let mut term_frequencies: HashMap<String, f32> = HashMap::new();
    for (term, count) in raw_counts {
        term_frequencies.insert(term, (count as f32) / total_tokens);
    }

    // 4. Tags
    let mut tag_set = HashSet::new();
    for tag in tags {
        let cleaned = tag.trim().trim_start_matches('#').to_lowercase();
        if !cleaned.is_empty() {
            tag_set.insert(cleaned);
        }
    }

    // 5. 3-grams
    let ngrams = extract_char_ngrams(content, 3);

    DocumentFingerprint {
        doc_id: doc_id.to_string(),
        title_tokens,
        heading_tokens,
        term_frequencies,
        tags: tag_set,
        ngrams,
    }
}

/// Inverted term index enabling O(1) to O(K) candidate document retrieval.
#[derive(Debug, Clone, Default)]
pub struct InvertedIndex {
    pub term_to_docs: HashMap<String, HashSet<String>>,
    pub doc_to_fingerprint: HashMap<String, DocumentFingerprint>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Indexes a document fingerprint.
    pub fn insert(&mut self, fp: DocumentFingerprint) {
        let doc_id = fp.doc_id.clone();

        for term in fp.title_tokens.iter().chain(fp.heading_tokens.iter()).chain(fp.term_frequencies.keys()) {
            self.term_to_docs
                .entry(term.clone())
                .or_default()
                .insert(doc_id.clone());
        }

        self.doc_to_fingerprint.insert(doc_id, fp);
    }

    /// Removes a document from the inverted index.
    pub fn remove(&mut self, doc_id: &str) {
        if let Some(fp) = self.doc_to_fingerprint.remove(doc_id) {
            for term in fp.title_tokens.iter().chain(fp.heading_tokens.iter()).chain(fp.term_frequencies.keys()) {
                if let Some(set) = self.term_to_docs.get_mut(term) {
                    set.remove(doc_id);
                }
            }
        }
    }

    /// Queries top-K candidate matching document IDs without O(N^2) evaluation.
    pub fn query_candidates(
        &self,
        query_fp: &DocumentFingerprint,
        max_candidates: usize,
    ) -> Vec<String> {
        let mut doc_scores: HashMap<String, f32> = HashMap::new();

        // Title matches weighted 3.0
        for term in &query_fp.title_tokens {
            if let Some(docs) = self.term_to_docs.get(term) {
                for doc in docs {
                    if doc != &query_fp.doc_id {
                        *doc_scores.entry(doc.clone()).or_insert(0.0) += 3.0;
                    }
                }
            }
        }

        // Heading matches weighted 2.0
        for term in &query_fp.heading_tokens {
            if let Some(docs) = self.term_to_docs.get(term) {
                for doc in docs {
                    if doc != &query_fp.doc_id {
                        *doc_scores.entry(doc.clone()).or_insert(0.0) += 2.0;
                    }
                }
            }
        }

        // Body term frequency matches weighted 1.0
        for (term, tf) in &query_fp.term_frequencies {
            if let Some(docs) = self.term_to_docs.get(term) {
                for doc in docs {
                    if doc != &query_fp.doc_id {
                        *doc_scores.entry(doc.clone()).or_insert(0.0) += 1.0 * tf;
                    }
                }
            }
        }

        let mut ranked: Vec<(String, f32)> = doc_scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        ranked
            .into_iter()
            .take(max_candidates)
            .map(|(id, _)| id)
            .collect()
    }
}
