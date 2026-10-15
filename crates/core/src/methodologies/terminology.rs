//! Terminology Management KOS evaluator using Aho-Corasick DFA.
//! Implements FR-022 and SC-008.

use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermDefinition {
    pub canonical: String,
    pub is_deprecated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermMatch {
    pub term: String,
    pub is_deprecated: bool,
    pub start_offset: usize,
    pub end_offset: usize,
}

/// Evaluates terminology occurrences in a document using Aho-Corasick DFA multi-pattern matching.
pub fn evaluate_terminology_occurrences(
    terms: &[TermDefinition],
    document_text: &str,
) -> Vec<TermMatch> {
    if terms.is_empty() || document_text.is_empty() {
        return Vec::new();
    }

    let patterns: Vec<&str> = terms.iter().map(|t| t.canonical.as_str()).collect();
    let ac = match AhoCorasick::new(&patterns) {
        Ok(aut) => aut,
        Err(_) => return Vec::new(),
    };

    let mut matches = Vec::new();
    for mat in ac.find_iter(document_text) {
        let term_def = &terms[mat.pattern()];
        matches.push(TermMatch {
            term: term_def.canonical.clone(),
            is_deprecated: term_def.is_deprecated,
            start_offset: mat.start(),
            end_offset: mat.end(),
        });
    }

    matches
}
