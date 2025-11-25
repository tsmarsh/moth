use crate::config::Config;
use crate::store::Store;
use anyhow::{Context, Result, anyhow};
use std::fs;

pub fn run(id: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let issue = match id {
        Some(id) => store.find(id)?,
        None => store
            .current()
            .context("Failed to get current issue")?
            .ok_or_else(|| anyhow!("No current issue"))?,
    };

    println!(
        "ID: {} | Priority: {} | Status: {}",
        issue.id, issue.priority, issue.status
    );
    println!("Title: {}", issue.title());
    println!("---");

    let content = fs::read_to_string(&issue.path)
        .with_context(|| format!("Failed to read issue file: {}", issue.path.display()))?;

    println!("{}", content);

    Ok(())
}
