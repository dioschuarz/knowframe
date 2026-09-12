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
}

impl Default for ScanSettings {
    fn default() -> Self {
        Self {
            include: default_include(),
            exclude: default_scan_exclude(),
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
        }
    }
}

impl ContextVaultConfig {
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

    pub fn get_scan_exclude(&self) -> Vec<String> {
        if let Some(ref scan) = self.scan {
            if !scan.exclude.is_empty() {
                return scan.exclude.clone();
            }
        }
        if let Some(ref exc) = self.exclude {
            if !exc.is_empty() {
                return exc.clone();
            }
        }
        default_scan_exclude()
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
        assert_eq!(config.get_graph_exclude(), Vec::<String>::new());
        assert_eq!(config.get_focus_mode(), FocusMode::All);
        assert!(config.get_show_orphans());
        assert!(!config.get_show_excluded());
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
}
