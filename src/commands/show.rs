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

    let content = std::fs::read_to_string(&path)?;
    let value: serde_json::Value = serde_json::from_str(&content)?;
    let pretty = serde_json::to_string_pretty(&value)?;

    println!("{}:", resolved_name.green().bold());
    println!("{}", pretty);
    Ok(())
}
