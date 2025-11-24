use crate::config::Config;
use crate::issue::Priority;
use crate::store::Store;
use anyhow::Result;
use std::process::Command;
use std::str::FromStr;

pub fn run(title: &str, priority: Option<&str>, skip_editor: bool) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let priority_str = priority.unwrap_or(&store.config().default_priority);
    let priority = Priority::from_str(priority_str)?;

    let issue = store.create_issue(title, priority)?;

    println!(
        "Created {}: {} [{}]",
        issue.id,
        issue.title(),
        issue.priority
    );

    if !skip_editor {
        let editor = &store.config().editor;
        Command::new(editor).arg(&issue.path).status().ok();
    }

    Ok(())
}
