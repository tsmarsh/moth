# Feature: `moth show` with no arguments

## Specification

- When `moth show` is called without any arguments, it should display the currently active issue.
- The active issue is the one that is currently in the 'doing' state.
- If there are multiple issues in the 'doing' state, it should display the one that was most recently started.
- If there are no issues in the 'doing' state, it should display a message indicating that there is no active issue.

## Decisions

### Decision: How to identify the current issue
- The current issue will be identified by looking for the most recently modified file in the `.moth/doing` directory.
- This approach is simple and avoids the need for additional state management.

### Rejected Idea: Storing the current issue in a separate file
- Storing the current issue in a file like `.moth/.current` was considered.
- This was rejected because it would introduce another file to manage and could get out of sync with the actual state of the issues. The file system's modification times are a more reliable source of truth.
