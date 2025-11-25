use cucumber::then;
use moth::config::Config;
use moth::store::Store;

use super::MothWorld;

#[then("the command succeeds")]
fn command_succeeds(world: &mut MothWorld) {
    let result = world.last_result.as_ref().expect("No result available");
    assert!(result.is_ok(), "Expected success, got error: {:?}", result);
}

#[then("the command fails")]
fn command_fails(world: &mut MothWorld) {
    let result = world.last_result.as_ref().expect("No result available");
    assert!(result.is_err(), "Expected error, but command succeeded");
}

#[then(expr = "the command fails with {string}")]
fn command_fails_with(world: &mut MothWorld, expected_error: String) {
    let result = world.last_result.as_ref().expect("No result available");
    assert!(result.is_err(), "Expected error, but command succeeded");
    let error_msg = result.as_ref().unwrap_err().to_string();
    assert!(
        error_msg.contains(&expected_error),
        "Expected error to contain '{}', got: {}",
        expected_error,
        error_msg
    );
}

#[then("a .moth directory exists")]
fn moth_directory_exists(world: &mut MothWorld) {
    let moth_dir = world.moth_path().join(".moth");
    assert!(moth_dir.exists(), ".moth directory should exist");
}

#[then("a config.yml file exists")]
fn config_file_exists(world: &mut MothWorld) {
    let config_path = world.moth_path().join(".moth/config.yml");
    assert!(config_path.exists(), "config.yml should exist");
}

#[then("ready, doing, done directories exist")]
fn status_directories_exist(world: &mut MothWorld) {
    let base = world.moth_path().join(".moth");
    assert!(base.join("ready").exists(), "ready directory should exist");
    assert!(base.join("doing").exists(), "doing directory should exist");
    assert!(base.join("done").exists(), "done directory should exist");
}

#[then(expr = "{int} issue exists in {string} status")]
fn n_issues_in_status(world: &mut MothWorld, count: usize, status: String) {
    world.set_current_dir();
    let config = Config::load().expect("Failed to load config");
    let store = Store::new(config).expect("Failed to create store");
    let issues = store
        .issues_by_status(&status)
        .expect("Failed to get issues");
    assert_eq!(
        issues.len(),
        count,
        "Expected {} issues in {}, got {}",
        count,
        status,
        issues.len()
    );
}

#[then(expr = "{int} issues exist in {string} status")]
fn n_issues_plural_in_status(world: &mut MothWorld, count: usize, status: String) {
    n_issues_in_status(world, count, status);
}

#[then(expr = "the issue has severity {string}")]
fn issue_has_severity(world: &mut MothWorld, expected_severity: String) {
    world.set_current_dir();
    let config = Config::load().expect("Failed to load config");
    let store = Store::new(config).expect("Failed to create store");
    let issues = store.all_issues().expect("Failed to get issues");

    let issue = issues.last().expect("No issues found");
    assert_eq!(
        issue.severity.as_str(),
        expected_severity,
        "Expected severity '{}', got '{}'",
        expected_severity,
        issue.severity.as_str()
    );
}

#[then(expr = "the issue has slug {string}")]
fn issue_has_slug(_world: &mut MothWorld, expected_slug: String) {
    let config = Config::load().expect("Failed to load config");
    let store = Store::new(config).expect("Failed to create store");
    let issues = store.all_issues().expect("Failed to get issues");

    let issue = issues.last().expect("No issues found");
    assert_eq!(
        issue.slug, expected_slug,
        "Expected slug '{}', got '{}'",
        expected_slug, issue.slug
    );
}

#[then("no issues exist")]
fn no_issues_exist(_world: &mut MothWorld) {
    let config = Config::load().expect("Failed to load config");
    let store = Store::new(config).expect("Failed to create store");
    let issues = store.all_issues().expect("Failed to get issues");
    assert!(
        issues.is_empty(),
        "Expected no issues, got {}",
        issues.len()
    );
}

#[then(expr = "{int} issues exist total")]
fn n_issues_total(_world: &mut MothWorld, count: usize) {
    let config = Config::load().expect("Failed to load config");
    let store = Store::new(config).expect("Failed to create store");
    let issues = store.all_issues().expect("Failed to get issues");
    assert_eq!(
        issues.len(),
        count,
        "Expected {} total issues, got {}",
        count,
        issues.len()
    );
}

#[then("the issue is in the ready directory")]
fn issue_in_ready(world: &mut MothWorld) {
    let ready_dir = world.moth_path().join(".moth/ready");
    let entries: Vec<_> = std::fs::read_dir(&ready_dir)
        .expect("Failed to read ready dir")
        .filter_map(|e| e.ok())
        .collect();
    assert!(!entries.is_empty(), "No issues in ready directory");
}

#[then("the issue is in the doing directory")]
fn issue_in_doing(world: &mut MothWorld) {
    let doing_dir = world.moth_path().join(".moth/doing");
    let entries: Vec<_> = std::fs::read_dir(&doing_dir)
        .expect("Failed to read doing dir")
        .filter_map(|e| e.ok())
        .collect();
    assert!(!entries.is_empty(), "No issues in doing directory");
}

#[then("the issue is in the done directory")]
fn issue_in_done(world: &mut MothWorld) {
    let done_dir = world.moth_path().join(".moth/done");
    let entries: Vec<_> = std::fs::read_dir(&done_dir)
        .expect("Failed to read done dir")
        .filter_map(|e| e.ok())
        .collect();
    assert!(!entries.is_empty(), "No issues in done directory");
}

#[then(expr = "the issue filename contains {string}")]
fn issue_filename_contains(world: &mut MothWorld, expected: String) {
    let ready_dir = world.moth_path().join(".moth/ready");
    let doing_dir = world.moth_path().join(".moth/doing");

    let mut found = false;
    for dir in [ready_dir, doing_dir] {
        if dir.exists() {
            for entry in std::fs::read_dir(&dir).expect("Failed to read dir") {
                let entry = entry.expect("Failed to read entry");
                let filename = entry.file_name().to_string_lossy().to_string();
                if filename.contains(&expected) {
                    found = true;
                    break;
                }
            }
        }
    }
    assert!(found, "No issue filename contains '{}'", expected);
}

#[then("partial ID matching works for the issue")]
fn partial_id_works(world: &mut MothWorld) {
    let id = world.last_issue_id.as_ref().expect("No issue ID");
    let partial: String = id.chars().take(3).collect();

    let config = Config::load().expect("Failed to load config");
    let store = Store::new(config).expect("Failed to create store");
    let result = store.find(&partial);

    assert!(result.is_ok(), "Partial ID lookup should succeed");
}
