use moth::cmd;
use moth::config::Config;
use moth::store::Store;
use serial_test::serial;
use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    env::set_current_dir(temp_dir.path()).unwrap();
    temp_dir
}

#[test]
#[serial]
fn test_init_creates_moth_directory() {
    let _temp = setup_test_env();

    let result = cmd::init::run();
    assert!(result.is_ok());

    let moth_dir = PathBuf::from(".moth");
    assert!(moth_dir.exists());
    assert!(moth_dir.join("config.yml").exists());
    assert!(moth_dir.join("ready").exists());
    assert!(moth_dir.join("doing").exists());
    assert!(moth_dir.join("done").exists());
}

#[test]
#[serial]
fn test_init_fails_when_already_initialized() {
    let _temp = setup_test_env();

    cmd::init::run().unwrap();
    let result = cmd::init::run();
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("already initialized")
    );
}

#[test]
#[serial]
fn test_new_creates_issue() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("Fix login bug", Some("high"), true, false, None);
    assert!(result.is_ok());

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].slug, "fix_login_bug");
    assert_eq!(issues[0].severity.as_str(), "high");
    assert_eq!(issues[0].status, "ready");
}

#[test]
#[serial]
fn test_new_with_default_severity() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("Add dark mode", None, true, false, None);
    assert!(result.is_ok());

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].severity.as_str(), "med");
}

#[test]
#[serial]
fn test_new_fails_with_empty_title() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("", None, true, false, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));
}

#[test]
#[serial]
fn test_new_fails_with_invalid_severity() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("Test issue", Some("invalid"), true, false, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid severity"));
}

#[test]
#[serial]
fn test_list_shows_all_except_done_by_default() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Issue 1", None, true, false, None).unwrap();
    cmd::new::run("Issue 2", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();

    let ready_issues = store.issues_by_status("ready").unwrap();
    assert_eq!(ready_issues.len(), 2);

    let done_issues = store.issues_by_status("done").unwrap();
    assert_eq!(done_issues.len(), 0);
}

#[test]
#[serial]
fn test_show_displays_issue_content() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", Some("high"), true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = &issues[0].id;

    fs::write(&issues[0].path, "This is the issue content").unwrap();

    let result = cmd::show::run(Some(id));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_show_with_partial_id() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let partial_id = &issues[0].id[..3];

    let result = cmd::show::run(Some(partial_id));
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_show_fails_with_nonexistent_id() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::show::run(Some("nonexistent"));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No issue found"));
}

#[test]
#[serial]
fn test_start_moves_issue_to_doing() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    cmd::start::run(&id).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let ready_issues = store.issues_by_status("ready").unwrap();
    let doing_issues = store.issues_by_status("doing").unwrap();

    assert_eq!(ready_issues.len(), 0);
    assert_eq!(doing_issues.len(), 1);
    assert_eq!(doing_issues[0].id, id);
}

#[test]
#[serial]
fn test_done_moves_issue_to_done() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    cmd::done::run(Some(&id)).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let ready_issues = store.issues_by_status("ready").unwrap();
    let done_issues = store.issues_by_status("done").unwrap();

    assert_eq!(ready_issues.len(), 0);
    assert_eq!(done_issues.len(), 1);
    assert_eq!(done_issues[0].id, id);
}

#[test]
#[serial]
fn test_mv_moves_issue_to_custom_status() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    cmd::mv::run(&id, "doing").unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let ready_issues = store.issues_by_status("ready").unwrap();
    let doing_issues = store.issues_by_status("doing").unwrap();

    assert_eq!(ready_issues.len(), 0);
    assert_eq!(doing_issues.len(), 1);
    assert_eq!(doing_issues[0].id, id);
}

#[test]
#[serial]
fn test_mv_fails_with_invalid_status() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = &issues[0].id;

    let result = cmd::mv::run(id, "invalid_status");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown status"));
}

#[test]
#[serial]
fn test_rm_deletes_issue() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    assert_eq!(issues.len(), 1);

    cmd::rm::run(&id).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();

    assert_eq!(issues.len(), 0);
}

#[test]
#[serial]
fn test_rm_fails_with_nonexistent_id() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::rm::run("nonexistent");
    assert!(result.is_err());
}

#[test]
#[serial]
fn test_full_workflow() {
    let _temp = setup_test_env();

    cmd::init::run().unwrap();

    cmd::new::run("Fix login bug", Some("high"), true, false, None).unwrap();
    cmd::new::run("Add dark mode", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let mut issues = store.issues_by_status("ready").unwrap();
    assert_eq!(issues.len(), 2);

    issues.sort_by(|a, b| a.severity.cmp(&b.severity));
    let high_priority_id = issues[0].id.clone();

    cmd::start::run(&high_priority_id).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let ready_issues = store.issues_by_status("ready").unwrap();
    let doing_issues = store.issues_by_status("doing").unwrap();

    assert_eq!(ready_issues.len(), 1);
    assert_eq!(doing_issues.len(), 1);

    cmd::done::run(Some(&high_priority_id)).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let ready_issues = store.issues_by_status("ready").unwrap();
    let doing_issues = store.issues_by_status("doing").unwrap();
    let done_issues = store.issues_by_status("done").unwrap();

    assert_eq!(ready_issues.len(), 1);
    assert_eq!(doing_issues.len(), 0);
    assert_eq!(done_issues.len(), 1);
}

#[test]
#[serial]
fn test_partial_id_ambiguous() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    for i in 0..10 {
        cmd::new::run(&format!("Issue {}", i), None, true, false, None).unwrap();
    }

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();

    if issues.len() >= 2 && issues[0].id.chars().next() == issues[1].id.chars().next() {
        let partial = &issues[0].id[..1];
        let result = store.find(partial);

        if result.is_err() {
            assert!(result.unwrap_err().to_string().contains("Ambiguous"));
        }
    }
}

#[test]
#[serial]
fn test_issue_sorting_by_severity() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Low severity issue", Some("low"), true, false, None).unwrap();
    cmd::new::run("High severity issue", Some("high"), true, false, None).unwrap();
    cmd::new::run("Critical issue", Some("crit"), true, false, None).unwrap();
    cmd::new::run("Medium severity issue", Some("med"), true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.issues_by_status("ready").unwrap();

    assert_eq!(issues.len(), 4);
    assert_eq!(issues[0].severity.as_str(), "crit");
    assert_eq!(issues[1].severity.as_str(), "high");
    assert_eq!(issues[2].severity.as_str(), "med");
    assert_eq!(issues[3].severity.as_str(), "low");
}

#[test]
#[serial]
fn test_show_no_args_shows_current() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    cmd::start::run(&id).unwrap();

    let result = cmd::show::run(None);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_show_no_args_no_current() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::show::run(None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No current issue"));
}

#[test]
#[serial]
fn test_done_no_args_finishes_current() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true, false, None).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    cmd::start::run(&id).unwrap();

    let result = cmd::done::run(None);
    assert!(result.is_ok());

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let done_issues = store.issues_by_status("done").unwrap();
    assert_eq!(done_issues.len(), 1);
    assert_eq!(done_issues[0].id, id);
}

#[test]
#[serial]
fn test_done_no_args_no_current() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::done::run(None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No current issue"));
}

#[test]
#[serial]
fn test_new_respects_no_edit_config() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    // Modify config to set no_edit to true
    let moth_dir = PathBuf::from(".moth");
    let config_path = moth_dir.join("config.yml");
    let original_config = fs::read_to_string(&config_path).unwrap();
    let modified_config = original_config.replace("no_edit: false", "no_edit: true");
    fs::write(&config_path, modified_config).unwrap();

    // Try to create a new issue without skipping editor
    let result = cmd::new::run("Test issue with no_edit", None, false, false, None);
    assert!(result.is_ok());

    // Verify that the issue was created
    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].slug, "test_issue_with_no_edit");
}

#[test]
#[serial]
#[cfg(unix)]
#[ignore] // Spawns subprocess that hangs in CI - run manually with `cargo test -- --ignored`
fn test_new_with_hooks() {
    use std::os::unix::fs::PermissionsExt;

    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let hooks_dir = PathBuf::from(".moth/hooks/new");
    fs::create_dir_all(hooks_dir.join("before")).unwrap();
    fs::create_dir_all(hooks_dir.join("after")).unwrap();

    let before_hook_path = hooks_dir.join("before/test.sh");
    let after_hook_path = hooks_dir.join("after/test.sh");

    fs::write(&before_hook_path, "echo 'before hook'").unwrap();
    fs::write(&after_hook_path, "echo 'after hook'").unwrap();

    let perms = fs::Permissions::from_mode(0o755);
    fs::set_permissions(&before_hook_path, perms.clone()).unwrap();
    fs::set_permissions(&after_hook_path, perms).unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_moth"))
        .args(["new", "Test issue"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(stdout.contains("before hook"));
    assert!(stdout.contains("after hook"));
}
