# Git commit hook for story ID injection

Automatically prepend the current story ID to commit messages.

## End User Experience
```bash
moth start abc12
# hack hack hack
git commit -m "Fix the login bug"
# Commit message becomes: "[abc12] Fix the login bug"
```

## How It Works

1. `moth start <id>` writes current story ID to `.moth/.current`
2. `prepare-commit-msg` hook reads `.moth/.current`
3. Hook prepends `[id]` to commit message (if not already present)
4. `moth done <id>` clears `.moth/.current`

## Commands

### moth hook install
```bash
moth hook install
```

Installs the `prepare-commit-msg` hook to `.git/hooks/`.

Options:
- `--force` - overwrite existing hook
- `--append` - append to existing hook instead of replacing

### moth hook uninstall
```bash
moth hook uninstall
```

Removes the moth hook from `.git/hooks/prepare-commit-msg`.

## Hook Script

`.git/hooks/prepare-commit-msg`:
```bash
#!/bin/bash

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

# Skip if already tagged with this or any story ID
if echo "$MSG" | grep -qE '^\[[a-z0-9]+\]'; then
    exit 0
fi

# Prepend story ID
echo "[$STORY_ID] $MSG" > "$COMMIT_MSG_FILE"
```

## .moth/.current File

Simple text file containing current story ID:
```
abc12
```

Written by `moth start`, cleared by `moth done`.

Multiple stories? Last `moth start` wins. Or error? TBD.

## Implementation Tasks

### 1. Track current story
- [ ] `moth start` writes ID to `.moth/.current`
- [ ] `moth done` removes `.moth/.current`
- [ ] `moth mv <id> <status>` updates/clears `.current` appropriately
- [ ] Add `.current` to default `.gitignore` recommendation? Or track it?

### 2. moth hook install command
- [ ] Create `src/cmd/hook.rs`
- [ ] Check if `.git/hooks/` exists
- [ ] Check for existing `prepare-commit-msg`
- [ ] Write hook script with proper permissions (755)
- [ ] Handle `--force` and `--append` flags

### 3. moth hook uninstall command
- [ ] Detect if hook is ours (check for moth marker comment)
- [ ] Remove or restore original hook
- [ ] Warn if hook was modified

### 4. moth current command (optional)
```bash
moth current        # Print current story ID
moth current clear  # Clear without completing story
```

### 5. Hook script
- [ ] Embed script in binary or generate dynamically
- [ ] Handle edge cases (merges, amends, rebases)
- [ ] Walk up directory tree to find `.moth/`
- [ ] Don't duplicate tag if already present

### 6. Update start/done commands
- [ ] `start`: write `.moth/.current`
- [ ] `done`: delete `.moth/.current`
- [ ] What if `done` called on different story than `.current`?

## Configuration
```yaml
hook:
  format: "[{id}] {message}"  # default
  # Alternatives:
  # format: "{id}: {message}"
  # format: "({id}) {message}"
```

## Edge Cases

- [ ] Multiple stories in progress: last `start` wins? or error?
- [ ] `moth done xyz` when `.current` is `abc`: clear anyway? warn?
- [ ] Commit from subdirectory: hook must find `.moth/` in parent
- [ ] Amend commits: don't double-tag
- [ ] Merge commits: skip tagging
- [ ] Rebase: skip tagging (COMMIT_SOURCE check)
- [ ] Interactive rebase rewording: preserve existing tags
- [ ] Existing hook: append mode vs replace
- [ ] User edits message in editor: don't clobber their `[xyz]` prefix
- [ ] Empty `.current` file: treat as no current story

## Testing

- [ ] Fresh repo: install hook, start story, commit, verify tag
- [ ] Existing hook: --append works correctly
- [ ] Nested directory: commit from subdir still works
- [ ] Merge commit: no tag added
- [ ] Already tagged message: no double tag
- [ ] No current story: commit unchanged

## Acceptance Criteria

- [ ] `moth hook install` creates working hook
- [ ] `moth start abc12` + `git commit -m "test"` → `[abc12] test`
- [ ] `moth done abc12` clears current story
- [ ] Commits without active story are unmodified
- [ ] Hook survives in subdirectories
- [ ] `moth hook uninstall` cleanly removes hook