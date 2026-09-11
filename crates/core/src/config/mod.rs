//! Workspace configuration parsing (.context-vault.yaml) and color groups.

use crate::model::{ClusterQueryType, VaultMode};
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
    #[serde(default = "default_include")]
    pub include: Vec<String>,
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
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

fn default_exclude() -> Vec<String> {
    vec![
        "**/node_modules/**".to_string(),
        "**/.git/**".to_string(),
        "**/target/**".to_string(),
        "**/dist/**".to_string(),
        "**/out/**".to_string(),
    ]
}

impl Default for ContextVaultConfig {
    fn default() -> Self {
        Self {
            version: default_version(),
            vault: VaultSettings::default(),
            include: default_include(),
            exclude: default_exclude(),
            token_budget: TokenBudgetSettings::default(),
            color_groups: Vec::new(),
        }
    }
}

impl ContextVaultConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
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
    pub fn new(query: impl Into<String>, color: impl Into<String>, label: impl Into<String>) -> Self {
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
