//! Section-level fragment metadata and frontmatter materialization.

pub mod concentration;
pub mod extractor;
pub mod frontmatter;

pub use concentration::*;
pub use extractor::*;
pub use frontmatter::*;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Addressable section metadata derived from Markdown AST headings.
/// Invariant V: Fragments are metadata belonging to Document Nodes, NEVER independent graph nodes.
/// Invariant IX: Numerical fields MUST retain complete raw integer representations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FragmentMetadata {
    /// Deterministic anchor derived from heading (e.g. "authentication" or "configuration-1").
    pub anchor: String,
    /// Heading plain text as written in Markdown (e.g. "## Authentication").
    pub heading: String,
    /// 1-indexed start line in source document.
    #[serde(alias = "startLine")]
    pub start_line: usize,
    /// 1-indexed end line (up to next heading of equal/higher rank or EOF).
    #[serde(alias = "endLine")]
    pub end_line: usize,
    /// Raw BPE token count (cl100k_base).
    #[serde(default)]
    pub tokens: u32,
    /// Inbound reference count targeting this fragment.
    #[serde(default)]
    pub inbound: u32,
    /// Outbound reference count originating within this fragment.
    #[serde(default)]
    pub outbound: u32,
}

/// Materialized frontmatter structure stored under `wiki` key in YAML.
/// Invariant VIII: Derived from and validated against Markdown AST during pre-commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MaterializedWikiFrontmatter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub fragments: BTreeMap<String, MaterializedFragmentEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedFragmentEntry {
    pub tokens: u32,
    pub inbound: u32,
    pub outbound: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_model_serialization() {
        let fragment = FragmentMetadata {
            anchor: "authentication".to_string(),
            heading: "Authentication".to_string(),
            start_line: 12,
            end_line: 45,
            tokens: 5132,
            inbound: 31,
            outbound: 7,
        };

        let json = serde_json::to_string(&fragment).unwrap();
        assert!(json.contains("\"tokens\":5132"));
        assert!(json.contains("\"inbound\":31"));
        assert!(json.contains("\"outbound\":7"));

        let deserialized: FragmentMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, fragment);

        let mut frontmatter = MaterializedWikiFrontmatter::default();
        frontmatter.fragments.insert(
            "authentication".to_string(),
            MaterializedFragmentEntry {
                tokens: 5132,
                inbound: 31,
                outbound: 7,
            },
        );

        let yaml = serde_yaml::to_string(&frontmatter).unwrap();
        assert!(yaml.contains("authentication:"));
        assert!(yaml.contains("tokens: 5132"));
    }
}
