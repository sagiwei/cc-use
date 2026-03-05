use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "cc-use",
    about = "Switch Claude Code settings.json configurations",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Config name to switch to (shorthand for direct switching)
    pub name: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new configuration (opens editor)
    Add {
        /// Name for the new configuration
        name: String,
    },
    /// List all configurations
    Ls,
    /// Remove a configuration
    Rm {
        /// Name of the configuration to remove
        name: String,
    },
    /// Show configuration content
    Show {
        /// Name of the configuration (defaults to current)
        name: Option<String>,
    },
    /// Edit an existing configuration (opens editor)
    Edit {
        /// Name of the configuration to edit
        name: String,
    },
}
