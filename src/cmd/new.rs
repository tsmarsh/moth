use crate::cmd::start;
use crate::config::Config;
use crate::issue::Severity;
use crate::store::Store;
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;
use std::str::FromStr;

pub fn run(
    title: &str,
    severity: Option<&str>,
    skip_editor: bool,
    start: bool,
    body: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let severity_str = severity.unwrap_or(&store.config().default_severity);
    let severity = Severity::from_str(severity_str)?;

    let issue = store.create_issue(title, severity)?;

    // Write body if provided
    if let Some(content) = body {
        fs::write(&issue.path, content)
            .with_context(|| format!("Failed to write issue body: {}", issue.path.display()))?;
    }

    println!(
        "Created {}: {} [{}]",
        issue.id,
        issue.title(),
        issue.severity
    );

    if start {
        start::run(&issue.id)?;
    }

    // If user did not explicitly skip editor AND no_edit is false, open editor.
    if !skip_editor && !store.config().no_edit {
        let editor = &store.config().editor;
        Command::new(editor)
            .arg(&issue.path)
            .status()
            .with_context(|| format!("Failed to open editor: {}", editor))?;
    }

    Ok(())
}
