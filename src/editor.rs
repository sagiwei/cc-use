use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};

fn detect_editor() -> String {
    std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| "vi".to_string())
}

pub fn open_editor(path: &Path) -> Result<()> {
    let editor = detect_editor();

    let status = Command::new(&editor)
        .arg(path)
        .status()
        .with_context(|| format!("failed to launch editor '{}'", editor))?;

    if !status.success() {
        bail!("editor exited with non-zero status");
    }

    Ok(())
}

pub fn validate_json(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    if content.trim().is_empty() {
        bail!("file is empty, configuration was not saved");
    }

    serde_json::from_str::<serde_json::Value>(&content)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;

    Ok(())
}
