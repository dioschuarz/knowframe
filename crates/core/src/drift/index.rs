//! Bidirectional reverse impact index mapping code files and symbols to markdown notes.

use std::collections::HashSet;

use super::extractor::CodeCitation;
pub use super::ReverseImpactIndex;

impl ReverseImpactIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a citation linking a markdown document to a code path or symbol.
    pub fn insert_citation(&mut self, doc_id: &str, citation: &CodeCitation) {
        let doc_id = doc_id.to_string();

        if !citation.code_path.is_empty() {
            self.code_to_docs
                .entry(citation.code_path.clone())
                .or_default()
                .insert(doc_id.clone());

            self.doc_to_code
                .entry(doc_id.clone())
                .or_default()
                .insert(citation.code_path.clone());
        }

        if let Some(ref sym) = citation.symbol {
            self.symbol_to_docs
                .entry(sym.clone())
                .or_default()
                .insert(doc_id);
        }
    }

    /// Removes a document and all its outgoing citations from the index.
    pub fn remove_doc(&mut self, doc_id: &str) {
        if let Some(codes) = self.doc_to_code.remove(doc_id) {
            for code in codes {
                if let Some(docs) = self.code_to_docs.get_mut(&code) {
                    docs.remove(doc_id);
                }
            }
        }

        for docs in self.symbol_to_docs.values_mut() {
            docs.remove(doc_id);
        }
    }

    /// Queries all Markdown documents citing a specific code file path.
    pub fn get_docs_for_code(&self, code_path: &str) -> HashSet<String> {
        let clean = code_path.trim().replace('\\', "/");
        let mut results = HashSet::new();

        if let Some(docs) = self.code_to_docs.get(&clean) {
            results.extend(docs.iter().cloned());
        }

        // Also check if path without leading dirs matches
        for (indexed_path, docs) in &self.code_to_docs {
            if indexed_path.ends_with(&clean) || clean.ends_with(indexed_path) {
                results.extend(docs.iter().cloned());
            }
        }

        results
    }

    /// Queries all Markdown documents citing a specific code symbol.
    pub fn get_docs_for_symbol(&self, symbol: &str) -> HashSet<String> {
        self.symbol_to_docs.get(symbol).cloned().unwrap_or_default()
    }

    /// Queries all code file paths cited in a specific Markdown document.
    pub fn get_code_for_doc(&self, doc_id: &str) -> HashSet<String> {
        self.doc_to_code.get(doc_id).cloned().unwrap_or_default()
    }
}
