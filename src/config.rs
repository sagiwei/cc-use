use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde_json::Value;

pub fn cc_use_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".cc-use"))
}

pub fn claude_settings_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".claude").join("settings.json"))
}

pub fn config_path(name: &str) -> Result<PathBuf> {
    Ok(cc_use_dir()?.join(format!("{}.json", name)))
}

pub fn base_config_path() -> Result<PathBuf> {
    Ok(cc_use_dir()?.join("base.json"))
}

pub fn ensure_init() -> Result<()> {
    let dir = cc_use_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create directory: {}", dir.display()))?;
    }
    Ok(())
}

pub fn list_configs() -> Result<Vec<String>> {
    let dir = cc_use_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut names: Vec<String> = Vec::new();
    for entry in fs::read_dir(&dir).context("failed to read cc-use directory")? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "json") {
            if let Some(stem) = path.file_stem() {
                let name = stem.to_string_lossy().to_string();
                // Skip base.json from the list
                if name != "base" {
                    names.push(name);
                }
            }
        }
    }
    names.sort();
    Ok(names)
}

pub fn current_config() -> Result<Option<String>> {
    let settings = claude_settings_path()?;
    if !settings.exists() {
        return Ok(None);
    }

    let meta = fs::symlink_metadata(&settings).context("failed to read settings.json metadata")?;
    if !meta.file_type().is_symlink() {
        return Ok(None);
    }

    let target = fs::read_link(&settings).context("failed to read symlink target")?;
    if let Some(stem) = target.file_stem() {
        Ok(Some(stem.to_string_lossy().to_string()))
    } else {
        Ok(None)
    }
}

pub fn config_exists(name: &str) -> Result<bool> {
    Ok(config_path(name)?.exists())
}

pub fn base_config_exists() -> Result<bool> {
    Ok(base_config_path()?.exists())
}

/// Load base configuration if it exists
pub fn load_base_config() -> Result<Option<Value>> {
    let path = base_config_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let value: Value = serde_json::from_str(&content)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;
    Ok(Some(value))
}

/// Load a named configuration
pub fn load_config(name: &str) -> Result<Value> {
    let path = config_path(name)?;
    if !path.exists() {
        bail!("configuration '{}' does not exist", name);
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let value: Value = serde_json::from_str(&content)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;
    Ok(value)
}

/// Recursively merge two JSON values.
/// The `overlay` value takes precedence over `base` for conflicting keys.
pub fn merge_json(base: &Value, overlay: &Value) -> Value {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            let mut result = base_map.clone();
            for (key, value) in overlay_map {
                if let Some(base_value) = result.get(key) {
                    result.insert(key.clone(), merge_json(base_value, value));
                } else {
                    result.insert(key.clone(), value.clone());
                }
            }
            Value::Object(result)
        }
        // For non-object types, overlay takes precedence
        _ => overlay.clone(),
    }
}

/// Merge base configuration with a named configuration.
/// Returns the merged configuration, or just the named config if no base exists.
pub fn get_merged_config(name: &str) -> Result<Value> {
    let config = load_config(name)?;
    
    match load_base_config()? {
        Some(base) => Ok(merge_json(&base, &config)),
        None => Ok(config),
    }
}

pub fn switch_to(name: &str) -> Result<()> {
    let target = config_path(name)?;
    if !target.exists() {
        bail!("configuration '{}' does not exist", name);
    }

    let settings = claude_settings_path()?;

    // Remove existing file/symlink
    if settings.exists() || fs::symlink_metadata(&settings).is_ok() {
        let meta =
            fs::symlink_metadata(&settings).context("failed to read settings.json metadata")?;

        if meta.file_type().is_symlink() {
            fs::remove_file(&settings).context("failed to remove existing symlink")?;
        } else {
            let backup = settings.with_extension("json.bak");
            fs::rename(&settings, &backup).with_context(|| {
                format!(
                    "failed to backup settings.json to {}",
                    backup.display()
                )
            })?;
            eprintln!(
                "Backed up existing settings.json to {}",
                backup.display()
            );
        }
    }

    // Check if base config exists - if so, write merged config instead of symlink
    if base_config_exists()? {
        let merged = get_merged_config(name)?;
        let pretty = serde_json::to_string_pretty(&merged)
            .context("failed to serialize merged configuration")?;
        fs::write(&settings, pretty)
            .with_context(|| format!("failed to write {}", settings.display()))?;
        eprintln!("Merged base.json with {}.json", name);
    } else {
        // No base config - use symlink as before
        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &settings).with_context(|| {
            format!(
                "failed to create symlink {} -> {}",
                settings.display(),
                target.display()
            )
        })?;

        #[cfg(not(unix))]
        bail!("symlinks are only supported on Unix systems");
    }

    Ok(())
}