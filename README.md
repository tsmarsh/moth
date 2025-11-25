# Moth: A Simple File-Based Issue Tracker

<p align="center">
  <img src="docs/images/moth.png" alt="Moth Logo" width="300">
</p>

![CI](https://github.com/YOUR_USERNAME/moth/workflows/CI/badge.svg)

Moth is a git-style, file-based issue tracker that stores issues as markdown files. It's designed to be simple, transparent, and git-friendly, offering an alternative to commericial issue tracking systems. 

## Philosophy on Story Points

Moth focuses on tracking the number of completed issues rather than story points. This approach is based on research indicating that story point estimates often have significant variance and may not accurately predict delivery timelines. We recommend using issue count as a more reliable metric for measuring team capacity and velocity. 

## Features

- **File-based storage**: Issues are markdown files stored in `.moth/{status}/`
- **Git-friendly**: Everything is plain text, perfect for version control
- **Simple workflow**: Move issues through customizable statuses
- **Priority support**: Track issue priority (crit, high, med, low)
- **Priority ordering**: Order stories within prioritized columns (e.g., backlog)
- **Git commit hook**: Automatically tag commits with active story ID
- **Partial ID matching**: Use short IDs to reference issues
- **Reporting**: Extract story change history from git commits as CSV
- **Configurable**: Customize statuses, priorities, and editor

## Installation

### macOS (Homebrew)

```bash
brew tap tsmarsh/moth
brew install moth
```

### From Source

```bash
cargo build --release
```

The binary will be available at `target/release/moth`.

## Quick Start

```bash
# Initialize moth in your project
moth init

# Create a new issue
moth new "Fix login bug" -p high

# List issues
moth ls

# Move issue to "doing" (sets as current story)
moth start x7k2m

# Mark issue as done (clears current story)
moth done x7k2m

# Show issue details
moth show x7k2m

# Edit issue content
moth edit x7k2m

# Delete issue
moth rm x7k2m

# Install git commit hook
moth hook install

# Set priority order (for prioritized columns)
moth priority x7k2m top
moth priority x7k2m 5
moth priority x7k2m above abc12

# Compact priority numbering
moth compact ready

# Generate CSV report of story changes
moth report --since HEAD~10
```

## Commands

### Core Commands

| Command | Description |
|---------|-------------|
| `moth init` | Create `.moth/` structure with default config |
| `moth new "<title>" [-p priority]` | Create issue in first status |
| `moth ls [-s status] [-a]` | List issues (default: all except last status) |
| `moth show <id>` | Display issue content |
| `moth start <id>` | Move issue to `statuses[1]` and set as current |
| `moth done <id>` | Move issue to `statuses[-1]` and clear current |
| `moth mv <id> <status>` | Move issue to any status |
| `moth edit <id>` | Open issue in editor |
| `moth rm <id>` | Delete an issue |

### Priority Ordering

| Command | Description |
|---------|-------------|
| `moth priority <id> top` | Move story to top of prioritized column |
| `moth priority <id> bottom` | Remove priority ordering (moves to bottom) |
| `moth priority <id> <number>` | Set specific priority number |
| `moth priority <id> above <other-id>` | Place story above another |
| `moth priority <id> below <other-id>` | Place story below another |
| `moth compact [status]` | Renumber priorities sequentially (1,2,3...) |

### Git Integration

| Command | Description |
|---------|-------------|
| `moth hook install [--force] [--append]` | Install prepare-commit-msg hook |
| `moth hook uninstall` | Remove moth git hook |
| `moth report [--since] [--until]` | Generate CSV report of story changes |

## Configuration

The configuration file is located at `.moth/config.yml`:

```yaml
# Workflow statuses (first = default for new issues, last = "done" equivalent)
statuses:
  - name: ready
    dir: ready
    prioritized: true  # Enable priority ordering for this column
  - name: doing
    dir: doing
  - name: done
    dir: done

# Default priority for new issues
default_priority: med

# Editor for `moth edit` (falls back to $EDITOR, then vi)
editor: nvim

# ID generation length (3-10)
id_length: 5

# Priority ordering settings
priority:
  auto_compact: false  # Auto-renumber on every priority change
```

### Config Behavior

- `statuses[0]`: Where `moth new` creates issues
- `statuses[1]`: Target for `moth start`
- `statuses[-1]`: Target for `moth done`
- At least 2 statuses required
- `default_priority` must be one of: `crit`, `high`, `med`, `low`

## File Structure

```
.moth/
├── config.yml
├── .current              # Tracks active story ID for git hooks
├── ready/                # Prioritized column
│   ├── 001-x7k2m-high-fix_login_bug.md
│   ├── 002-p3j9n-med-add_dark_mode.md
│   └── q8w3r-low-update_readme.md
├── doing/
│   └── a9f4k-crit-security_patch.md
└── done/
    └── b2h8l-low-update_docs.md
```

## Filename Convention

- **Format**: `[order-]id-priority-slug.md`
- **Order** (optional): 3-digit priority number for ordering within prioritized columns
- **ID**: Random lowercase alphanumeric (default 5 chars), e.g., `a3f8k`
- **Priority**: One of `crit`, `high`, `med`, `low`
- **Slug**: Snake_case derived from title
- **Examples**:
  - Unprioritized: `x7k2m-high-fix_login_bug.md`
  - Prioritized: `001-x7k2m-high-fix_login_bug.md`

## Priority Ordering

Priority ordering allows you to control the order stories should be worked within specific columns (like a backlog). Enable it per-column in your config:

```yaml
statuses:
  - name: ready
    dir: ready
    prioritized: true  # Enable ordering for this column
```

### How It Works

- Stories in prioritized columns can have an order number (001, 002, etc.)
- Stories with order numbers appear first, sorted numerically
- Stories without order numbers appear last, sorted by priority/slug
- Moving stories out of prioritized columns automatically strips the order number

### Examples

```bash
# Move story to top priority
moth priority abc12 top

# Set specific position
moth priority abc12 5

# Position relative to another story
moth priority abc12 above xyz89
moth priority abc12 below xyz89

# Remove priority (moves to bottom)
moth priority abc12 bottom

# Clean up gaps: 1,5,12 → 1,2,3
moth compact ready
```

## Git Commit Hook Integration

Moth can automatically tag your commits with the active story ID, making it easy to track which commits belong to which story.

### Setup

```bash
# Install the git hook
moth hook install

# Start a story (sets it as current)
moth start abc12

# Make commits - they'll be automatically tagged
git commit -m "Fix the bug"
# Becomes: "[abc12] Fix the bug"

# Complete the story (clears current)
moth done abc12
```

### How It Works

1. `moth start <id>` writes the story ID to `.moth/.current`
2. The `prepare-commit-msg` hook reads `.current` and prepends `[id]` to commit messages
3. `moth done <id>` removes `.current`
4. Commits made without an active story are unmodified

### Hook Behavior

- Skips merge and squash commits
- Won't double-tag messages that already have a `[tag]` prefix
- Works from any subdirectory in your repo
- Use `--append` flag to add to existing hooks

## Reporting

Extract story change history from git commits as CSV:

```bash
# Generate report for recent commits
moth report --since HEAD~50

# Report for specific time range
moth report --since "2024-01-01" --until "2024-12-31"

# Full history
moth report
```

The report includes: timestamp, story ID, event type (created/moved/edited/deleted), and details.

## Testing

Run the full test suite:

```bash
# Run all tests (integration + e2e)
cargo test

# Run only integration tests
cargo test --test integration_test

# Run only e2e shell tests
cargo test --test e2e_shell_test
```

The project includes:
- **19 integration tests**: Direct function testing
- **17 e2e shell tests**: Full CLI workflow testing via shell commands

## License

This project is open source.
