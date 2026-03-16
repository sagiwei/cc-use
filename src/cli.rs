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
    /// Manage base configuration (shared across all configs)
    Base {
        #[command(subcommand)]
        action: Option<BaseAction>,
    },
    /// List all configurations
    Ls,
    /// Remove a configuration
    Rm {
        /// Name of the configuration to remove
        name: String,
    },
    /// Show configuration content (merged with base if exists)
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

#[derive(Subcommand)]
pub enum BaseAction {
    /// Show base configuration
    Show,
    /// Remove base configuration
    Rm,
}