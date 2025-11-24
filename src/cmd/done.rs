use crate::config::Config;
use crate::store::Store;
use anyhow::Result;
use std::fs;

pub fn run(id: &str) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let last_status = store.config().last_status();
    let issue = store.find(id)?;
    let target_status = &last_status.name;

    store.move_issue(&issue, target_status)?;

    // Remove .moth/.current if it contains this story ID
    let current_file = store.config().moth_dir.join(".current");
    if current_file.exists()
        && let Ok(current_id) = fs::read_to_string(&current_file)
        && current_id.trim() == issue.id
    {
        let _ = fs::remove_file(&current_file);
    }

    println!("Moved {} to {}", issue.id, target_status);

    Ok(())
}
