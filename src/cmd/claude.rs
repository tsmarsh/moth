use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

const CLAUDE_MD_CONTENT: &str = include_str!("../../CLAUDE.md");

pub fn run(force: bool) -> Result<()> {
    let target_path = Path::new("CLAUDE.md");

    if target_path.exists() && !force {
        return Err(anyhow!(
            "CLAUDE.md already exists. Use --force to overwrite or --append to add to it."
        ));
    }

    fs::write(target_path, CLAUDE_MD_CONTENT)?;
    println!("Created CLAUDE.md with moth agent guide");

    Ok(())
}

pub fn append() -> Result<()> {
    let target_path = Path::new("CLAUDE.md");

    if target_path.exists() {
        let existing = fs::read_to_string(target_path)?;

        // Check if moth guide is already present
        if existing.contains("# Moth Agent Guide") {
            println!("CLAUDE.md already contains moth agent guide");
            return Ok(());
        }

        let combined = format!("{}\n\n---\n\n{}", existing, CLAUDE_MD_CONTENT);
        fs::write(target_path, combined)?;
        println!("Appended moth agent guide to CLAUDE.md");
    } else {
        fs::write(target_path, CLAUDE_MD_CONTENT)?;
        println!("Created CLAUDE.md with moth agent guide");
    }

    Ok(())
}
