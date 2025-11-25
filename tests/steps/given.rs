use cucumber::given;
use moth::cmd;
use moth::config::Config;
use moth::store::Store;
use tempfile::TempDir;

use super::MothWorld;

fn get_last_issue_id() -> Option<String> {
    let config = Config::load().ok()?;
    let store = Store::new(config).ok()?;
    let issues = store.all_issues().ok()?;
    issues.last().map(|i| i.id.clone())
}

#[given("an empty directory")]
fn empty_directory(world: &mut MothWorld) {
    world.temp_dir = Some(TempDir::new().expect("Failed to create temp dir"));
    world.set_current_dir();
}

#[given("a moth workspace is initialized")]
fn initialized_workspace(world: &mut MothWorld) {
    world.temp_dir = Some(TempDir::new().expect("Failed to create temp dir"));
    world.set_current_dir();
    cmd::init::run().expect("Failed to initialize moth workspace");
}

#[given(expr = "an issue {string} exists")]
fn issue_exists(world: &mut MothWorld, title: String) {
    cmd::new::run(&title, None, true, false).expect("Failed to create issue");
    world.last_issue_id = get_last_issue_id();
}

#[given(expr = "an issue {string} exists with severity {string}")]
fn issue_exists_with_severity(world: &mut MothWorld, title: String, severity: String) {
    cmd::new::run(&title, Some(&severity), true, false).expect("Failed to create issue");
    world.last_issue_id = get_last_issue_id();
}

#[given(expr = "the issue is started")]
fn issue_is_started(world: &mut MothWorld) {
    let id = world.last_issue_id.as_ref().expect("No issue ID available");
    cmd::start::run(id).expect("Failed to start issue");
}

#[given(expr = "the config has no_edit set to true")]
fn config_no_edit_true(_world: &mut MothWorld) {
    let config_path = std::path::PathBuf::from(".moth/config.yml");
    let original = std::fs::read_to_string(&config_path).expect("Failed to read config");
    let modified = original.replace("no_edit: false", "no_edit: true");
    std::fs::write(&config_path, modified).expect("Failed to write config");
}

#[given(expr = "{int} issues exist")]
fn multiple_issues_exist(world: &mut MothWorld, count: usize) {
    for i in 0..count {
        cmd::new::run(&format!("Issue {}", i + 1), None, true, false)
            .expect("Failed to create issue");
    }
    world.last_issue_id = get_last_issue_id();
}
