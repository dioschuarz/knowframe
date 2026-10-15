//! DocumentNode and FragmentMetadata builders enforcing Document-Centric Graph invariant.
//! Invariant V: Documents are graph nodes (O(N) graph size), headings are fragment metadata.
//! Implements FR-004 and FR-005.

use crate::fragment::FragmentMetadata;
use crate::model::{DocumentNode, ReferenceEdge};

/// Builder for constructing DocumentNode instances with fragment metadata.
pub struct DocumentNodeBuilder {
    id: String,
    relative_path: String,
    title: String,
    raw_token_count: u64,
    frontmatter_schema: Option<String>,
    fragments: Vec<FragmentMetadata>,
    outgoing_references: Vec<ReferenceEdge>,
    git_last_modified_commit: String,
    git_last_modified_timestamp: u64,
}

impl DocumentNodeBuilder {
    pub fn new(relative_path: impl Into<String>) -> Self {
        let path = relative_path.into();
        Self {
            id: path.clone(),
            relative_path: path,
            title: "Untitled".to_string(),
            raw_token_count: 0,
            frontmatter_schema: None,
            fragments: Vec::new(),
            outgoing_references: Vec::new(),
            git_last_modified_commit: String::new(),
            git_last_modified_timestamp: 0,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn raw_token_count(mut self, count: u64) -> Self {
        self.raw_token_count = count;
        self
    }

    pub fn frontmatter_schema(mut self, schema: Option<String>) -> Self {
        self.frontmatter_schema = schema;
        self
    }

    pub fn fragments(mut self, fragments: Vec<FragmentMetadata>) -> Self {
        self.fragments = fragments;
        self
    }

    pub fn outgoing_references(mut self, refs: Vec<ReferenceEdge>) -> Self {
        self.outgoing_references = refs;
        self
    }

    pub fn git_commit(mut self, commit: impl Into<String>, timestamp: u64) -> Self {
        self.git_last_modified_commit = commit.into();
        self.git_last_modified_timestamp = timestamp;
        self
    }

    pub fn build(self) -> DocumentNode {
        DocumentNode {
            id: self.id,
            relative_path: self.relative_path,
            title: self.title,
            raw_token_count: self.raw_token_count,
            frontmatter_schema: self.frontmatter_schema,
            fragments: self.fragments,
            outgoing_references: self.outgoing_references,
            git_last_modified_commit: self.git_last_modified_commit,
            git_last_modified_timestamp: self.git_last_modified_timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_node_builder() {
        let node = DocumentNodeBuilder::new("notes/intro.md")
            .title("Introduction")
            .raw_token_count(250)
            .git_commit("commit1", 1600000000)
            .build();

        assert_eq!(node.id, "notes/intro.md");
        assert_eq!(node.title, "Introduction");
        assert_eq!(node.raw_token_count, 250);
        assert_eq!(node.git_last_modified_commit, "commit1");
    }
}
