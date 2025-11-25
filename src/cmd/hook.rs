use crate::config::Config;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const HOOK_MARKER: &str = "# MOTH_HOOK_MARKER";
const HOOK_SCRIPT: &str = r#"#!/bin/bash
# MOTH_HOOK_MARKER - Do not edit this section manually

COMMIT_MSG_FILE=$1
COMMIT_SOURCE=$2

# Skip if this is a merge, squash, or amend
if [ "$COMMIT_SOURCE" = "merge" ] || [ "$COMMIT_SOURCE" = "squash" ]; then
    exit 0
fi

# Find .moth directory (walk up from current dir)
find_moth_dir() {
    local dir="$PWD"
    while [ "$dir" != "/" ]; do
        if [ -d "$dir/.moth" ]; then
            echo "$dir/.moth"
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    return 1
}

MOTH_DIR=$(find_moth_dir)
if [ -z "$MOTH_DIR" ]; then
    exit 0
fi

CURRENT_FILE="$MOTH_DIR/.current"
if [ ! -f "$CURRENT_FILE" ]; then
    exit 0
fi

STORY_ID=$(cat "$CURRENT_FILE" | tr -d '[:space:]')
if [ -z "$STORY_ID" ]; then
    exit 0
fi

# Read existing message
MSG=$(cat "$COMMIT_MSG_FILE")

# Skip if already tagged - use moth prefix for single source of truth
if moth prefix "$MSG" >/dev/null 2>&1; then
    exit 0
fi

# Prepend story ID
echo "[$STORY_ID] $MSG" > "$COMMIT_MSG_FILE"
"#;

fn find_git_dir() -> Result<PathBuf> {
    let mut current = std::env::current_dir().context("Failed to get current directory")?;

    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Ok(git_dir);
        }

        if !current.pop() {
            return Err(anyhow!(
                "No .git directory found. Are you in a git repository?"
            ));
        }
    }
}

pub fn install(force: bool, append: bool) -> Result<()> {
    let _config = Config::load()?;
    let git_dir = find_git_dir()?;
    let hooks_dir = git_dir.join("hooks");

    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir).with_context(|| {
            format!("Failed to create hooks directory: {}", hooks_dir.display())
        })?;
    }

    let hook_path = hooks_dir.join("prepare-commit-msg");

    // Check if hook already exists
    if hook_path.exists() {
        let existing_content = fs::read_to_string(&hook_path)?;

        // Check if our hook is already installed
        if existing_content.contains(HOOK_MARKER) {
            if !force {
                println!("Moth hook is already installed");
                return Ok(());
            }
        } else if !force && !append {
            return Err(anyhow!(
                "Hook file already exists at {}. Use --force to overwrite or --append to append.",
                hook_path.display()
            ));
        }

        if append {
            // Append our hook to existing content
            let combined = format!("{}\n\n{}", existing_content, HOOK_SCRIPT);
            fs::write(&hook_path, combined)?;
            println!("Appended moth hook to existing prepare-commit-msg");
        } else {
            // Force mode: replace the hook
            fs::write(&hook_path, HOOK_SCRIPT)?;
            println!("Installed moth hook (replaced existing)");
        }
    } else {
        // No existing hook, just write ours
        fs::write(&hook_path, HOOK_SCRIPT)?;
        println!("Installed moth hook");
    }

    // Make the hook executable
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)?;
    }

    println!("Hook installed at: {}", hook_path.display());

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let _config = Config::load()?;
    let git_dir = find_git_dir()?;
    let hook_path = git_dir.join("hooks").join("prepare-commit-msg");

    if !hook_path.exists() {
        println!("No prepare-commit-msg hook found");
        return Ok(());
    }

    let content = fs::read_to_string(&hook_path)?;

    if !content.contains(HOOK_MARKER) {
        return Err(anyhow!(
            "The prepare-commit-msg hook doesn't appear to be a moth hook. \
            Remove it manually if needed."
        ));
    }

    // If the hook only contains our script, delete it
    if content.trim() == HOOK_SCRIPT.trim() {
        fs::remove_file(&hook_path)?;
        println!("Removed moth hook");
    } else {
        // Hook has additional content, just remove our section
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines = Vec::new();
        let mut in_moth_section = false;

        for line in lines {
            if line.contains(HOOK_MARKER) {
                in_moth_section = !in_moth_section;
                continue;
            }
            if !in_moth_section {
                new_lines.push(line);
            }
        }

        let new_content = new_lines.join("\n");
        fs::write(&hook_path, new_content)?;
        println!("Removed moth hook section from prepare-commit-msg");
    }

    Ok(())
}
