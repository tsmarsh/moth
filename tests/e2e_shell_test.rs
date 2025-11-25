use serial_test::serial;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn get_binary_path() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.join("moth")
}

fn setup_test_env() -> TempDir {
    TempDir::new().unwrap()
}

fn run_moth_cmd(args: &[&str], cwd: &std::path::Path) -> (bool, String, String) {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("Failed to execute moth command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

#[test]
#[serial]
fn test_e2e_init_creates_directory() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    let (success, _stdout, stderr) = run_moth_cmd(&["init"], temp_path);

    assert!(success, "Command failed: {}", stderr);
    assert!(temp_path.join(".moth").exists());
    assert!(temp_path.join(".moth/config.yml").exists());
    assert!(temp_path.join(".moth/ready").exists());
    assert!(temp_path.join(".moth/doing").exists());
    assert!(temp_path.join(".moth/done").exists());
}

#[test]
#[serial]
fn test_e2e_init_twice_fails() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    let (success, _, _) = run_moth_cmd(&["init"], temp_path);
    assert!(success);

    let (success, _, stderr) = run_moth_cmd(&["init"], temp_path);
    assert!(!success, "Should fail on second init");
    assert!(stderr.contains("already initialized"));
}

#[test]
#[serial]
fn test_e2e_new_creates_issue() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    let (success, _stdout, stderr) = run_moth_cmd(
        &["new", "Fix login bug", "--no-edit", "-s", "high"],
        temp_path,
    );

    assert!(success, "Command failed: {}", stderr);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();
    assert!(name.contains("fix_login_bug"));
}

#[test]
#[serial]
fn test_e2e_new_with_invalid_severity() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    let (success, _, stderr) = run_moth_cmd(
        &["new", "Test issue", "--no-edit", "-s", "invalid"],
        temp_path,
    );

    assert!(!success);
    assert!(stderr.contains("Invalid severity"));
}

#[test]
#[serial]
fn test_e2e_ls_shows_issues() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Issue 1", "--no-edit"], temp_path);
    run_moth_cmd(&["new", "Issue 2", "--no-edit", "-s", "high"], temp_path);

    let (success, stdout, stderr) = run_moth_cmd(&["ls"], temp_path);

    assert!(success, "Command failed: {}", stderr);
    assert!(stdout.contains("Issue 1") || stdout.contains("issue_1"));
    assert!(stdout.contains("Issue 2") || stdout.contains("issue_2"));
}

#[test]
#[serial]
fn test_e2e_ls_filters_by_status() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Ready issue", "--no-edit"], temp_path);

    let (success, stdout, _) = run_moth_cmd(&["ls", "-t", "ready"], temp_path);
    assert!(success);
    assert!(stdout.contains("Ready") || stdout.contains("ready"));

    let (success, stdout, _) = run_moth_cmd(&["ls", "-t", "doing"], temp_path);
    assert!(success);
    assert!(!stdout.contains("Ready") && !stdout.contains("ready"));
}

#[test]
#[serial]
fn test_e2e_start_moves_to_doing() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let issue_name = entries[0].file_name();
    let id = issue_name.to_string_lossy();
    let id_part: String = id.chars().take(3).collect();

    let (success, _, stderr) = run_moth_cmd(&["start", &id_part], temp_path);
    assert!(success, "Command failed: {}", stderr);

    let doing_dir = temp_path.join(".moth/doing");
    let doing_entries: Vec<_> = std::fs::read_dir(&doing_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(doing_entries.len(), 1);

    let ready_entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(ready_entries.len(), 0);
}

#[test]
#[serial]
fn test_e2e_done_moves_to_done() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let issue_name = entries[0].file_name();
    let id = issue_name.to_string_lossy();
    let id_part: String = id.chars().take(3).collect();

    let (success, _, stderr) = run_moth_cmd(&["done", &id_part], temp_path);
    assert!(success, "Command failed: {}", stderr);

    let done_dir = temp_path.join(".moth/done");
    let done_entries: Vec<_> = std::fs::read_dir(&done_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(done_entries.len(), 1);
}

#[test]
#[serial]
fn test_e2e_mv_changes_status() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let issue_name = entries[0].file_name();
    let id = issue_name.to_string_lossy();
    let id_part: String = id.chars().take(3).collect();

    let (success, _, stderr) = run_moth_cmd(&["mv", &id_part, "doing"], temp_path);
    assert!(success, "Command failed: {}", stderr);

    let doing_dir = temp_path.join(".moth/doing");
    let doing_entries: Vec<_> = std::fs::read_dir(&doing_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(doing_entries.len(), 1);
}

#[test]
#[serial]
fn test_e2e_mv_with_invalid_status() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let issue_name = entries[0].file_name();
    let id = issue_name.to_string_lossy();
    let id_part: String = id.chars().take(3).collect();

    let (success, _, stderr) = run_moth_cmd(&["mv", &id_part, "invalid"], temp_path);
    assert!(!success);
    assert!(stderr.contains("Unknown status"));
}

#[test]
#[serial]
fn test_e2e_show_displays_content() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let issue_path = entries[0].path();
    std::fs::write(&issue_path, "This is test content").unwrap();

    let issue_name = entries[0].file_name();
    let id = issue_name.to_string_lossy();
    let id_part: String = id.chars().take(3).collect();

    let (success, stdout, stderr) = run_moth_cmd(&["show", &id_part], temp_path);
    assert!(success, "Command failed: {}", stderr);
    assert!(stdout.contains("This is test content"));
}

#[test]
#[serial]
fn test_e2e_show_with_nonexistent_id() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);

    let (success, _, stderr) = run_moth_cmd(&["show", "nonexistent"], temp_path);
    assert!(!success);
    assert!(stderr.contains("No issue found"));
}

#[test]
#[serial]
fn test_e2e_rm_deletes_issue() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(entries.len(), 1);

    let issue_name = entries[0].file_name();
    let id = issue_name.to_string_lossy();
    let id_part: String = id.chars().take(3).collect();

    let (success, _, stderr) = run_moth_cmd(&["rm", &id_part], temp_path);
    assert!(success, "Command failed: {}", stderr);

    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(entries.len(), 0);
}

#[test]
#[serial]
fn test_e2e_rm_with_nonexistent_id() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);

    let (success, _, stderr) = run_moth_cmd(&["rm", "nonexistent"], temp_path);
    assert!(!success);
    assert!(stderr.contains("No issue found") || stderr.contains("Error"));
}

#[test]
#[serial]
fn test_e2e_full_workflow() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    let (success, _, _) = run_moth_cmd(&["init"], temp_path);
    assert!(success);

    let (success, _, _) = run_moth_cmd(&["new", "Fix bug", "--no-edit", "-s", "high"], temp_path);
    assert!(success);

    let (success, _, _) = run_moth_cmd(&["new", "Add feature", "--no-edit"], temp_path);
    assert!(success);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(entries.len(), 2);

    let first_issue = entries[0].file_name();
    let id = first_issue.to_string_lossy();
    let id_part: String = id.chars().take(4).collect();

    let (success, _, _) = run_moth_cmd(&["start", &id_part], temp_path);
    assert!(success);

    let doing_dir = temp_path.join(".moth/doing");
    let doing_entries: Vec<_> = std::fs::read_dir(&doing_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(doing_entries.len(), 1);

    let (success, _, _) = run_moth_cmd(&["done", &id_part], temp_path);
    assert!(success);

    let done_dir = temp_path.join(".moth/done");
    let done_entries: Vec<_> = std::fs::read_dir(&done_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(done_entries.len(), 1);

    let ready_entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(ready_entries.len(), 1);
}

#[test]
#[serial]
fn test_e2e_commands_without_init_fail() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    let (success, _, stderr) = run_moth_cmd(&["new", "Test", "--no-edit"], temp_path);
    assert!(!success);
    assert!(
        stderr.contains("not initialized")
            || stderr.contains("No such file")
            || stderr.contains("Error"),
        "Expected error message, got: {}",
        stderr
    );

    let (success, _, stderr) = run_moth_cmd(&["ls"], temp_path);
    assert!(!success);
    assert!(
        stderr.contains("not initialized")
            || stderr.contains("No such file")
            || stderr.contains("Error"),
        "Expected error message, got: {}",
        stderr
    );
}

#[test]
#[serial]
fn test_e2e_partial_id_matching() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    run_moth_cmd(&["new", "Test issue", "--no-edit"], temp_path);

    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    let issue_name = entries[0].file_name();
    let full_id = issue_name.to_string_lossy();
    let short_id: String = full_id.chars().take(3).collect();

    let (success, _, stderr) = run_moth_cmd(&["show", &short_id], temp_path);
    assert!(success, "Short ID should work: {}", stderr);

    let (success, _, stderr) = run_moth_cmd(&["start", &short_id], temp_path);
    assert!(success, "Short ID should work: {}", stderr);
}

#[test]
#[serial]
fn test_e2e_new_respects_no_edit_config() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);

    // Modify config to set no_edit to true
    let config_path = temp_path.join(".moth/config.yml");
    let original_config = std::fs::read_to_string(&config_path).unwrap();
    let modified_config = original_config.replace("no_edit: false", "no_edit: true");
    std::fs::write(&config_path, modified_config).unwrap();

    // Try to create a new issue without --no-edit
    let (success, _stdout, stderr) = run_moth_cmd(&["new", "Test issue with no_edit"], temp_path);
    assert!(success, "Command failed: {}", stderr);

    // Verify that the issue was created
    let ready_dir = temp_path.join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();
    assert!(name.contains("test_issue_with_no_edit"));
}

#[test]
#[serial]
fn test_e2e_new_with_start_flag_moves_to_doing() {
    let temp = setup_test_env();
    let temp_path = temp.path();

    run_moth_cmd(&["init"], temp_path);
    let (success, _stdout, stderr) = run_moth_cmd(
        &["new", "Test issue with start", "--no-edit", "--start"],
        temp_path,
    );

    assert!(success, "Command failed: {}", stderr);

    let doing_dir = temp_path.join(".moth/doing");
    let entries: Vec<_> = std::fs::read_dir(&doing_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    let file_name = entry.file_name();
    let name = file_name.to_string_lossy();
    assert!(name.contains("test_issue_with_start"));
}
