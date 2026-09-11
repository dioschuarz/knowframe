//! Auto-generator for .context-vault.yaml default configuration.

use super::ContextVaultConfig;
use std::path::Path;

pub fn generate_default_config(discovered_hub: Option<&str>) -> ContextVaultConfig {
    let mut config = ContextVaultConfig::default();
    if let Some(hub) = discovered_hub {
        config.vault.auto_discovered_hub = Some(hub.to_string());
        config.vault.hubs.push(hub.to_string());
    }
    config
}

pub fn write_config_if_missing(
    workspace_root: &Path,
    discovered_hub: Option<&str>,
) -> Result<bool, std::io::Error> {
    let config_path = workspace_root.join(".context-vault.yaml");
    if config_path.exists() {
        return Ok(false);
    }
    let config = generate_default_config(discovered_hub);
    let yaml = serde_yaml::to_string(&config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    std::fs::write(&config_path, yaml)?;
    Ok(true)
}
