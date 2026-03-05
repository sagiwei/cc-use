use anyhow::{bail, Result};
use colored::Colorize;
use dialoguer::Select;

use crate::config;

pub fn run_interactive() -> Result<()> {
    let configs = config::list_configs()?;
    if configs.is_empty() {
        bail!("no configurations found. Use 'cc-use add <name>' to create one.");
    }

    let current = config::current_config()?;

    let items: Vec<String> = configs
        .iter()
        .map(|name| {
            if current.as_deref() == Some(name.as_str()) {
                format!("{} {}", name, "(active)".green())
            } else {
                name.clone()
            }
        })
        .collect();

    let default_idx = current
        .as_ref()
        .and_then(|c| configs.iter().position(|n| n == c))
        .unwrap_or(0);

    let selection = Select::new()
        .with_prompt("Select a configuration")
        .items(&items)
        .default(default_idx)
        .interact_opt()?;

    match selection {
        Some(idx) => {
            let name = &configs[idx];
            if current.as_deref() == Some(name.as_str()) {
                println!("{} is already active", name.green().bold());
                return Ok(());
            }
            run_direct(name)
        }
        None => Ok(()),
    }
}

pub fn run_direct(name: &str) -> Result<()> {
    config::switch_to(name)?;
    println!("Switched to {}", name.green().bold());
    Ok(())
}
