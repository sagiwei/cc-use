use anyhow::{bail, Result};
use colored::Colorize;

use crate::config;

pub fn run(name: &str) -> Result<()> {
    let path = config::config_path(name)?;
    if !path.exists() {
        bail!("configuration '{}' does not exist", name);
    }

    let current = config::current_config()?;
    if current.as_deref() == Some(name) {
        bail!(
            "cannot remove '{}' because it is currently active. Switch to another configuration first.",
            name
        );
    }

    std::fs::remove_file(&path)?;
    println!("Removed configuration {}", name.yellow().bold());
    Ok(())
}
