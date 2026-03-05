use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};

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
                names.push(stem.to_string_lossy().to_string());
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

pub fn switch_to(name: &str) -> Result<()> {
    let target = config_path(name)?;
    if !target.exists() {
        bail!("configuration '{}' does not exist", name);
    }

    let settings = claude_settings_path()?;

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

    Ok(())
}

pub fn config_exists(name: &str) -> Result<bool> {
    Ok(config_path(name)?.exists())
}
