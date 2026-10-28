//! Relative path resolver, multi-repo/submodule traversal, and dead link & anchor detector.

pub use crate::fragment::extractor::slugify_heading;
use crate::model::{LinkResolutionStatus, LinkType, NoteEdge};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Precomputed lookup index for O(1) canonical, case-folded, and basename resolution.
pub struct ResolverIndex<'a> {
    pub known_nodes: &'a HashMap<String, PathBuf>,
    pub case_folded: HashMap<String, String>,
    pub basename_map: HashMap<String, Vec<String>>,
    pub excluded_nodes: HashSet<String>,
}

impl<'a> ResolverIndex<'a> {
    pub fn new(known_nodes: &'a HashMap<String, PathBuf>) -> Self {
        Self::with_exclusions(known_nodes, HashSet::new())
    }

    pub fn with_exclusions(
        known_nodes: &'a HashMap<String, PathBuf>,
        excluded_nodes: HashSet<String>,
    ) -> Self {
        let mut sorted_keys: Vec<&String> = known_nodes.keys().collect();
        sorted_keys.sort();

        let mut case_folded = HashMap::with_capacity(known_nodes.len());
        let mut basename_map: HashMap<String, Vec<String>> = HashMap::with_capacity(known_nodes.len());

        for k in sorted_keys {
            case_folded.insert(k.to_lowercase(), k.clone());
            if let Some(file_name) = Path::new(k).file_name() {
                let name = file_name.to_string_lossy().to_lowercase();
                let stem = Path::new(k)
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_lowercase())
                    .unwrap_or_else(|| name.clone());

                basename_map.entry(name.clone()).or_default().push(k.clone());
                if stem != name {
                    basename_map.entry(stem).or_default().push(k.clone());
                }
            }
        }

        Self {
            known_nodes,
            case_folded,
            basename_map,
            excluded_nodes,
        }
    }
}

/// Given a list of candidate file paths for a basename/stem and the source file path,
/// chooses the best candidate using deterministic Obsidian shortest-path rules:
/// 1. Candidate in the exact same directory as the source note.
/// 2. Candidate with maximum shared common path components (closest ancestor directory).
/// 3. Candidate with minimum path depth (fewest components, closest to root).
/// 4. Strict alphabetical order as tie-breaker.
pub fn resolve_best_basename_candidate(candidates: &[String], source_path: &str) -> Option<String> {
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() == 1 {
        return Some(candidates[0].clone());
    }

    let source_dir = Path::new(source_path).parent().unwrap_or_else(|| Path::new(""));

    // 1. Same directory match
    for cand in candidates {
        let cand_dir = Path::new(cand).parent().unwrap_or_else(|| Path::new(""));
        if cand_dir == source_dir {
            return Some(cand.clone());
        }
    }

    // 2. Maximum shared prefix components with source_path
    let source_components: Vec<_> = source_dir.components().collect();

    let mut best_cand = &candidates[0];
    let mut best_score = compute_common_prefix_depth(&source_components, best_cand);

    for cand in &candidates[1..] {
        let score = compute_common_prefix_depth(&source_components, cand);
        if score > best_score {
            best_score = score;
            best_cand = cand;
        } else if score == best_score {
            let best_depth = Path::new(best_cand).components().count();
            let cand_depth = Path::new(cand).components().count();
            if cand_depth < best_depth {
                best_cand = cand;
            } else if cand_depth == best_depth && cand < best_cand {
                best_cand = cand;
            }
        }
    }

    Some(best_cand.clone())
}

fn compute_common_prefix_depth(source_components: &[std::path::Component], cand_path: &str) -> usize {
    let cand_dir = Path::new(cand_path).parent().unwrap_or_else(|| Path::new(""));
    let cand_components: Vec<_> = cand_dir.components().collect();
    let mut common = 0;
    for (sc, cc) in source_components.iter().zip(cand_components.iter()) {
        if sc == cc {
            common += 1;
        } else {
            break;
        }
    }
    common
}

/// Normalizes a relative path from the source file directory to a canonical vault POSIX path.
pub fn resolve_relative_path(source_id: &str, target_path: &str) -> String {
    let source_path = Path::new(source_id);
    let parent = source_path.parent().unwrap_or_else(|| Path::new(""));
    let target_p = Path::new(target_path);

    let combined = if parent.as_os_str().is_empty() || target_p.starts_with(parent) {
        target_p.to_path_buf()
    } else {
        parent.join(target_path)
    };

    // Normalize components (e.g. eliminating "." and "..")
    let mut normalized = Vec::new();
    for component in combined.components() {
        match component {
            std::path::Component::Normal(c) => normalized.push(c.to_string_lossy().to_string()),
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::CurDir => {}
            _ => {}
        }
    }

    normalized.join("/")
}

/// Resolves edges against registered nodes and validates section heading anchors.
/// Supports multi-repo cross-submodule traversal, case-insensitive normalization,
/// and Obsidian shortest-path vault lookups with O(1) indexed lookups.
pub fn resolve_edge_integrity(
    edge: &mut NoteEdge,
    index: &ResolverIndex,
    node_headings: &HashMap<String, Vec<String>>,
) {
    let raw_target = edge.target.trim().replace('\\', "/");
    let rel_target = resolve_relative_path(&edge.source, &raw_target);

    // Build ordered list of candidate paths to check:
    // 1. rel_target: relative to source note directory (e.g. "../02_memory/active_context.md")
    // 2. vault_rel_target: if source is in "notes/..." (or another top-level vault folder)
    //    and target does not start with that folder, try prepending "notes/"
    // 3. raw_target: relative to workspace root (e.g. "04_harness/AGENTS.md")
    let mut candidate_paths = Vec::with_capacity(3);
    candidate_paths.push(rel_target.clone());

    if let Some(first_segment) = edge.source.split('/').next() {
        if !first_segment.is_empty() && !raw_target.starts_with(first_segment) {
            candidate_paths.push(format!("{}/{}", first_segment, raw_target));
        }
    }

    if !candidate_paths.contains(&raw_target) {
        candidate_paths.push(raw_target.clone());
    }

    // Step A: Direct match in known_nodes (trying with .md appended if missing)
    let mut matched_target: Option<String> = None;

    for cand in &candidate_paths {
        if cand.ends_with(".md") {
            if index.known_nodes.contains_key(cand) {
                matched_target = Some(cand.clone());
                break;
            }
        } else {
            let cand_md = format!("{}.md", cand);
            if index.known_nodes.contains_key(&cand_md) {
                matched_target = Some(cand_md);
                break;
            }
            if index.known_nodes.contains_key(cand) {
                matched_target = Some(cand.clone());
                break;
            }
        }
    }

    // Step B: Case-folded match in case_folded
    if matched_target.is_none() {
        for cand in &candidate_paths {
            if cand.ends_with(".md") {
                if let Some(target) = index.case_folded.get(&cand.to_lowercase()) {
                    matched_target = Some(target.clone());
                    break;
                }
            } else {
                let cand_md = format!("{}.md", cand);
                if let Some(target) = index.case_folded.get(&cand_md.to_lowercase()) {
                    matched_target = Some(target.clone());
                    break;
                }
                if let Some(target) = index.case_folded.get(&cand.to_lowercase()) {
                    matched_target = Some(target.clone());
                    break;
                }
            }
        }
    }

    // Step C: Vault shortest-path basename lookup with proximity scoring
    if matched_target.is_none() {
        let target_file_name = Path::new(&raw_target)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| raw_target.clone());

        let target_stem = Path::new(&raw_target)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| target_file_name.clone());

        let target_name_md = if target_file_name.ends_with(".md") {
            target_file_name.to_lowercase()
        } else {
            format!("{}.md", target_file_name.to_lowercase())
        };

        if let Some(cands) = index
            .basename_map
            .get(&target_name_md)
            .or_else(|| index.basename_map.get(&target_stem.to_lowercase()))
        {
            matched_target = resolve_best_basename_candidate(cands, &edge.source);
        }
    }

    let canonical_target = match matched_target {
        Some(found) => found,
        None => {
            // Check if target file physically exists on disk (e.g. non-markdown workspace files like Caddyfile, configs, scripts)
            let source_dir_exists = index
                .known_nodes
                .get(&edge.source)
                .and_then(|p| p.parent())
                .map(|dir| dir.join(&raw_target).is_file())
                .unwrap_or(false);

            let disk_target_exists = source_dir_exists
                || candidate_paths.iter().any(|c| Path::new(c).is_file())
                || Path::new(&raw_target).is_file()
                || Path::new(&rel_target).is_file();

            if disk_target_exists {
                edge.target = rel_target.clone();
                edge.id = format!("{}->{}", edge.source, edge.target);
                edge.is_broken = false;
                edge.resolution_status = Some(LinkResolutionStatus::Resolved);
                return;
            }

            edge.target = rel_target.clone();
            edge.id = format!("{}->{}", edge.source, edge.target);
            edge.is_broken = true;
            edge.resolution_status = Some(if edge.link_type == LinkType::Wikilink {
                LinkResolutionStatus::BrokenWikilink
            } else {
                LinkResolutionStatus::BrokenMissingFile
            });
            return;
        }
    };

    edge.target = canonical_target.clone();
    edge.id = format!("{}->{}", edge.source, edge.target);

    // Validate section heading anchor if present
    let anchor_candidate = edge
        .anchor
        .as_ref()
        .or(edge.target_fragment.as_ref())
        .cloned();

    if let Some(ref anchor) = anchor_candidate {
        let clean_anchor = anchor.trim_start_matches('#');
        let anchor_slug = slugify_heading(clean_anchor);
        let mut anchor_found = false;
        let mut matched_slug = anchor_slug.clone();

        if let Some(headings) = node_headings.get(&canonical_target) {
            let mut disambiguator = crate::fragment::extractor::AnchorDisambiguator::new();
            for h in headings {
                let h_slug = slugify_heading(h);
                let disambiguated = disambiguator.disambiguate(&h_slug);
                if h_slug == anchor_slug
                    || disambiguated == anchor_slug
                    || h.trim().eq_ignore_ascii_case(clean_anchor)
                {
                    anchor_found = true;
                    matched_slug = disambiguated;
                    break;
                }
            }
        }

        if !anchor_found {
            edge.is_broken = true;
            edge.resolution_status = Some(LinkResolutionStatus::BrokenFragmentAnchor);
            return;
        } else {
            edge.target_fragment = Some(matched_slug);
        }
    }

    edge.is_broken = false;
    if index.excluded_nodes.contains(&canonical_target) {
        edge.resolution_status = Some(LinkResolutionStatus::ResolvedExcluded);
    } else {
        edge.resolution_status = Some(LinkResolutionStatus::Resolved);
    }
}

/// Validates fragment anchors across edges and emits deterministic BROKEN_FRAGMENT_ANCHOR findings.
pub fn validate_fragment_anchors(
    edges: &[NoteEdge],
    index: &ResolverIndex,
    node_headings: &HashMap<String, Vec<String>>,
) -> Vec<crate::model::DeterministicFinding> {
    let mut findings = Vec::new();
    let mut counter = 1;

    for edge in edges {
        let anchor_candidate = edge
            .anchor
            .as_ref()
            .or(edge.target_fragment.as_ref());

        if let Some(anchor) = anchor_candidate {
            let clean_anchor = anchor.trim_start_matches('#');

            let mut test_edge = edge.clone();
            resolve_edge_integrity(&mut test_edge, index, node_headings);

            if test_edge.resolution_status == Some(LinkResolutionStatus::BrokenFragmentAnchor) {
                findings.push(crate::model::DeterministicFinding {
                    finding_id: format!("FINDING-ANCHOR-{}", counter),
                    rule_id: "BROKEN_FRAGMENT_ANCHOR".to_string(),
                    severity: crate::model::DeterministicFindingSeverity::Error,
                    subject: edge.source.clone(),
                    evidence: crate::model::FindingEvidence {
                        changed_paths: Vec::new(),
                        dependency_sources: vec![format!("{:?}", edge.link_type)],
                        rule_description: format!(
                            "Broken fragment anchor '#{}' in target document '{}'",
                            clean_anchor, test_edge.target
                        ),
                        line_number: edge.line,
                        anchor: Some(clean_anchor.to_string()),
                        target: Some(test_edge.target.clone()),
                    },
                    required_action: format!(
                        "Update heading anchor '#{}' in '{}' or restore heading in '{}'",
                        clean_anchor, edge.source, test_edge.target
                    ),
                    epistemic_tier: Some(crate::model::EpistemicTier::TierAFullyDeterministic),
                });
                counter += 1;
            }
        }
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::LinkType;

    #[test]
    fn test_target_resolvability_invariant_with_excluded_target() {
        let mut known_nodes = HashMap::new();
        known_nodes.insert(
            "docs/index.md".to_string(),
            PathBuf::from("/vault/docs/index.md"),
        );
        known_nodes.insert(
            "packages/lib/README.md".to_string(),
            PathBuf::from("/vault/packages/lib/README.md"),
        );

        let mut excluded_nodes = HashSet::new();
        excluded_nodes.insert("packages/lib/README.md".to_string());

        let index = ResolverIndex::with_exclusions(&known_nodes, excluded_nodes);
        let headings = HashMap::new();

        let mut edge = NoteEdge::new(
            "docs/index.md",
            "../packages/lib/README.md",
            LinkType::Markdown,
            "Lib Readme",
            None,
            Some(10),
        );

        resolve_edge_integrity(&mut edge, &index, &headings);

        assert_eq!(edge.target, "packages/lib/README.md");
        assert!(!edge.is_broken, "Excluded target must not be marked broken");
        assert_eq!(
            edge.resolution_status,
            Some(LinkResolutionStatus::ResolvedExcluded)
        );
    }

    #[test]
    fn test_generated_fragment_anchor_uses_same_slug_as_link_validation() {
        let mut known_nodes = HashMap::new();
        known_nodes.insert(
            "specs/spec.md".to_string(),
            PathBuf::from("/vault/specs/spec.md"),
        );
        let index = ResolverIndex::new(&known_nodes);
        let mut headings = HashMap::new();
        headings.insert(
            "specs/spec.md".to_string(),
            vec!["3.1 Knowledge State Boundary".to_string()],
        );

        let generated_anchor =
            crate::fragment::extractor::slugify_heading("3.1 Knowledge State Boundary");
        assert_eq!(generated_anchor, "31-knowledge-state-boundary");

        let mut edge = NoteEdge::new(
            "index.md",
            "specs/spec.md",
            LinkType::Markdown,
            "Knowledge State Boundary",
            Some(generated_anchor),
            Some(1),
        );
        resolve_edge_integrity(&mut edge, &index, &headings);

        assert!(!edge.is_broken);
        assert_eq!(edge.resolution_status, Some(LinkResolutionStatus::Resolved));
    }

    #[test]
    fn test_non_markdown_file_link_resolved_if_exists_on_disk() {
        let temp_dir = tempfile::tempdir().unwrap();
        let src_file = temp_dir.path().join("README.md");
        let caddy_file = temp_dir.path().join("Caddyfile");
        std::fs::write(&src_file, "# Readme").unwrap();
        std::fs::write(&caddy_file, "# Caddyfile config").unwrap();

        let mut known_nodes = HashMap::new();
        known_nodes.insert("README.md".to_string(), src_file);
        let index = ResolverIndex::new(&known_nodes);
        let headings = HashMap::new();

        let mut edge = NoteEdge::new(
            "README.md",
            "Caddyfile",
            LinkType::Markdown,
            "Caddyfile",
            None,
            Some(10),
        );
        resolve_edge_integrity(&mut edge, &index, &headings);

        assert!(!edge.is_broken, "Caddyfile physically exists on disk and should be resolved");
        assert_eq!(edge.resolution_status, Some(LinkResolutionStatus::Resolved));
    }
}
