//! Evidence-based deterministic freshness models and evaluation.
//! Invariant III: Determinism & Zero External AI.
//! FR-038: Deterministic freshness evidence signals (Signals 1 through 4).
//! FR-039: Discrete tier classification (Fresh >= 80, PotentiallyStale 60..79, Stale < 60).

use std::collections::HashSet;
use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};

use super::extractor::{extract_code_citations, CodeCitation};
use super::TechnicalDriftFinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessTier {
    /// Score >= 80
    Fresh,
    /// Score 60..79
    PotentiallyStale,
    /// Score < 60
    Stale,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreshnessEvidence {
    /// Deterministic freshness score (0.0 .. 100.0).
    pub score: f64,
    pub tier: FreshnessTier,
    /// Code symbols or file paths cited in doc that do not exist on disk (-15 pts per broken citation).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_citations: Vec<String>,
    /// Code files cited in doc that were modified in Git after the doc's last commit (-10 pts per unreviewed commit, max -30 pts).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_modified_after_doc: Vec<String>,
    /// Symbols cited in doc that were deleted or renamed in Git history (-20 pts per deleted symbol).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted_symbols: Vec<String>,
    /// Path of authoritative canonical document this document contradicts (-25 pts).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_contradiction: Option<String>,
    /// Itemized deterministic reasons explaining deductions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reasons: Vec<String>,
}

impl Default for FreshnessEvidence {
    fn default() -> Self {
        Self {
            score: 100.0,
            tier: FreshnessTier::Fresh,
            missing_citations: Vec::new(),
            code_modified_after_doc: Vec::new(),
            deleted_symbols: Vec::new(),
            canonical_contradiction: None,
            reasons: Vec::new(),
        }
    }
}

/// Signal 1: Verifies that cited code paths exist on local disk.
/// Returns list of missing code paths relative to workspace root.
pub fn verify_code_citations_disk(
    workspace_root: &Path,
    citations: &[CodeCitation],
) -> Vec<String> {
    let mut missing = Vec::new();
    let mut seen = HashSet::new();

    for cit in citations {
        if cit.code_path.is_empty() {
            continue;
        }
        if seen.insert(cit.code_path.clone()) {
            let abs_path = workspace_root.join(&cit.code_path);
            if !abs_path.exists() {
                missing.push(cit.code_path.clone());
            }
        }
    }

    missing.sort();
    missing
}

/// Signal 2: Git commit chronology analyzer.
/// Checks if cited code paths were modified in Git after the doc's last commit.
/// Returns list of modified code paths and total commit count.
pub fn check_git_commit_chronology(
    workspace_root: &Path,
    doc_path: &str,
    valid_code_paths: &[String],
) -> (Vec<String>, u32) {
    let mut modified_files = Vec::new();
    let mut total_commits = 0u32;

    if let Some(doc_commit) = get_doc_last_commit_hash(workspace_root, doc_path) {
        for code_path in valid_code_paths {
            if let Some(cnt) = count_commits_affecting_code_since(workspace_root, &doc_commit, code_path) {
                if cnt > 0 {
                    modified_files.push(code_path.clone());
                    total_commits += cnt;
                }
            }
        }
    }

    modified_files.sort();
    (modified_files, total_commits)
}

fn get_doc_last_commit_hash(workspace_root: &Path, doc_path: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["log", "-1", "--format=%H", "--", doc_path])
        .current_dir(workspace_root)
        .output()
        .ok()?;
    if output.status.success() {
        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !hash.is_empty() {
            return Some(hash);
        }
    }
    None
}

fn count_commits_affecting_code_since(
    workspace_root: &Path,
    base_commit: &str,
    code_path: &str,
) -> Option<u32> {
    let rev_range = format!("{}..HEAD", base_commit);
    let output = Command::new("git")
        .args(["rev-list", "--count", &rev_range, "--", code_path])
        .current_dir(workspace_root)
        .output()
        .ok()?;
    if output.status.success() {
        let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return count_str.parse::<u32>().ok();
    }
    None
}

/// Signal 3: Deleted/renamed symbol impact analyzer.
/// Detects symbols cited in the doc that no longer exist in cited source code or were deleted.
pub fn detect_deleted_symbols(
    workspace_root: &Path,
    citations: &[CodeCitation],
    valid_code_paths: &[String],
    drifts: &[TechnicalDriftFinding],
) -> Vec<String> {
    let mut deleted = Vec::new();
    let mut seen = HashSet::new();

    // 1. Incorporate any explicit findings from reverse impact drift engine
    for d in drifts {
        if d.drift_type == crate::drift::DriftType::DeletedSymbol {
            if let Some(ref sym) = d.symbol_name {
                if seen.insert(sym.clone()) {
                    deleted.push(sym.clone());
                }
            }
        }
    }

    // 2. Scan cited symbols against existing source files
    let mut code_contents = Vec::new();
    for code_path in valid_code_paths {
        let abs_path = workspace_root.join(code_path);
        if let Ok(content) = std::fs::read_to_string(&abs_path) {
            code_contents.push(content);
        }
    }

    for cit in citations {
        if let Some(ref sym) = cit.symbol {
            if seen.insert(sym.clone()) && !code_contents.is_empty() {
                let mut found = false;
                for content in &code_contents {
                    if content.contains(sym) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    deleted.push(sym.clone());
                }
            }
        }
    }

    deleted.sort();
    deleted
}

/// Signal 4: Canonical contradiction detector.
pub fn detect_canonical_contradiction(
    _doc_id: &str,
    canonical_contradiction: Option<String>,
) -> Option<String> {
    canonical_contradiction
}

/// Computes composite freshness evidence from individual signals.
pub fn compute_freshness_from_signals(
    missing_citations: Vec<String>,
    code_modified_after_doc: Vec<String>,
    unreviewed_commits: u32,
    deleted_symbols: Vec<String>,
    canonical_contradiction: Option<String>,
) -> FreshnessEvidence {
    let mut deductions = 0.0;
    let mut reasons = Vec::new();

    // Signal 1: Missing code citations (-15 pts each)
    if !missing_citations.is_empty() {
        let pts = 15.0 * missing_citations.len() as f64;
        deductions += pts;
        for path in &missing_citations {
            reasons.push(format!("Missing code citation on disk: '{}' (-15.0 pts)", path));
        }
    }

    // Signal 2: Code modified after doc (-10 pts per commit, capped at -30 pts max)
    if unreviewed_commits > 0 {
        let pts = (10.0 * unreviewed_commits as f64).min(30.0);
        deductions += pts;
        for path in &code_modified_after_doc {
            reasons.push(format!(
                "Cited code '{}' modified after doc commit (-{:.1} pts)",
                path,
                pts / code_modified_after_doc.len().max(1) as f64
            ));
        }
    }

    // Signal 3: Deleted symbols (-20 pts each)
    if !deleted_symbols.is_empty() {
        let pts = 20.0 * deleted_symbols.len() as f64;
        deductions += pts;
        for sym in &deleted_symbols {
            reasons.push(format!("Cited symbol '{}' deleted or renamed (-20.0 pts)", sym));
        }
    }

    // Signal 4: Canonical contradiction (-25 pts)
    if let Some(ref canon) = canonical_contradiction {
        deductions += 25.0;
        reasons.push(format!(
            "Contradicts authoritative canonical document '{}' (-25.0 pts)",
            canon
        ));
    }

    let raw_score = 100.0 - deductions;
    let score = ((raw_score.clamp(0.0, 100.0)) * 10.0).round() / 10.0;

    let tier = if score >= 80.0 {
        FreshnessTier::Fresh
    } else if score >= 60.0 {
        FreshnessTier::PotentiallyStale
    } else {
        FreshnessTier::Stale
    };

    FreshnessEvidence {
        score,
        tier,
        missing_citations,
        code_modified_after_doc,
        deleted_symbols,
        canonical_contradiction,
        reasons,
    }
}

/// Evaluates complete freshness evidence for a given document.
pub fn evaluate_document_freshness(
    workspace_root: &Path,
    doc_id: &str,
    doc_content: &str,
    canonical_contradiction: Option<String>,
    drifts: &[TechnicalDriftFinding],
) -> FreshnessEvidence {
    let citations = extract_code_citations(doc_id, doc_content);

    let missing_citations = verify_code_citations_disk(workspace_root, &citations);
    let missing_set: HashSet<_> = missing_citations.iter().cloned().collect();

    let valid_code_paths: Vec<String> = citations
        .iter()
        .map(|c| c.code_path.clone())
        .filter(|p| !p.is_empty() && !missing_set.contains(p))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let (code_modified_after_doc, unreviewed_commits) =
        check_git_commit_chronology(workspace_root, doc_id, &valid_code_paths);

    let deleted_symbols =
        detect_deleted_symbols(workspace_root, &citations, &valid_code_paths, drifts);

    compute_freshness_from_signals(
        missing_citations,
        code_modified_after_doc,
        unreviewed_commits,
        deleted_symbols,
        canonical_contradiction,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freshness_evidence_serialization() {
        let mut evidence = FreshnessEvidence::default();
        evidence.score = 75.0;
        evidence.tier = FreshnessTier::PotentiallyStale;
        evidence.missing_citations.push("src/auth/service.rs".to_string());
        evidence.reasons.push("Missing code citation: src/auth/service.rs".to_string());

        let json = serde_json::to_string(&evidence).unwrap();
        assert!(json.contains("\"tier\":\"potentially_stale\""));
        assert!(json.contains("src/auth/service.rs"));

        let deserialized: FreshnessEvidence = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, evidence);
    }

    #[test]
    fn test_compute_freshness_from_signals_tiers() {
        // 1. Fresh (100.0)
        let f1 = compute_freshness_from_signals(vec![], vec![], 0, vec![], None);
        assert_eq!(f1.score, 100.0);
        assert_eq!(f1.tier, FreshnessTier::Fresh);

        // 2. PotentiallyStale (missing citation -15 -> 85.0 -> still fresh; add 2 commits -> -20 -> 65.0 -> PotentiallyStale)
        let f2 = compute_freshness_from_signals(
            vec!["src/missing.rs".to_string()],
            vec!["src/mod.rs".to_string()],
            2,
            vec![],
            None,
        );
        assert_eq!(f2.score, 65.0);
        assert_eq!(f2.tier, FreshnessTier::PotentiallyStale);

        // 3. Stale (missing citation -15, commits -30, deleted sym -20 -> 35.0 -> Stale)
        let f3 = compute_freshness_from_signals(
            vec!["src/missing.rs".to_string()],
            vec!["src/mod.rs".to_string()],
            5, // capped at 30
            vec!["OldClass".to_string()],
            None,
        );
        assert_eq!(f3.score, 35.0);
        assert_eq!(f3.tier, FreshnessTier::Stale);
    }
}
