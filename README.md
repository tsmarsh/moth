# Moth: A Simple File-Based Issue Tracker

![CI](https://github.com/YOUR_USERNAME/moth/workflows/CI/badge.svg)

Moth is a git-style, file-based issue tracker that stores issues as markdown files. It's designed to be simple, transparent, and git-friendly, offering an alternative to commericial issue tracking systems. 

## Philosophy on Story Points

Moth focuses on tracking the number of completed issues rather than story points. This approach is based on research indicating that story point estimates often have significant variance and may not accurately predict delivery timelines. We recommend using issue count as a more reliable metric for measuring team capacity and velocity. 

## Features

- **File-based storage**: Issues are markdown files stored in `.moth/{status}/`
- **Git-friendly**: Everything is plain text, perfect for version control
- **Simple workflow**: Move issues through customizable statuses
- **Priority support**: Track issue priority (crit, high, med, low)
- **Partial ID matching**: Use short IDs to reference issues
- **Configurable**: Customize statuses, priorities, and editor

## Installation

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

# Move issue to "doing"
moth start x7k2m

# Mark issue as done
moth done x7k2m

# Show issue details
moth show x7k2m

# Edit issue content
moth edit x7k2m

# Delete issue
moth rm x7k2m
```

## Commands

| Command | Description |
|---------|-------------|
| `moth init` | Create `.moth/` structure with default config |
| `moth new "<title>" [-p priority]` | Create issue in first status |
| `moth ls [-s status] [-a]` | List issues (default: all except last status) |
| `moth show <id>` | Display issue content |
| `moth start <id>` | Move issue to `statuses[1]` |
| `moth done <id>` | Move issue to `statuses[-1]` |
| `moth mv <id> <status>` | Move issue to any status |
| `moth edit <id>` | Open issue in editor |
| `moth rm <id>` | Delete an issue |

## Configuration

The configuration file is located at `.moth/config.yml`:

```yaml
# Workflow statuses (first = default for new issues, last = "done" equivalent)
statuses:
  - name: ready
    dir: ready
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
├── ready/
│   ├── x7k2m-high-fix-login-bug.md
│   └── p3j9n-med-add-dark-mode.md
├── doing/
│   └── a9f4k-crit-security-patch.md
└── done/
    └── b2h8l-low-update-docs.md
```

## Filename Convention

- **ID**: Random lowercase alphanumeric (default 5 chars), e.g., `a3f8k`
- **Priority**: One of `crit`, `high`, `med`, `low`
- **Slug**: Kebab-case derived from title
- **Example**: `x7k2m-high-fix-login-bug.md`

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
