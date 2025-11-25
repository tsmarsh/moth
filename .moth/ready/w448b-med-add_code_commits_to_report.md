# Add Code Commits to Report

## Summary

Enhance the `moth report` command to include code commits that reference moth issues in their commit messages. Currently, the report only tracks changes to `.moth/` files (created, moved, edited, deleted). This feature adds a new event type `code_commit` for commits that contain `[<issue_id>]` at the start - the exact format produced by the moth git hook.

## Background

The `moth hook install` command installs a `prepare-commit-msg` git hook that automatically prefixes commit messages with the current issue ID. The report command should detect these prefixes and include them as `code_commit` events, creating a complete audit trail that links code changes to issues.

## Design Principle: Single Source of Truth

The issue ID prefix format (`[id] message`) should be defined **once** in Rust code and used everywhere:
- The git hook should call a moth command to parse/validate prefixes
- The report command should use the same function to detect prefixes
- No duplicate regex patterns in bash and Rust

## Current Behavior

The report command outputs CSV with these columns:
```
commit_sha,commit_date,committer_name,committer_email,story_id,severity,column,event
```

Events tracked: `created`, `moved`, `edited`, `deleted`

## Proposed Behavior

Add a new `code_commit` event type that captures when a commit message starts with `[id]`.

## Implementation Details

### 1. Create Issue Prefix Parsing Function

Add to `src/issue.rs` (or new `src/issue_id.rs`):

```rust
/// Extract issue ID from a commit message prefix.
/// Returns the issue ID if the message starts with [id] format, None otherwise.
///
/// # Format
/// Messages must start with `[id]` where id is lowercase alphanumeric.
/// Example: "[abc12] Fix the login bug" -> Some("abc12")
///
/// # Example
/// ```
/// assert_eq!(extract_issue_id("[abc12] Fix bug"), Some("abc12".to_string()));
/// assert_eq!(extract_issue_id("No prefix"), None);
/// ```
pub fn extract_issue_id(message: &str) -> Option<String> {
    let first_line = message.lines().next()?;

    if !first_line.starts_with('[') {
        return None;
    }

    let end_bracket = first_line.find(']')?;
    let id = &first_line[1..end_bracket];

    // Validate: must be non-empty, lowercase alphanumeric only
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
        return None;
    }

    Some(id.to_string())
}

/// Extract issue ID and remaining message from a commit message.
/// Returns (id, message_without_prefix) if valid prefix found.
pub fn parse_issue_prefix(message: &str) -> Option<(String, String)> {
    let first_line = message.lines().next()?;
    let id = extract_issue_id(message)?;

    let prefix_end = first_line.find(']')? + 1;
    let rest = first_line[prefix_end..].trim_start().to_string();

    Some((id, rest))
}
```

### 2. Add CLI Command for Hook to Use

Add a new subcommand `moth prefix` that the git hook can call:

```rust
// src/cmd/prefix.rs

use crate::issue::extract_issue_id;

/// Check if a message has an issue prefix, output the ID if found
/// Exit code 0 if prefix found, 1 if not
pub fn check(message: &str) -> Result<()> {
    match extract_issue_id(message) {
        Some(id) => {
            println!("{}", id);
            Ok(())
        }
        None => {
            std::process::exit(1);
        }
    }
}
```

CLI definition:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Check for issue ID prefix in a message (used by git hook)
    Prefix {
        /// The message to check
        message: String,
    },
}
```

### 3. Update Git Hook to Use moth Command

```bash
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

# Skip if already tagged - use moth to check (single source of truth)
if moth prefix "$MSG" >/dev/null 2>&1; then
    exit 0
fi

# Prepend story ID
echo "[$STORY_ID] $MSG" > "$COMMIT_MSG_FILE"
"#;
```

**Key change**: Instead of `grep -qE '^\[[a-z0-9]+\]'`, we now use `moth prefix "$MSG"` which calls the Rust function. This ensures the hook and report use identical parsing logic.

### 4. Update report.rs to Use Shared Function

```rust
// src/cmd/report.rs
use crate::issue::parse_issue_prefix;

// In the main loop:
for commit in commits {
    let current_state = extract_stories(&repo, &commit)?;
    let changes = detect_changes(&prev_state, &current_state);

    // Existing: output .moth file changes
    for (story_id, event, story) in &changes {
        // ... existing output with empty message column ...
    }

    // New: check for code commit referencing an issue
    if let Some((issue_id, message)) = parse_issue_prefix(
        commit.message().unwrap_or("")
    ) {
        // Look up issue state (try current, fall back to prev)
        if let Some(story) = current_state.get(&issue_id)
            .or_else(|| prev_state.get(&issue_id))
        {
            println!(
                "{},{},{},{},{},{},{},code_commit,{}",
                commit.id(),
                timestamp,
                escape_csv(commit.committer().name().unwrap_or("")),
                escape_csv(commit.committer().email().unwrap_or("")),
                escape_csv(&issue_id),
                escape_csv(&story.key.severity),
                escape_csv(&story.column),
                escape_csv(&message)
            );
        }
    }

    prev_state = current_state;
}
```

### 5. Update CSV Header

```rust
println!(
    "commit_sha,commit_date,committer_name,committer_email,story_id,severity,column,event,message"
);
```

### 6. Handle Edge Cases

- **Issue doesn't exist in `.moth/`**: Skip the code_commit event
- **Same commit has both code and .moth changes**: Output both event types
- **moth command not in PATH**: Hook falls back gracefully (or we handle the error)

## Files to Modify

- `src/issue.rs` - Add `extract_issue_id()` and `parse_issue_prefix()` functions
- `src/cmd/mod.rs` - Add `prefix` module
- `src/cmd/prefix.rs` - New file for prefix command
- `src/main.rs` - Add Prefix subcommand
- `src/cmd/hook.rs` - Update HOOK_SCRIPT to use `moth prefix`
- `src/cmd/report.rs` - Add code commit detection, update CSV output

## Testing

Add tests in `src/issue.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_issue_id_valid() {
        assert_eq!(extract_issue_id("[abc12] Fix bug"), Some("abc12".to_string()));
    }

    #[test]
    fn test_extract_issue_id_no_space() {
        assert_eq!(extract_issue_id("[abc12]Fix bug"), Some("abc12".to_string()));
    }

    #[test]
    fn test_extract_issue_id_no_prefix() {
        assert_eq!(extract_issue_id("No prefix here"), None);
    }

    #[test]
    fn test_extract_issue_id_uppercase_rejected() {
        assert_eq!(extract_issue_id("[ABC12] Upper"), None);
    }

    #[test]
    fn test_extract_issue_id_special_chars_rejected() {
        assert_eq!(extract_issue_id("[abc-12] Hyphen"), None);
    }

    #[test]
    fn test_parse_issue_prefix() {
        assert_eq!(
            parse_issue_prefix("[abc12] Fix bug"),
            Some(("abc12".to_string(), "Fix bug".to_string()))
        );
        assert_eq!(
            parse_issue_prefix("[abc12]"),
            Some(("abc12".to_string(), "".to_string()))
        );
    }
}
```

## Acceptance Criteria

- [ ] `extract_issue_id()` function in `src/issue.rs`
- [ ] `parse_issue_prefix()` function in `src/issue.rs`
- [ ] New `moth prefix <message>` command that uses `extract_issue_id()`
- [ ] Git hook updated to call `moth prefix` instead of using bash regex
- [ ] Report command uses `parse_issue_prefix()` to detect code commits
- [ ] Commits with `[id]` prefix generate `code_commit` events
- [ ] Code commit events include the commit message (minus prefix)
- [ ] Issue severity and column looked up from `.moth/` state
- [ ] Unknown issue IDs silently skipped
- [ ] CSV header includes new `message` column
- [ ] All unit tests pass
