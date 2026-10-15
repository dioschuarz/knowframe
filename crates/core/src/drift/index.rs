//! Bidirectional reverse impact index mapping code files and symbols to markdown notes.

use std::collections::HashSet;

use super::extractor::CodeCitation;
use crate::model::{CodeDependency, DependencySource};
pub use super::ReverseImpactIndex;

impl ReverseImpactIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an explicit code dependency with its declaration source.
    pub fn insert_dependency(&mut self, doc_id: &str, dep: &CodeDependency) {
        let doc_id = doc_id.to_string();
        let clean = dep.code_path.trim().replace('\\', "/");

        if !clean.is_empty() {
            self.code_to_docs
                .entry(clean.clone())
                .or_default()
                .insert(doc_id.clone());

            self.doc_to_code
                .entry(doc_id.clone())
                .or_default()
                .insert(clean.clone());

            self.code_to_doc_sources
                .entry(clean)
                .or_default()
                .entry(doc_id)
                .or_default()
                .insert(dep.source);
        }
    }

    /// Inserts a citation linking a markdown document to a code path or symbol.
    pub fn insert_citation(&mut self, doc_id: &str, citation: &CodeCitation) {
        let doc_id = doc_id.to_string();

        if !citation.code_path.is_empty() {
            let clean = citation.code_path.trim().replace('\\', "/");
            self.code_to_docs
                .entry(clean.clone())
                .or_default()
                .insert(doc_id.clone());

            self.doc_to_code
                .entry(doc_id.clone())
                .or_default()
                .insert(clean.clone());

            let source = if citation.symbol.is_some() {
                DependencySource::InlineSymbolCitation
            } else {
                DependencySource::InlineMarkdownLink
            };

            self.code_to_doc_sources
                .entry(clean)
                .or_default()
                .entry(doc_id.clone())
                .or_default()
                .insert(source);
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
                if let Some(doc_map) = self.code_to_doc_sources.get_mut(&code) {
                    doc_map.remove(doc_id);
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

    /// Queries declaration origins for a specific document and code path.
    pub fn get_sources_for_doc_and_code(&self, doc_id: &str, code_path: &str) -> Vec<DependencySource> {
        let clean = code_path.trim().replace('\\', "/");
        if let Some(doc_map) = self.code_to_doc_sources.get(&clean) {
            if let Some(sources) = doc_map.get(doc_id) {
                let mut sorted: Vec<DependencySource> = sources.iter().copied().collect();
                sorted.sort_by_key(|s| match s {
                    DependencySource::Frontmatter => 0,
                    DependencySource::InlineMarkdownLink => 1,
                    DependencySource::InlineSymbolCitation => 2,
                });
                return sorted;
            }
        }

        // Fuzzy match for indexed paths
        for (indexed_path, doc_map) in &self.code_to_doc_sources {
            if indexed_path.ends_with(&clean) || clean.ends_with(indexed_path) {
                if let Some(sources) = doc_map.get(doc_id) {
                    let mut sorted: Vec<DependencySource> = sources.iter().copied().collect();
                    sorted.sort_by_key(|s| match s {
                        DependencySource::Frontmatter => 0,
                        DependencySource::InlineMarkdownLink => 1,
                        DependencySource::InlineSymbolCitation => 2,
                    });
                    return sorted;
                }
            }
        }

        vec![DependencySource::InlineMarkdownLink]
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
