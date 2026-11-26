//! Workspace configuration parsing (.context-vault.yaml) and color groups.

use crate::model::{ClusterQueryType, FocusMode, VaultMode};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod generator;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextVaultConfig {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub vault: VaultSettings,
    /// V1 backward-compatibility include array
    #[serde(default)]
    pub include: Option<Vec<String>>,
    /// V1 backward-compatibility exclude array
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
    /// V2 Two-tier scan settings (technical ignore)
    #[serde(default)]
    pub scan: Option<ScanSettings>,
    /// V2 Two-tier graph settings (curated knowledge exclusion)
    #[serde(default)]
    pub graph: Option<GraphSettings>,
    #[serde(default)]
    pub token_budget: TokenBudgetSettings,
    #[serde(default)]
    pub color_groups: Vec<ClusterColorGroup>,
    /// V3 Documentation Orientation (AI-Oriented, Human-Oriented, Balanced, Custom)
    #[serde(default)]
    pub orientation: Option<crate::evolution::DocumentationOrientation>,
    /// V3 Two-tier Custom profile configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_profile: Option<CustomProfileConfig>,
    /// V4 AI Harness configuration (active harness and entry points)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harness: Option<HarnessConfig>,
    /// V4 AI Harness Entry Point paths (backward compatibility)
    #[serde(default)]
    pub harness_paths: Vec<String>,
    /// Feature 010: Review gating policy (.context-vault.yaml)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_policy: Option<crate::model::ReviewGatingConfig>,
    /// Feature 010: Architectural rules configuration
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<ArchitectureConfig>,
    /// Link clickability and portability policy (.context-vault.yaml)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkSettings>,
}

/// Link clickability & portability settings in `.context-vault.yaml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LinkSettings {
    #[serde(default)]
    pub portability: crate::model::LinkPortabilityMode,
}

/// Architectural rules configuration schema in `.context-vault.yaml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ArchitectureConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ArchitecturalRule>,
}

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
    pub severity: crate::model::DeterministicFindingSeverity,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchitecturalRuleType {
    ProhibitedDependency,
    MandatoryDocumentation,
    PathExists,
}

/// AI Harness configuration schema for multi-agent repositories (FR-001, FR-003)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HarnessConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_points: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomProfileConfig {
    #[serde(default)]
    pub tier1: Option<crate::evolution::Tier1Objectives>,
    #[serde(default)]
    pub tier2: Option<crate::evolution::Tier2Parameters>,
    #[serde(default)]
    pub constraints: Vec<crate::evolution::ConstraintRule>,
}

fn default_version() -> String {
    "1.0".to_string()
}

fn default_include() -> Vec<String> {
    vec!["**/*.md".to_string()]
}

fn default_scan_exclude() -> Vec<String> {
    vec![
        "**/node_modules/**".to_string(),
        "**/.git/**".to_string(),
        "**/target/**".to_string(),
        "**/dist/**".to_string(),
        "**/out/**".to_string(),
        "**/build/**".to_string(),
        "**/.venv/**".to_string(),
        "**/venv/**".to_string(),
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanSettings {
    #[serde(default = "default_include")]
    pub include: Vec<String>,
    #[serde(default = "default_scan_exclude")]
    pub exclude: Vec<String>,
    /// Follow symlinked directories and files that resolve inside the workspace root.
    /// Default `true`: catalogs of symlinks (e.g. a skills catalog pointing at the one
    /// canonical copy in a nested repository) are scanned like real folders.
    #[serde(default = "default_follow_symlinks")]
    pub follow_symlinks: bool,
    /// Skip linked git worktrees (directories whose `.git` file points into
    /// `<repo>/.git/worktrees/`). Default `true`: they are duplicate checkouts.
    #[serde(default = "default_skip_linked_worktrees")]
    pub skip_linked_worktrees: bool,
}

fn default_follow_symlinks() -> bool {
    true
}

fn default_skip_linked_worktrees() -> bool {
    true
}

impl Default for ScanSettings {
    fn default() -> Self {
        Self {
            include: default_include(),
            exclude: default_scan_exclude(),
            follow_symlinks: default_follow_symlinks(),
            skip_linked_worktrees: default_skip_linked_worktrees(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphSettings {
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub focus_mode: Option<FocusMode>,
    #[serde(default = "default_true")]
    pub show_orphans: bool,
    #[serde(default)]
    pub show_excluded: bool,
}

impl Default for GraphSettings {
    fn default() -> Self {
        Self {
            exclude: Vec::new(),
            focus_mode: Some(FocusMode::All),
            show_orphans: true,
            show_excluded: false,
        }
    }
}

impl Default for ContextVaultConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            vault: VaultSettings::default(),
            include: Some(default_include()),
            exclude: Some(default_scan_exclude()),
            scan: Some(ScanSettings::default()),
            graph: Some(GraphSettings::default()),
            token_budget: TokenBudgetSettings::default(),
            color_groups: Vec::new(),
            orientation: None,
            custom_profile: None,
            harness: None,
            harness_paths: Vec::new(),
            review_policy: None,
            architecture: None,
            links: None,
        }
    }
}

impl ContextVaultConfig {
    pub fn get_link_portability_mode(&self) -> crate::model::LinkPortabilityMode {
        self.links
            .as_ref()
            .map(|l| l.portability)
            .unwrap_or_default()
    }
    pub fn get_scan_include(&self) -> Vec<String> {
        if let Some(ref scan) = self.scan {
            if !scan.include.is_empty() {
                return scan.include.clone();
            }
        }
        if let Some(ref inc) = self.include {
            if !inc.is_empty() {
                return inc.clone();
            }
        }
        default_include()
    }

    /// Effective scan exclusions: the union of `scan.exclude` and the legacy top-level
    /// `exclude` (in that order, deduplicated). A pattern written in either list is honored;
    /// previously a non-empty `scan.exclude` silently discarded the top-level list.
    pub fn get_scan_exclude(&self) -> Vec<String> {
        let mut merged: Vec<String> = Vec::new();
        let sources = [
            self.scan.as_ref().map(|s| &s.exclude),
            self.exclude.as_ref(),
        ];
        for list in sources.into_iter().flatten() {
            for pat in list {
                if !merged.contains(pat) {
                    merged.push(pat.clone());
                }
            }
        }
        if merged.is_empty() {
            default_scan_exclude()
        } else {
            merged
        }
    }

    /// Whether the scanner skips linked git worktrees (default `true`).
    pub fn get_skip_linked_worktrees(&self) -> bool {
        self.scan
            .as_ref()
            .map(|s| s.skip_linked_worktrees)
            .unwrap_or_else(default_skip_linked_worktrees)
    }

    /// Whether the scanner follows symlinks that resolve inside the workspace root (default `true`).
    pub fn get_follow_symlinks(&self) -> bool {
        self.scan
            .as_ref()
            .map(|s| s.follow_symlinks)
            .unwrap_or_else(default_follow_symlinks)
    }

    pub fn get_graph_exclude(&self) -> Vec<String> {
        if let Some(ref graph) = self.graph {
            return graph.exclude.clone();
        }
        Vec::new()
    }

    pub fn get_focus_mode(&self) -> FocusMode {
        self.graph
            .as_ref()
            .and_then(|g| g.focus_mode)
            .unwrap_or(FocusMode::All)
    }

    pub fn get_show_orphans(&self) -> bool {
        self.graph.as_ref().map(|g| g.show_orphans).unwrap_or(true)
    }

    pub fn get_show_excluded(&self) -> bool {
        self.graph
            .as_ref()
            .map(|g| g.show_excluded)
            .unwrap_or(false)
    }

    pub fn get_orientation(&self) -> crate::evolution::DocumentationOrientation {
        self.orientation.unwrap_or_default()
    }

    pub fn find_config_path(workspace_root: &Path) -> Option<std::path::PathBuf> {
        let yml = workspace_root.join(".context-vault.yml");
        if yml.exists() {
            return Some(yml);
        }
        let yaml = workspace_root.join(".context-vault.yaml");
        if yaml.exists() {
            return Some(yaml);
        }
        None
    }

    pub fn canonical_config_path(workspace_root: &Path) -> std::path::PathBuf {
        Self::find_config_path(workspace_root)
            .unwrap_or_else(|| workspace_root.join(".context-vault.yaml"))
    }
}

pub fn find_config_path(workspace_root: &Path) -> Option<std::path::PathBuf> {
    ContextVaultConfig::find_config_path(workspace_root)
}

pub fn canonical_config_path(workspace_root: &Path) -> std::path::PathBuf {
    ContextVaultConfig::canonical_config_path(workspace_root)
}

impl ContextVaultConfig {

    pub fn load_from_workspace(workspace_root: &Path) -> Self {
        if let Some(path) = Self::find_config_path(workspace_root) {
            Self::load_from_file(&path).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let yaml = serde_yaml::to_string(self).map_err(|e| std::io::Error::other(e.to_string()))?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    pub fn save_to_workspace(&self, workspace_root: &Path) -> Result<(), std::io::Error> {
        let path = Self::canonical_config_path(workspace_root);
        self.save_to_file(&path)
    }

    pub fn get_active_harness(&self) -> Option<String> {
        if let Some(ref h) = self.harness {
            if let Some(ref act) = h.active {
                return Some(act.clone());
            }
        }
        None
    }

    pub fn get_harness_entry_points(&self) -> Vec<String> {
        if let Some(ref h) = self.harness {
            if !h.entry_points.is_empty() {
                return h.entry_points.clone();
            }
        }
        if !self.harness_paths.is_empty() {
            return self.harness_paths.clone();
        }
        vec![
            "AGENTS.md".to_string(),
            "CLAUDE.md".to_string(),
            "GEMINI.md".to_string(),
            ".github/copilot-instructions.md".to_string(),
        ]
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for group in &self.color_groups {
            group.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultSettings {
    #[serde(default = "default_vault_name")]
    pub name: String,
    #[serde(default)]
    pub mode: VaultMode,
    #[serde(default)]
    pub auto_discovered_hub: Option<String>,
    #[serde(default)]
    pub hubs: Vec<String>,
}

fn default_vault_name() -> String {
    "Workspace Context".to_string()
}

impl Default for VaultSettings {
    fn default() -> Self {
        Self {
            name: default_vault_name(),
            mode: VaultMode::SingleRepo,
            auto_discovered_hub: None,
            hubs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenBudgetSettings {
    #[serde(default = "default_warning_threshold")]
    pub warning_threshold: u64,
    #[serde(default = "default_critical_threshold")]
    pub critical_threshold: u64,
}

fn default_warning_threshold() -> u64 {
    60_000
}

fn default_critical_threshold() -> u64 {
    100_000
}

impl Default for TokenBudgetSettings {
    fn default() -> Self {
        Self {
            warning_threshold: default_warning_threshold(),
            critical_threshold: default_critical_threshold(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterColorGroup {
    pub query: String,
    pub color: String,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl ClusterColorGroup {
    pub fn new(
        query: impl Into<String>,
        color: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            query: query.into(),
            color: color.into(),
            label: label.into(),
            enabled: true,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let re = Regex::new(r"^(tag:#[a-zA-Z0-9_/-]+|path:.+)$").unwrap();
        if !re.is_match(&self.query) {
            return Err(format!(
                "Invalid query '{}': must match 'tag:#...' or 'path:...'",
                self.query
            ));
        }
        let color_re = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
        if !color_re.is_match(&self.color) {
            return Err(format!(
                "Invalid color hex '{}': must be #RRGGBB",
                self.color
            ));
        }
        Ok(())
    }

    pub fn query_type(&self) -> ClusterQueryType {
        if self.query.starts_with("tag:") {
            ClusterQueryType::Tag
        } else {
            ClusterQueryType::Path
        }
    }

    pub fn pattern(&self) -> &str {
        if let Some(pat) = self.query.strip_prefix("tag:") {
            pat
        } else if let Some(pat) = self.query.strip_prefix("path:") {
            pat
        } else {
            &self.query
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_config_fallback() {
        let yaml = r#"
version: "1.0"
include:
  - "notes/**/*.md"
exclude:
  - "**/tmp/**"
"#;
        let config: ContextVaultConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.get_scan_include(), vec!["notes/**/*.md".to_string()]);
        assert_eq!(config.get_scan_exclude(), vec!["**/tmp/**".to_string()]);
        assert!(config.get_follow_symlinks());
        assert_eq!(config.get_graph_exclude(), Vec::<String>::new());
        assert_eq!(config.get_focus_mode(), FocusMode::All);
        assert!(config.get_show_orphans());
        assert!(!config.get_show_excluded());
    }

    #[test]
    fn test_scan_exclude_merges_top_level_and_scan_lists() {
        let yaml = r#"
version: "2.0"
exclude:
  - "**/graphify-out/**"
  - "**/node_modules/**"
scan:
  exclude:
    - "**/node_modules/**"
    - "vendor/**"
  follow_symlinks: false
"#;
        let config: ContextVaultConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            config.get_scan_exclude(),
            vec![
                "**/node_modules/**".to_string(),
                "vendor/**".to_string(),
                "**/graphify-out/**".to_string(),
            ]
        );
        assert!(!config.get_follow_symlinks());
    }

    #[test]
    fn test_v2_two_tier_config() {
        let yaml = r#"
version: "2.0"
scan:
  include:
    - "vault/**/*.md"
  exclude:
    - "node_modules/**"
    - "target/**"
graph:
  exclude:
    - "packages/**/README.md"
    - "vendor/**"
  focus_mode: "1_hop"
  show_orphans: false
  show_excluded: true
"#;
        let config: ContextVaultConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.get_scan_include(), vec!["vault/**/*.md".to_string()]);
        assert_eq!(
            config.get_scan_exclude(),
            vec!["node_modules/**".to_string(), "target/**".to_string()]
        );
        assert_eq!(
            config.get_graph_exclude(),
            vec!["packages/**/README.md".to_string(), "vendor/**".to_string()]
        );
        assert_eq!(config.get_focus_mode(), FocusMode::OneHop);
        assert!(!config.get_show_orphans());
        assert!(config.get_show_excluded());
    }

    #[test]
    fn test_vault_orientation_yaml_roundtrip() {
        let yaml = r#"
version: "1.0"
orientation: human_oriented
custom_profile:
  tier1:
    ai_discoverability: medium
    context_efficiency: low
    human_readability: high
    knowledge_reuse: medium
    narrative_coherence: high
    maintenance_simplicity: high
"#;
        let config: ContextVaultConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(
            config.get_orientation(),
            crate::evolution::DocumentationOrientation::HumanOriented
        );
        let custom = config
            .custom_profile
            .as_ref()
            .expect("custom_profile exists");
        let tier1 = custom.tier1.as_ref().expect("tier1 exists");
        assert_eq!(
            tier1.human_readability,
            crate::evolution::ObjectiveLevel::High
        );

        // Serialize back
        let serialized = serde_yaml::to_string(&config).unwrap();
        let de: ContextVaultConfig = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(
            de.get_orientation(),
            crate::evolution::DocumentationOrientation::HumanOriented
        );
    }
}
