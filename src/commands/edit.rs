use anyhow::{bail, Result};
use colored::Colorize;

use crate::config;
use crate::editor;

pub fn run(name: &str) -> Result<()> {
    let path = config::config_path(name)?;
    if !path.exists() {
        bail!("configuration '{}' does not exist. Use 'cc-use add {}' to create it.", name, name);
    }

    editor::open_editor(&path)?;
    editor::validate_json(&path)?;

    println!("Configuration {} updated", name.green().bold());
    Ok(())
}
