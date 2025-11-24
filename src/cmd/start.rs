use crate::config::Config;
use crate::store::Store;
use anyhow::{Result, anyhow};
use std::fs;

pub fn run(id: &str) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let second_status = store
        .config()
        .second_status()
        .ok_or_else(|| anyhow!("Cannot use 'start' with less than 2 statuses configured"))?;

    let issue = store.find(id)?;
    let target_status = &second_status.name;

    store.move_issue(&issue, target_status)?;

    // Write the current story ID to .moth/.current
    let current_file = store.config().moth_dir.join(".current");
    fs::write(&current_file, &issue.id)?;

    println!("Moved {} to {}", issue.id, target_status);

    Ok(())
}
