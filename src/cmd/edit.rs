use crate::config::Config;
use crate::store::Store;
use anyhow::{Context, Result};
use std::process::Command;

pub fn run(id: &str) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let issue = store.find(id)?;
    let editor = &store.config().editor;

    Command::new(editor)
        .arg(&issue.path)
        .status()
        .with_context(|| format!("Failed to open editor: {}", editor))?;

    Ok(())
}
