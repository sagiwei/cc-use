use anyhow::{bail, Result};
use colored::Colorize;

use crate::config;
use crate::editor;

pub fn edit() -> Result<()> {
    config::ensure_init()?;

    let path = config::base_config_path()?;

    // Create empty file if doesn't exist
    if !path.exists() {
        std::fs::write(&path, "{}\n")?;
    }

    editor::open_editor(&path)?;
    editor::validate_json(&path)?;

    if config::base_config_exists()? {
        println!("Base configuration updated");
    } else {
        // User deleted content, remove the file
        std::fs::remove_file(&path).ok();
        println!("Base configuration removed");
    }
    Ok(())
}

pub fn show() -> Result<()> {
    if !config::base_config_exists()? {
        bail!("no base configuration. Use 'cc-use base' to create one.");
    }

    let path = config::base_config_path()?;
    let content = std::fs::read_to_string(&path)?;
    let value: serde_json::Value = serde_json::from_str(&content)?;
    let pretty = serde_json::to_string_pretty(&value)?;

    println!("{}:", "base".green().bold());
    println!("{}", pretty);
    Ok(())
}

pub fn remove() -> Result<()> {
    if !config::base_config_exists()? {
        bail!("no base configuration to remove");
    }

    let path = config::base_config_path()?;
    std::fs::remove_file(&path)?;

    println!("Base configuration removed");
    Ok(())
}
