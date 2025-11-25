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

    let result = cmd::new::run("Fix login bug", Some("high"), true);
    assert!(result.is_ok());

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].slug, "fix_login_bug");
    assert_eq!(issues[0].priority.as_str(), "high");
    assert_eq!(issues[0].status, "ready");
}

#[test]
#[serial]
fn test_new_with_default_priority() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("Add dark mode", None, true);
    assert!(result.is_ok());

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].priority.as_str(), "med");
}

#[test]
#[serial]
fn test_new_fails_with_empty_title() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("", None, true);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("cannot be empty"));
}

#[test]
#[serial]
fn test_new_fails_with_invalid_priority() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::new::run("Test issue", Some("invalid"), true);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid priority"));
}

#[test]
#[serial]
fn test_list_shows_all_except_done_by_default() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Issue 1", None, true).unwrap();
    cmd::new::run("Issue 2", None, true).unwrap();

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

    cmd::new::run("Test issue", Some("high"), true).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = &issues[0].id;

    fs::write(&issues[0].path, "This is the issue content").unwrap();

    let result = cmd::show::run(id);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_show_with_partial_id() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let partial_id = &issues[0].id[..3];

    let result = cmd::show::run(partial_id);
    assert!(result.is_ok());
}

#[test]
#[serial]
fn test_show_fails_with_nonexistent_id() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    let result = cmd::show::run("nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No issue found"));
}

#[test]
#[serial]
fn test_start_moves_issue_to_doing() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Test issue", None, true).unwrap();

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

    cmd::new::run("Test issue", None, true).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.all_issues().unwrap();
    let id = issues[0].id.clone();

    cmd::done::run(&id).unwrap();

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

    cmd::new::run("Test issue", None, true).unwrap();

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

    cmd::new::run("Test issue", None, true).unwrap();

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

    cmd::new::run("Test issue", None, true).unwrap();

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

    cmd::new::run("Fix login bug", Some("high"), true).unwrap();
    cmd::new::run("Add dark mode", None, true).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let mut issues = store.issues_by_status("ready").unwrap();
    assert_eq!(issues.len(), 2);

    issues.sort_by(|a, b| a.priority.cmp(&b.priority));
    let high_priority_id = issues[0].id.clone();

    cmd::start::run(&high_priority_id).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let ready_issues = store.issues_by_status("ready").unwrap();
    let doing_issues = store.issues_by_status("doing").unwrap();

    assert_eq!(ready_issues.len(), 1);
    assert_eq!(doing_issues.len(), 1);

    cmd::done::run(&high_priority_id).unwrap();

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
        cmd::new::run(&format!("Issue {}", i), None, true).unwrap();
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
fn test_issue_sorting_by_priority() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    cmd::new::run("Low priority issue", Some("low"), true).unwrap();
    cmd::new::run("High priority issue", Some("high"), true).unwrap();
    cmd::new::run("Critical issue", Some("crit"), true).unwrap();
    cmd::new::run("Medium priority issue", Some("med"), true).unwrap();

    let config = Config::load().unwrap();
    let store = Store::new(config).unwrap();
    let issues = store.issues_by_status("ready").unwrap();

    assert_eq!(issues.len(), 4);
    assert_eq!(issues[0].priority.as_str(), "crit");
    assert_eq!(issues[1].priority.as_str(), "high");
    assert_eq!(issues[2].priority.as_str(), "med");
    assert_eq!(issues[3].priority.as_str(), "low");
}

#[test]
#[serial]
fn test_new_respects_no_edit_on_new_config() {
    let _temp = setup_test_env();
    cmd::init::run().unwrap();

    // Modify config to set no_edit_on_new to true
    let moth_dir = PathBuf::from(".moth");
    let config_path = moth_dir.join("config.yml");
    let original_config = fs::read_to_string(&config_path).unwrap();
    let modified_config = original_config.replace("no_edit_on_new: false", "no_edit_on_new: true");
    fs::write(&config_path, modified_config).unwrap();

    // Try to create a new issue without skipping editor
    let result = cmd::new::run("Test issue with no_edit_on_new", None, false);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Editing is disabled by configuration (no_edit_on_new: true).")
    );
}
