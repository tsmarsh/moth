# Feature Specification: Project-Level `no_edit` Configuration

## Issue: `5bsq3-med-make_no_edit_a_project_level_config_yaml.md`

### Description
This feature introduces a new configuration option, `no_edit`, at the project level within the `.moth/config.yml` file. When set to `true`, this option will prevent any modifications to issues. This is intended to provide a safeguard against accidental edits, particularly for issues that are considered finalized or should not be altered without explicit intent.

### Decisions Taken

1.  **Configuration Location:** The `no_edit` flag will be added to the top-level `Config` struct in `src/config.rs` and consequently to the `.moth/config.yml` file. This ensures it's a project-wide setting.
2.  **Data Type:** A boolean type (`bool`) was chosen for `no_edit` to represent its on/off nature clearly.
3.  **Default Value:** The default value for `no_edit` will be `false`. This means that by default, issues can be edited, maintaining the current behavior unless explicitly configured otherwise.
4.  **Serialization/Deserialization:** The `#[serde(default)]` attribute is used for the `no_edit` field in `src/config.rs` to ensure that if the `no_edit` field is missing from `config.yml`, it defaults to `false` without causing a deserialization error.

### Decisions Dismissed

1.  **Per-Status `no_edit`:** Initially considered adding `no_edit` to `StatusConfig`. This was dismissed because the request specifically mentioned a "project-level" config. A per-status `no_edit` would add unnecessary complexity for the current requirement and could be considered in a future enhancement if a use case arises (e.g., making "done" issues uneditable).
2.  **Command-line Override:** While a command-line flag to override `no_edit` could be useful, it was decided to keep the initial implementation focused on the configuration file setting as per the issue title. Command-line overrides can be added later if needed.

### Implementation Details

-   **File:** `src/config.rs`
    -   Added `pub no_edit: bool,` to the `Config` struct.
    -   Added `no_edit: false,` to the `impl Default for Config` block.
-   **File:** `.moth/config.yml`
    -   Added `no_edit: false` under the `id_length` field.

### Future Considerations

-   Implementing the actual logic in the application to respect the `no_edit` flag. This will involve checking `config.no_edit` before allowing any edit operations.
-   Potentially adding a command-line flag to temporarily override the `no_edit` setting.
-   Exploring per-status `no_edit` if a strong use case emerges.
