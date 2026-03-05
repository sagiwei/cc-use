use anyhow::{bail, Result};
use colored::Colorize;

use crate::config;
use crate::editor;

pub fn run(name: &str) -> Result<()> {
    config::ensure_init()?;

    if config::config_exists(name)? {
        bail!("configuration '{}' already exists. Use 'cc-use edit {}' to modify it.", name, name);
    }

    let path = config::config_path(name)?;
    std::fs::write(&path, "{}\n")?;

    editor::open_editor(&path)?;

    match editor::validate_json(&path) {
        Ok(()) => {
            println!("Configuration {} created successfully", name.green().bold());
            Ok(())
        }
        Err(e) => {
            std::fs::remove_file(&path).ok();
            Err(e)
        }
    }
}
