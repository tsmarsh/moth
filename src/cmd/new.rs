use crate::cmd::start;
use crate::config::Config;
use crate::issue::Priority;
use crate::store::Store;
use anyhow::{Context, Result, anyhow};
use std::process::Command;
use std::str::FromStr;

pub fn run(title: &str, priority: Option<&str>, skip_editor: bool, start: bool) -> Result<()> {
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

    if start {
        start::run(&issue.id)?;
    }

    // If user did not explicitly skip editor AND no_edit_on_new is true, return an error.
    if !skip_editor && store.config().no_edit_on_new {
        return Err(anyhow!(
            "Editing is disabled by configuration (no_edit_on_new: true)."
        ));
    }

    // If user did not explicitly skip editor AND no_edit_on_new is false, open editor.
    if !skip_editor && !store.config().no_edit_on_new {
        let editor = &store.config().editor;
        Command::new(editor)
            .arg(&issue.path)
            .status()
            .with_context(|| format!("Failed to open editor: {}", editor))?;
    }

    Ok(())
}
