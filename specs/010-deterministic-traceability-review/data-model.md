---
wiki:
  content_hash: 6bfe149cb6ea45979eb289d9fc37be7c19255396edf2e292e616f8ede12b0a0c
  fragments:
    data-model-deterministic-traceability-review-knowledge-consistency:
      tokens: 2202
      inbound: 0
      outbound: 2
    1-rust-domain-models-cratescoresrc:
      tokens: 1846
      inbound: 0
      outbound: 0
    11-code-dependency-representation:
      tokens: 568
      inbound: 0
      outbound: 0
    12-review-lifecycle-obligations:
      tokens: 421
      inbound: 0
      outbound: 0
    13-tri-state-classification-deterministic-findings:
      tokens: 597
      inbound: 0
      outbound: 0
    14-architectural-rules-gating-config-context-vaultyaml:
      tokens: 245
      inbound: 0
      outbound: 0
    2-typescript-interfaces-extensionsrctypes:
      tokens: 287
      inbound: 0
      outbound: 0
---
# Data Model: Deterministic Traceability, Review & Knowledge Consistency

**Feature Branch**: `010-deterministic-traceability-review`  
**Date**: 2026-10-13  
**Status**: Draft  
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)  

---

## 1. Rust Domain Models (`crates/core/src/`)

### 1.1 Code Dependency Representation
```rust
use serde::{Deserialize, Serialize};

/// Origin of a code dependency declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencySource {
    /// Declared in Markdown YAML frontmatter `code_dependencies` array
    Frontmatter,
*path/to/code.rs* *path/to/code.rs* *path/to/code.rs* *path/to/code.rs* *path/to/code.rs*/*path/to/code.rs*/*path/to/code.rs*/*path/to/code.rs* *path/to/code.rs*S*path/to/code.rs*t*path/to/code.rs*a*path/to/code.rs*n*path/to/code.rs*d*path/to/code.rs*a*path/to/code.rs*r*path/to/code.rs*d*path/to/code.rs* *path/to/code.rs*M*path/to/code.rs*a*path/to/code.rs*r*path/to/code.rs*k*path/to/code.rs*d*path/to/code.rs*o*path/to/code.rs*w*path/to/code.rs*n*path/to/code.rs* *path/to/code.rs*l*path/to/code.rs*i*path/to/code.rs*n*path/to/code.rs*k*path/to/code.rs* *path/to/code.rs*`*path/to/code.rs*[*path/to/code.rs*L*path/to/code.rs*a*path/to/code.rs*b*path/to/code.rs*e*path/to/code.rs*l*path/to/code.rs*]*path/to/code.rs*(*path/to/code.rs*p*path/to/code.rs*a*path/to/code.rs*t*path/to/code.rs*h*path/to/code.rs*/*path/to/code.rs*t*path/to/code.rs*o*path/to/code.rs*/*path/to/code.rs*c*path/to/code.rs*o*path/to/code.rs*d*path/to/code.rs*e*path/to/code.rs*.*path/to/code.rs*r*path/to/code.rs*s*path/to/code.rs*)*path/to/code.rs*`*path/to/code.rs*
*path/to/code.rs*    InlineMarkdownLink,
    /// Backticked symbol reference `code_symbol()`
    InlineSymbolCitation,
}

/// A normalized code dependency linking a Markdown note to a codebase file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeDependency {
    /// Normalized relative POSIX path from workspace root (e.g. "crates/core/src/lib.rs")
    pub code_path: String,
    /// Source of declaration
    pub source: DependencySource,
    /// Line number where declared (Some(line) for inline citations, None for frontmatter)
    pub line_number: Option<u32>,
}
```

### 1.2 Review Lifecycle & Obligations
```rust
/// Status of a documentation review obligation following code changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    /// Documentation was modified and updated in the same change set
    Updated,
    /// Document was reviewed and explicitly verified as correct without code edits
    ReviewedUnchanged,
    /// Pending review (change detected, documentation untouched)
    Pending,
    /// Blocked by repository gate policy (unreviewed regression in blocking mode)
    Blocked,
}

/// A concrete review obligation generated for a document affected by Git changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewObligation {
    /// Relative path of the affected Markdown document
    pub doc_id: String,
    /// Current resolution status
    pub status: ReviewStatus,
    /// List of modified/deleted/renamed code files triggering this obligation
    pub triggering_code_files: Vec<String>,
    /// All declaration origins triggering this obligation
    pub dependency_sources: Vec<DependencySource>,
    /// Receipt evidence string if marked `ReviewedUnchanged` (commit trailer or receipt ID)
    pub resolution_receipt: Option<String>,
}

/// Structured review receipt stored in `.context-vault/reviews.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewReceiptStore {
    /// Map of Git commit SHA or branch ref -> list of approved document IDs
    pub reviews: std::collections::HashMap<String, Vec<String>>,
}

/// Consolidated impact and review report for a Git change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedDocumentsReport {
    pub changed_code_files: Vec<String>,
    pub obligations: Vec<ReviewObligation>,
    pub findings: Vec<DeterministicFinding>,
    pub pending_count: u32,
    pub reviewed_count: u32,
    pub blocked_count: u32,
}
```

### 1.3 Tri-State Classification & Deterministic Findings
```rust
/// Tri-state classification for change evaluation (including Epistemic Tier D).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeEvaluationCategory {
    /// Code modification detected, no declaring documentation
    ChangeDetected,
    /// Document declared dependency on modified code; review required
    ReviewRequired,
    /// Concrete architectural constraint or integrity rule broken
    RuleViolation,
    /// Epistemic Tier D evaluation when semantic prose cannot be verified deterministically
    NotVerifiable,
}

/// Severity of a deterministic diagnostic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterministicFindingSeverity {
    Info,
    Warning,
    Error,
}

/// Standardized deterministic finding entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeterministicFinding {
    /// Unique stable identifier (e.g. "FINDING-BROKEN-ANCHOR-001")
    pub finding_id: String,
    /// Machine-verifiable rule ID (e.g. "BROKEN_FRAGMENT_ANCHOR", "DOCUMENT_REVIEW_REQUIRED")
    pub rule_id: String,
    /// Finding severity level
    pub severity: DeterministicFindingSeverity,
    /// Subject document or file path
    pub subject: String,
    /// Concrete evidence supporting the finding
    pub evidence: FindingEvidence,
    /// Actionable remediation instructions
    pub required_action: String,
    /// Epistemic classification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epistemic_tier: Option<crate::epistemic::EpistemicTier>,
}

/// Traceable evidence supporting a deterministic finding.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FindingEvidence {
    /// Changed code paths triggering the evaluation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changed_paths: Vec<String>,
    /// Declaration origins (e.g. ["frontmatter"], ["inline_markdown_link"])
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency_sources: Vec<String>,
    /// Plain-text rule rationale
    pub rule_description: String,
    /// Line number in subject file (if applicable)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<u32>,
    /// Targeted anchor if anchor finding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Targeted document path if link finding
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
```

### 1.4 Architectural Rules & Gating Config (`.context-vault.yaml`)
```rust
/// Configured architectural rule from `.context-vault.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchitecturalRule {
    pub id: String,
    pub rule_type: ArchitecturalRuleType,
    #[serde(alias = "code_pattern")]
    pub source_pattern: Option<String>,
    #[serde(alias = "forbidden_target")]
    pub target_pattern: Option<String>,
    pub required_doc: Option<String>,
    pub severity: DeterministicFindingSeverity,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchitecturalRuleType {
    ProhibitedDependency,
    MandatoryDocumentation,
    PathExists,
}

/// Review and gating policy configuration in `.context-vault.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReviewGatingConfig {
    #[serde(default)]
    pub enforce_doc_review: bool,
    #[serde(default)]
    pub block_on_legacy_debt: bool,
}
```

---

## 2. TypeScript Interfaces (`extension/src/types/`)

```typescript
export type DependencySource = 'frontmatter' | 'inline_markdown_link' | 'inline_symbol_citation';

export interface CodeDependency {
  code_path: string;
  source: DependencySource;
  line_number?: number;
}

export type ReviewStatus = 'updated' | 'reviewed_unchanged' | 'pending' | 'blocked';

export interface ReviewObligation {
  doc_id: string;
  status: ReviewStatus;
  triggering_code_files: string[];
  dependency_sources: DependencySource[];
  resolution_receipt?: string;
}

export type DeterministicFindingSeverity = 'info' | 'warning' | 'error';

export interface FindingEvidence {
  changed_paths?: string[];
  dependency_sources?: string[];
  rule_description: string;
  line_number?: number;
  anchor?: string;
  target?: string;
}

export interface DeterministicFinding {
  finding_id: string;
  rule_id: string;
  severity: DeterministicFindingSeverity;
  subject: string;
  evidence: FindingEvidence;
  required_action: string;
  epistemic_tier?: string;
}

export interface AffectedDocumentsReport {
  changed_code_files: string[];
  obligations: ReviewObligation[];
  findings: DeterministicFinding[];
  pending_count: number;
  reviewed_count: number;
  blocked_count: number;
}
```
