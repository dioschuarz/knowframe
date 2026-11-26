//! Multi-signal document fingerprint extraction and inverted index for candidate pruning.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use super::DocumentFingerprint;

const STOP_WORDS: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "been", "being", "but", "by", "can", "could", "did",
    "do", "does", "for", "from", "had", "has", "have", "in", "is", "it", "its", "not", "of", "on",
    "or", "should", "that", "the", "these", "they", "this", "those", "to", "was", "we", "were",
    "will", "with", "would", "you",
];

#[inline]
fn is_stop_word(word: &str) -> bool {
    STOP_WORDS.binary_search(&word).is_ok()
}

/// Deterministic FNV-1a 64-bit hash for compact zero-String corpus indexing.
#[inline]
pub fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// Deterministic FNV-1a 32-bit hash for bottom-K MinHash 3-gram sketches.
#[inline]
fn fnv1a_32_chars(c0: char, c1: char, c2: char) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    for ch in [c0, c1, c2] {
        let mut buf = [0u8; 4];
        for &b in ch.encode_utf8(&mut buf).as_bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
    }
    // Mix avalanche so bottom-K selection is uniform across the hash space
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x7feb352d);
    hash ^= hash >> 15;
    hash = hash.wrapping_mul(0x846ca68b);
    hash ^= hash >> 16;
    hash
}

/// Extracts cleaned, lowercased word tokens excluding stop words.
pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        let cleaned = raw.trim().to_lowercase();
        if cleaned.len() >= 2 && !is_stop_word(cleaned.as_str()) {
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
        .take(4000)
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
    // 1. Title tokens (file stem + first H1 heading in markdown)
    let file_stem = Path::new(doc_id)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| doc_id.to_string());
    let mut title_tokens: HashSet<String> = tokenize(&file_stem).into_iter().collect();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            let h1_text = trimmed.trim_start_matches("# ").trim();
            for t in tokenize(h1_text) {
                title_tokens.insert(t);
            }
            break;
        }
    }

    // 2. Heading tokens
    let mut heading_tokens = HashSet::new();
    if headings.is_empty() {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                let h_text = trimmed.trim_start_matches('#').trim();
                for t in tokenize(h_text) {
                    heading_tokens.insert(t);
                }
            }
        }
    } else {
        for h in headings {
            for t in tokenize(h) {
                heading_tokens.insert(t);
            }
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

        for term in fp
            .title_tokens
            .iter()
            .chain(fp.heading_tokens.iter())
            .chain(fp.term_frequencies.keys())
        {
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
            for term in fp
                .title_tokens
                .iter()
                .chain(fp.heading_tokens.iter())
                .chain(fp.term_frequencies.keys())
            {
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

        // Body term frequency matches: query top 25 highest-TF terms and skip ubiquitous terms (>20% of corpus)
        let max_doc_freq = (self.doc_to_fingerprint.len() / 5).max(10);
        let mut top_terms: Vec<(&String, &f32)> = query_fp.term_frequencies.iter().collect();
        top_terms.sort_by(|a, b| {
            b.1.partial_cmp(a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(b.0))
        });

        for (term, tf) in top_terms.into_iter().take(25) {
            if let Some(docs) = self.term_to_docs.get(term.as_str()) {
                if docs.len() > max_doc_freq {
                    continue; // Skip ubiquitous common words
                }
                for doc in docs {
                    if doc != &query_fp.doc_id {
                        *doc_scores.entry(doc.clone()).or_insert(0.0) += 1.0 * tf;
                    }
                }
            }
        }

        let mut ranked: Vec<(String, f32)> = doc_scores.into_iter().collect();
        ranked.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });

        ranked
            .into_iter()
            .take(max_candidates)
            .map(|(id, _)| id)
            .collect()
    }
}

// ============================================================================
// Enterprise-Scale Compact Fingerprint & Interned Inverted Index
// Uses ~1.5 KB per document instead of ~250 KB (over 100x lower RAM on 20k+ docs)
// ============================================================================

const COMPACT_TOP_TERMS_CAP: usize = 80;
const COMPACT_MINHASH_K: usize = 64;

/// Memory-compact document fingerprint storing hashed tokens, top-K TF terms,
/// and a fixed-size bottom-K MinHash 3-gram sketch (~1.5 KB per document).
#[derive(Debug, Clone)]
pub struct CompactFingerprint {
    pub doc_idx: u32,
    pub title_hashes: Vec<u64>,
    pub heading_hashes: Vec<u64>,
    /// Sorted by term hash for O(M+N) zero-allocation cosine merge-join.
    pub top_terms: Vec<(u64, f32)>,
    /// Precomputed L2 norm of `top_terms` values.
    pub tf_norm: f32,
    pub tag_hashes: Vec<u64>,
    /// Sorted bottom-K 32-bit FNV-1a hashes of 3-grams for unbiased Jaccard estimation.
    pub minhash: Vec<u32>,
    /// Exact unique 3-gram count (used for summary length-ratio classification).
    pub ngram_count: u32,
    pub has_summary_intent: bool,
}

fn push_sorted_dedup<T: Ord>(v: &mut Vec<T>) {
    v.sort_unstable();
    v.dedup();
}

/// Computes Jaccard similarity over two sorted, deduplicated slices in O(M + N) with 0 allocations.
#[inline]
pub fn sorted_jaccard<T: Ord>(a: &[T], b: &[T]) -> f32 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    let (mut i, mut j, mut inter) = (0usize, 0usize, 0usize);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Equal => {
                inter += 1;
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
        }
    }
    let union = a.len() + b.len() - inter;
    if union == 0 {
        0.0
    } else {
        (inter as f32) / (union as f32)
    }
}

/// Computes bottom-K MinHash Jaccard estimate over two sorted bottom-K sketches.
#[inline]
pub fn minhash_jaccard(a: &[u32], b: &[u32]) -> f32 {
    if a.is_empty() && b.is_empty() {
        return 0.0;
    }
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let k = a.len().min(b.len()).min(COMPACT_MINHASH_K);
    let (mut i, mut j, mut seen, mut inter) = (0usize, 0usize, 0usize, 0usize);
    while seen < k && (i < a.len() || j < b.len()) {
        if i < a.len() && (j >= b.len() || a[i] < b[j]) {
            i += 1;
            seen += 1;
        } else if j < b.len() && (i >= a.len() || b[j] < a[i]) {
            j += 1;
            seen += 1;
        } else {
            inter += 1;
            i += 1;
            j += 1;
            seen += 1;
        }
    }
    if seen == 0 {
        0.0
    } else {
        (inter as f32) / (seen as f32)
    }
}

/// Computes Cosine similarity over two hash-sorted `(u64, f32)` TF slices in O(M + N) with 0 allocations.
#[inline]
pub fn sorted_cosine_tf(a: &[(u64, f32)], norm_a: f32, b: &[(u64, f32)], norm_b: f32) -> f32 {
    if a.is_empty() || b.is_empty() || norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    let (mut i, mut j) = (0usize, 0usize);
    let mut dot = 0.0f32;
    while i < a.len() && j < b.len() {
        match a[i].0.cmp(&b[j].0) {
            std::cmp::Ordering::Equal => {
                dot += a[i].1 * b[j].1;
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
        }
    }
    (dot / (norm_a * norm_b)).min(1.0)
}

/// Extracts an ultra-compact `CompactFingerprint` without retaining any per-token/3-gram `String`s.
pub fn extract_compact_fingerprint(
    doc_idx: u32,
    doc_id: &str,
    content: &str,
    tags: &[String],
) -> CompactFingerprint {
    let mut has_summary_intent = false;

    // 1. Title hashes (file stem + first H1)
    let file_stem = Path::new(doc_id)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| doc_id.to_string());
    let mut title_hashes: Vec<u64> = Vec::new();
    for tok in tokenize(&file_stem) {
        if tok == "summary" || tok == "overview" {
            has_summary_intent = true;
        }
        title_hashes.push(fnv1a_64(tok.as_bytes()));
    }

    let mut heading_hashes: Vec<u64> = Vec::new();
    let mut found_h1 = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let is_h1 = trimmed.starts_with("# ");
            let h_text = trimmed.trim_start_matches('#').trim();
            for tok in tokenize(h_text) {
                let h = fnv1a_64(tok.as_bytes());
                heading_hashes.push(h);
                if is_h1 && !found_h1 {
                    if tok == "summary" || tok == "overview" {
                        has_summary_intent = true;
                    }
                    title_hashes.push(h);
                }
            }
            if is_h1 {
                found_h1 = true;
            }
        }
    }
    push_sorted_dedup(&mut title_hashes);
    push_sorted_dedup(&mut heading_hashes);

    // 2. Body term frequencies (hashed on the fly; keep only top COMPACT_TOP_TERMS_CAP terms)
    let mut raw_counts: HashMap<u64, u32> = HashMap::new();
    let mut total_tokens: u32 = 0;
    for raw in content.split(|c: char| !c.is_alphanumeric()) {
        if raw.len() < 2 {
            continue;
        }
        let cleaned = raw.to_lowercase();
        if cleaned.len() >= 2 && !is_stop_word(cleaned.as_str()) {
            total_tokens = total_tokens.saturating_add(1);
            *raw_counts.entry(fnv1a_64(cleaned.as_bytes())).or_insert(0) += 1;
        }
    }
    let denom = total_tokens.max(1) as f32;
    let mut top_terms: Vec<(u64, f32)> = raw_counts
        .into_iter()
        .map(|(h, c)| (h, (c as f32) / denom))
        .collect();
    if top_terms.len() > COMPACT_TOP_TERMS_CAP {
        top_terms.select_nth_unstable_by(COMPACT_TOP_TERMS_CAP, |a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });
        top_terms.truncate(COMPACT_TOP_TERMS_CAP);
    }
    let tf_norm = top_terms.iter().map(|(_, v)| v * v).sum::<f32>().sqrt();
    top_terms.sort_unstable_by_key(|(h, _)| *h);

    // 3. Tags
    let mut tag_hashes: Vec<u64> = Vec::with_capacity(tags.len());
    for tag in tags {
        let cleaned = tag.trim().trim_start_matches('#').to_lowercase();
        if !cleaned.is_empty() {
            tag_hashes.push(fnv1a_64(cleaned.as_bytes()));
        }
    }
    push_sorted_dedup(&mut tag_hashes);

    // 4. Bottom-K MinHash 3-gram sketch (max 4,000 chars, scratch set dropped immediately)
    let filtered: Vec<char> = content
        .chars()
        .take(4000)
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();

    let (minhash, ngram_count) = if filtered.len() < 3 {
        (Vec::new(), 0u32)
    } else {
        let mut unique_hashes: Vec<u32> = Vec::with_capacity(filtered.len() - 2);
        for i in 0..=(filtered.len() - 3) {
            unique_hashes.push(fnv1a_32_chars(filtered[i], filtered[i + 1], filtered[i + 2]));
        }
        unique_hashes.sort_unstable();
        unique_hashes.dedup();
        let count = unique_hashes.len() as u32;
        if unique_hashes.len() > COMPACT_MINHASH_K {
            unique_hashes.truncate(COMPACT_MINHASH_K);
        }
        unique_hashes.shrink_to_fit();
        (unique_hashes, count)
    };

    CompactFingerprint {
        doc_idx,
        title_hashes,
        heading_hashes,
        top_terms,
        tf_norm,
        tag_hashes,
        minhash,
        ngram_count,
        has_summary_intent,
    }
}

/// Interned inverted index mapping `u64` term hashes to compact `Vec<u32>` doc indices.
pub struct CompactInvertedIndex {
    pub term_to_docs: HashMap<u64, Vec<u32>>,
    pub num_docs: usize,
}

impl CompactInvertedIndex {
    pub fn build(fingerprints: &[CompactFingerprint]) -> Self {
        let num_docs = fingerprints.len();
        let mut term_to_docs: HashMap<u64, Vec<u32>> = HashMap::new();

        for fp in fingerprints {
            let idx = fp.doc_idx;
            for &h in fp
                .title_hashes
                .iter()
                .chain(fp.heading_hashes.iter())
                .chain(fp.top_terms.iter().map(|(h, _)| h))
            {
                let postings = term_to_docs.entry(h).or_default();
                if postings.last().copied() != Some(idx) {
                    postings.push(idx);
                }
            }
        }

        // Drop ubiquitous terms (>20% of corpus or >1,500 docs on enterprise repos) once up front
        let max_postings = (num_docs / 5).max(10).min(1500);
        term_to_docs.retain(|_, v| v.len() <= max_postings);
        term_to_docs.shrink_to_fit();

        Self {
            term_to_docs,
            num_docs,
        }
    }

    /// Queries top-K candidate document indices with ZERO `String` allocations.
    pub fn query_candidates(&self, query_fp: &CompactFingerprint, max_candidates: usize) -> Vec<u32> {
        let mut doc_scores: HashMap<u32, f32> = HashMap::new();
        let self_idx = query_fp.doc_idx;

        for &h in &query_fp.title_hashes {
            if let Some(docs) = self.term_to_docs.get(&h) {
                for &d in docs {
                    if d > self_idx {
                        *doc_scores.entry(d).or_insert(0.0) += 3.0;
                    }
                }
            }
        }

        for &h in &query_fp.heading_hashes {
            if let Some(docs) = self.term_to_docs.get(&h) {
                for &d in docs {
                    if d > self_idx {
                        *doc_scores.entry(d).or_insert(0.0) += 2.0;
                    }
                }
            }
        }

        // Top 25 highest-TF terms
        let mut top_tf: Vec<(u64, f32)> = query_fp.top_terms.clone();
        if top_tf.len() > 25 {
            top_tf.select_nth_unstable_by(25, |a, b| {
                b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
            });
            top_tf.truncate(25);
        }
        for (h, tf) in top_tf {
            if let Some(docs) = self.term_to_docs.get(&h) {
                for &d in docs {
                    if d > self_idx {
                        *doc_scores.entry(d).or_insert(0.0) += tf;
                    }
                }
            }
        }

        let mut ranked: Vec<(u32, f32)> = doc_scores.into_iter().collect();
        if ranked.len() > max_candidates {
            ranked.select_nth_unstable_by(max_candidates, |a, b| {
                b.1.partial_cmp(&a.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.0.cmp(&b.0))
            });
            ranked.truncate(max_candidates);
        }
        ranked.sort_unstable_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });

        ranked.into_iter().map(|(idx, _)| idx).collect()
    }
}
