use anyhow::Result;
use colored::Colorize;

use crate::config;

pub fn run() -> Result<()> {
    let configs = config::list_configs()?;
    if configs.is_empty() {
        println!("No configurations found. Use 'cc-use add <name>' to create one.");
        return Ok(());
    }

    let current = config::current_config()?;

    for name in &configs {
        if current.as_deref() == Some(name.as_str()) {
            println!("  {} {}", name.green().bold(), "(active)".green());
        } else {
            println!("  {}", name);
        }
    }

    Ok(())
}
