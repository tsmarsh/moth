use crate::config::Config;
use crate::issue::Severity;
use crate::store::Store;
use anyhow::Result;
use std::fs;

pub fn run(id: &str, level: Severity) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let mut issue = store.find(id)?;

    let old_severity = issue.severity;
    issue.severity = level;

    // Get status config for the directory
    let status_config = store
        .config()
        .get_status(&issue.status)
        .expect("Issue has valid status");

    let status_dir = store.config().status_dir(status_config);
    let new_path = status_dir.join(issue.filename());

    fs::rename(&issue.path, &new_path)?;

    println!(
        "Changed severity of {} from {} to {}",
        issue.id, old_severity, level
    );

    Ok(())
}
