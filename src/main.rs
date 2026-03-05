mod cli;
mod commands;
mod config;
mod editor;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use cli::{Cli, Command};

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Add { name }) => commands::add::run(&name),
        Some(Command::Ls) => commands::list::run(),
        Some(Command::Rm { name }) => commands::remove::run(&name),
        Some(Command::Show { name }) => commands::show::run(name.as_deref()),
        Some(Command::Edit { name }) => commands::edit::run(&name),
        None => match cli.name {
            Some(name) => commands::switch::run_direct(&name),
            None => commands::switch::run_interactive(),
        },
    }
}
