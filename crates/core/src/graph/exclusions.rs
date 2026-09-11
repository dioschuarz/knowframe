//! Exclusion engine using globset for high-speed compiled glob evaluation.

use crate::ipc::protocol::PreviewExclusionResult;
use crate::model::NoiseSuggestion;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ExclusionEngine {
    scan_globs: GlobSet,
    graph_globs: GlobSet,
    pub graph_patterns: Vec<String>,
}

impl ExclusionEngine {
    /// Creates a new ExclusionEngine from two-tier glob pattern slices.
    pub fn new(
        scan_patterns: &[String],
        graph_patterns: &[String],
    ) -> Result<Self, globset::Error> {
        let mut scan_builder = GlobSetBuilder::new();
        for pat in scan_patterns {
            scan_builder.add(Glob::new(pat)?);
        }
        let scan_globs = scan_builder.build()?;

        let mut graph_builder = GlobSetBuilder::new();
        for pat in graph_patterns {
            graph_builder.add(Glob::new(pat)?);
        }
        let graph_globs = graph_builder.build()?;

        Ok(Self {
            scan_globs,
            graph_globs,
            graph_patterns: graph_patterns.to_vec(),
        })
    }

    /// Tests if a relative or absolute POSIX path matches scan.exclude.
    pub fn is_scan_excluded(&self, path: &str) -> bool {
        self.scan_globs.is_match(path)
    }

    /// Tests if a relative or absolute POSIX path matches graph.exclude.
    pub fn is_graph_excluded(&self, path: &str) -> bool {
        self.graph_globs.is_match(path)
    }

    /// Simulates a prospective glob pattern against a collection of workspace paths.
    /// Returns the total matching file count and up to 10 sample paths.
    pub fn preview(
        pattern: &str,
        candidate_paths: &[String],
    ) -> Result<PreviewExclusionResult, globset::Error> {
        let glob = Glob::new(pattern)?.compile_matcher();
        let mut matches = Vec::new();
        let mut count = 0u32;

        for path in candidate_paths {
            if glob.is_match(path) {
                count += 1;
                if matches.len() < 10 {
                    matches.push(path.clone());
                }
            }
        }

        Ok(PreviewExclusionResult {
            matching_count: count,
            sample_paths: matches,
        })
    }

    /// Automated noise detection clustering disconnected orphan files by stem or directory.
    /// Minimum matching threshold: 5 items.
    pub fn detect_noise_suggestions(orphan_paths: &[String]) -> Vec<NoiseSuggestion> {
        let mut suggestions = Vec::new();
        let mut stem_counts: HashMap<String, Vec<String>> = HashMap::new();
        let mut dir_counts: HashMap<String, Vec<String>> = HashMap::new();

        for path in orphan_paths {
            let p = Path::new(path);
            if let Some(file_name) = p.file_name() {
                let name_str = file_name.to_string_lossy().to_string();
                stem_counts.entry(name_str).or_default().push(path.clone());
            }

            let components: Vec<_> = p.components().collect();
            if components.len() >= 2 {
                let top_dir = components[0].as_os_str().to_string_lossy().to_string();
                dir_counts.entry(top_dir).or_default().push(path.clone());
            }
        }

        // Filename stem clustering (e.g. **/README.md)
        for (name, matches) in &stem_counts {
            if matches.len() >= 5 {
                let pattern = format!("**/{}", name);
                suggestions.push(NoiseSuggestion {
                    pattern,
                    matching_count: matches.len() as u32,
                    reason: format!(
                        "{} disconnected {} files found across subdirectories",
                        matches.len(),
                        name
                    ),
                    recommended: true,
                });
            }
        }

        // Directory prefix clustering (e.g. vendor/**, packages/**)
        for (dir, matches) in &dir_counts {
            if matches.len() >= 5 {
                let pattern = format!("{}/**", dir);
                let already_suggested = suggestions.iter().any(|s| s.pattern == pattern);
                if !already_suggested {
                    suggestions.push(NoiseSuggestion {
                        pattern,
                        matching_count: matches.len() as u32,
                        reason: format!(
                            "{} disconnected files in directory '{}'",
                            matches.len(),
                            dir
                        ),
                        recommended: matches.len() >= 10,
                    });
                }
            }
        }

        suggestions.sort_by_key(|a| std::cmp::Reverse(a.matching_count));
        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_tier_exclusion_matching() {
        let scan_ex = vec!["**/.git/**".to_string(), "**/node_modules/**".to_string()];
        let graph_ex = vec!["packages/**/README.md".to_string(), "vendor/**".to_string()];

        let engine = ExclusionEngine::new(&scan_ex, &graph_ex).unwrap();

        assert!(engine.is_scan_excluded("packages/lib/node_modules/index.js"));
        assert!(!engine.is_scan_excluded("packages/lib/README.md"));

        assert!(engine.is_graph_excluded("packages/core/README.md"));
        assert!(engine.is_graph_excluded("vendor/docs/guide.md"));
        assert!(!engine.is_graph_excluded("docs/architecture.md"));
    }

    #[test]
    fn test_exclusion_preview() {
        let candidates = vec![
            "packages/a/README.md".to_string(),
            "packages/b/README.md".to_string(),
            "packages/c/README.md".to_string(),
            "docs/README.md".to_string(),
            "docs/index.md".to_string(),
        ];

        let res = ExclusionEngine::preview("packages/**/README.md", &candidates).unwrap();
        assert_eq!(res.matching_count, 3);
        assert_eq!(res.sample_paths.len(), 3);
    }

    #[test]
    fn test_noise_suggestions_detection() {
        let mut orphans = Vec::new();
        for i in 0..6 {
            orphans.push(format!("packages/pkg{}/README.md", i));
        }
        orphans.push("docs/standalone.md".to_string());

        let suggestions = ExclusionEngine::detect_noise_suggestions(&orphans);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].pattern, "**/README.md");
        assert_eq!(suggestions[0].matching_count, 6);
    }
}
