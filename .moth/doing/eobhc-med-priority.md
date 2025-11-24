Here's the full story:

```markdown
# Priority ordering for stories

Allow teams to control the order stories should be worked. Priority is per-column
and optional.

## Naming Convention Change

Old: `id-severity-slug.md`
New: `[priority-]id-severity-slug_with_underscores.md`

Examples:
- `rxj8y-med-report_command.md` (unprioritized)
- `001-rxj8y-med-report_command.md` (priority 1)
- `042-abc12-crit-fix_login_bug.md` (priority 42)

Slugs change from kebab-case to snake_case (hyphen reserved for field delimiter).

## Configuration

```yaml
statuses:
- name: ready
  dir: ready
  prioritized: true
- name: doing
  dir: doing
- name: done
  dir: done

priority:
  auto_compact: false  # default
```

## Commands

### moth priority

```
moth priority <id> top [--compact | --no-compact]
moth priority <id> bottom [--compact | --no-compact]
moth priority <id> above <other-id> [--compact | --no-compact]
moth priority <id> below <other-id> [--compact | --no-compact]
moth priority <id> <number> [--compact | --no-compact]
```

Flag overrides `priority.auto_compact` config.

### moth compact

```
moth compact [status]
```

Renumber all prioritized stories in column to 1,2,3...n preserving relative order.
If status omitted, compact all prioritized columns.

## Behavior

- Stories created in or moved to prioritized column get no priority (bottom of pile)
- Unprioritized stories sort after all prioritized stories
- When story leaves prioritized column, others optionally compact
- `moth ls` sorts by priority in prioritized columns (unprioritized at bottom)
- Priority numbers can have gaps (unless compacted)

## Implementation Tasks

### 1. Filename parsing/generation
- [ ] Update `issue.rs` to parse new format
- [ ] Handle optional priority prefix
- [ ] Convert existing slugs: kebab-case → snake_case
- [ ] Update `new` command to generate snake_case slugs

### 2. Config changes
- [ ] Add `prioritized: bool` to status config
- [ ] Add `priority.auto_compact` global config
- [ ] Default only `ready` to prioritized

### 3. moth priority command
- [ ] Create `src/cmd/priority.rs`
- [ ] Implement `top`: find lowest existing priority - 1, or 1 if none
- [ ] Implement `bottom`: remove priority prefix (or max + 1?)
- [ ] Implement `above`/`below`: insert and shift if needed
- [ ] Implement numeric: set directly
- [ ] Add `--compact`/`--no-compact` flags

### 4. moth compact command
- [ ] Create `src/cmd/compact.rs`
- [ ] Read all stories in column, sort by current priority
- [ ] Rename to 1, 2, 3...n

### 5. Update existing commands
- [ ] `moth ls`: sort by priority in prioritized columns
- [ ] `moth mv`/`moth start`/`moth done`: strip priority when leaving prioritized column
- [ ] `moth new`: respect column's prioritized setting

### 6. Update report command spec
- [ ] Add `reprioritized` event type
- [ ] Track by story ID, not filename
- [ ] Detect priority changes within same column

## Edge Cases

- [ ] Moving prioritized story to non-prioritized column strips priority
- [ ] Moving unprioritized story to prioritized column leaves unprioritized
- [ ] `priority above/below` on unprioritized target - error? or treat as bottom?
- [ ] Priority 0 - valid or reserved?
- [ ] Padding: `001` vs `1` - use padding based on column count? Or always 3 digits?

## Acceptance Criteria

- [ ] `moth priority abc12 top` moves story to top
- [ ] `moth ls` shows prioritized stories in order
- [ ] `moth compact ready` renumbers 3,7,12 → 1,2,3
- [ ] Config `auto_compact: true` compacts on every priority change
- [ ] Stories without priority appear last in listings
```