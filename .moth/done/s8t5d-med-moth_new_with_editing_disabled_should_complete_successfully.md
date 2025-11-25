# Feature Specification: Moth New With Editing Disabled Should Complete Successfully

## Goal
Previous implementation mistakenly returned an error on new task creation if the editing-on-create was disabled in configuration.

## Decisions Taken

**`moth new` behavior with `no_edit_on_new`**:
*   When `no_edit_on_new` is set to `true` in the configuration, the `moth new` command creates a new task file but does *not* open an editor for immediate content entry. Crucially, it does *not* return an error in this scenario; it simply completes successfully without opening the editor.
*   When `no_edit_on_new` is set to `false` (or is not specified, defaulting to false), the `moth new` command behaves as it previously did, opening an editor for the newly created task file.
This behavior was implemented by modifying `src/cmd/new.rs` to remove the error return when `no_edit_on_new` is true and `skip_editor` is false.

## Decisions Rejected

1.  **Applying `no_edit` to `moth edit`**: Initially considered making `no_edit` a general flag for all editing operations. This was rejected to maintain the explicit editing functionality of `moth edit` and avoid user confusion. The new name `no_edit_on_new` further reinforces this decision.

2.  **Introducing a separate `moth view` command**: An alternative approach was to introduce a `moth view` command that would respect `no_edit`. This was rejected in favor of modifying the existing `moth new` command to incorporate the `no_edit_on_new` flag, simplifying the command structure and reducing the number of commands a user needs to learn. The `moth new` command is the natural place for this functionality as it dictates the initial state of a new task.

## Verification

*   All unit, integration, and end-to-end tests were run using `cargo test` and passed successfully after the changes.
*   Specifically, `test_e2e_new_respects_no_edit_on_new_config` and `test_new_respects_no_edit_on_new_config` were updated to assert successful completion rather than an error when `no_edit_on_new` is true and the editor is not explicitly skipped.
*   An unused `anyhow` import in `src/cmd/new.rs` was removed.
*   A release build was successfully performed using `cargo build --release`.
