# Refactor Priority → Severity Terminology

The codebase conflates two distinct concepts:
- **Priority** = order/sequence of work (what to do next)
- **Severity** = impact/importance level (crit/high/med/low)

## Tasks

1. Rename `Priority` enum → `Severity` in `src/issue.rs`
2. Rename `Issue.priority` field → `Issue.severity`
3. Update config: `default_priority` → `default_severity`
4. Update all references in store.rs, list.rs, priority.rs
5. Add `moth severity <id> <level>` command to change severity
6. Add `-S, --severity` filter to `moth ls`
7. Update `moth new` flag: `-p, --priority` → `-s, --severity`
8. Update `.moth/config.yml`

