# moth report - Git-based history extraction

Extract story change history from git commits as CSV for external analysis.

## Usage
```
moth report [--since <commit>] [--until <commit>]
```

## Output Format

CSV to stdout:
```
commit_sha,commit_date,committer_name,committer_email,story_id,priority,column,event
```

Events: `created` | `moved` | `edited` | `deleted`

## Implementation Tasks

### 1. CLI Scaffolding
- [ ] Add `Report` variant to `Commands` enum in main.rs
- [ ] Add optional `--since` and `--until` args (commit refs)
- [ ] Create `src/cmd/report.rs` 
- [ ] Register module in `src/cmd/mod.rs`

### 2. Git Integration
- [ ] Add `git2` crate to Cargo.toml
- [ ] Open repository from current directory
- [ ] Walk commits in chronological order (reverse topological)
- [ ] Apply `--since`/`--until` filtering if provided

### 3. Story State Extraction
- [ ] For each commit, read `.moth/` tree
- [ ] Parse story files: extract id, priority, column from path/filename
- [ ] Handle missing `.moth/` directory in early commits gracefully

### 4. Diff Detection
- [ ] Track previous commit's story states
- [ ] Compare current vs previous to detect:
  - New file → `created`
  - Same id, different column → `moved`  
  - Same id, same column, different content/priority → `edited`
  - Missing file that existed before → `deleted`

### 5. CSV Output
- [ ] Print header row
- [ ] For each (commit, changed story), emit one row
- [ ] Use commit's committer name/email (not author)
- [ ] Format date as ISO 8601

### 6. Edge Cases
- [ ] Initial commit creates all existing stories
- [ ] Merge commits - treat as single commit with cumulative changes
- [ ] Non-UTF8 content - skip or warn
- [ ] Stories with same ID across columns (shouldn't happen, but handle)

## Acceptance Criteria

- [ ] `moth report` outputs valid CSV
- [ ] Output is chronological (oldest commits first)
- [ ] Piping to file works: `moth report > history.csv`
- [ ] `--since` and `--until` correctly filter commit range
- [ ] Works in repos where .moth was added mid-history

## Non-Goals

- No aggregation or analysis
- No interpretation of who "owns" a story
- No cycle time calculations

