# Feature: Optional --start flag for `moth new`

## Specification:
When `moth new` is executed, it should support an optional `--start` flag.
- If the `--start` flag is present, the newly created issue will be immediately moved to the `doing` state.
- If the `--start` flag is not present, the issue will be created in the default `ready` state.

## Decisions:
- The flag will be named `--start` for clarity and conciseness.
- The default state for new issues without the flag will remain `ready`.

## Rejected Options:
- Using a different flag name (e.g., `--do`, `--active`) was considered but `--start` aligns better with the concept of moving an issue to the 'doing' state.
- Automatically moving all new issues to 'doing' was rejected to maintain flexibility for users who prefer to triage issues in 'ready' first.

## Implementation Plan:
1.  Modify the `moth new` command's argument parsing to recognize the `--start` flag.
2.  Adjust the issue creation logic to check for the presence of the `--start` flag.
3.  If `--start` is found, call the necessary function to transition the issue to the `doing` state.
4.  Add new unit and/or integration tests to verify the correct behavior of the `--start` flag.