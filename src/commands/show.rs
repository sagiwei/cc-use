use anyhow::{bail, Result};
use colored::Colorize;

use crate::config;

pub fn run(name: Option<&str>) -> Result<()> {
    let resolved_name = match name {
        Some(n) => n.to_string(),
        None => config::current_config()?
            .ok_or_else(|| anyhow::anyhow!("no active configuration. Specify a name or switch to one first."))?,
    };

    let path = config::config_path(&resolved_name)?;
    if !path.exists() {
        bail!("configuration '{}' does not exist", resolved_name);
    }

    // Show merged config if base exists
    let value = config::get_merged_config(&resolved_name)?;
    let pretty = serde_json::to_string_pretty(&value)?;

    // Indicate if this is a merged view
    if config::base_config_exists()? {
        println!("{} (merged with base):", resolved_name.green().bold());
    } else {
        println!("{}:", resolved_name.green().bold());
    }
    println!("{}", pretty);
    Ok(())
}