use cucumber::when;
use moth::cmd;
use moth::config::Config;
use moth::store::Store;

use super::MothWorld;

fn get_last_issue_id() -> Option<String> {
    let config = Config::load().ok()?;
    let store = Store::new(config).ok()?;
    let issues = store.all_issues().ok()?;
    issues.last().map(|i| i.id.clone())
}

#[when("the user runs init")]
fn user_runs_init(world: &mut MothWorld) {
    world.last_result = Some(cmd::init::run().map(|_| ()));
}

#[when(expr = "the user creates issue {string}")]
fn user_creates_issue(world: &mut MothWorld, title: String) {
    let result = cmd::new::run(&title, None, true, false, None);
    match result {
        Ok(()) => {
            world.last_issue_id = get_last_issue_id();
            world.last_result = Some(Ok(()));
        }
        Err(e) => {
            world.last_result = Some(Err(e));
        }
    }
}

#[when(expr = "the user creates issue {string} with severity {string}")]
fn user_creates_issue_with_severity(world: &mut MothWorld, title: String, severity: String) {
    let result = cmd::new::run(&title, Some(&severity), true, false, None);
    match result {
        Ok(()) => {
            world.last_issue_id = get_last_issue_id();
            world.last_result = Some(Ok(()));
        }
        Err(e) => {
            world.last_result = Some(Err(e));
        }
    }
}

#[when(expr = "the user creates issue {string} with --start flag")]
fn user_creates_issue_with_start(world: &mut MothWorld, title: String) {
    let result = cmd::new::run(&title, None, true, true, None);
    match result {
        Ok(()) => {
            world.last_issue_id = get_last_issue_id();
            world.last_result = Some(Ok(()));
        }
        Err(e) => {
            world.last_result = Some(Err(e));
        }
    }
}

#[when(expr = "the user creates issue {string} without --no-edit")]
fn user_creates_issue_without_no_edit(world: &mut MothWorld, title: String) {
    // skip_editor = false means editor would open (but config may override)
    let result = cmd::new::run(&title, None, false, false, None);
    match result {
        Ok(()) => {
            world.last_issue_id = get_last_issue_id();
            world.last_result = Some(Ok(()));
        }
        Err(e) => {
            world.last_result = Some(Err(e));
        }
    }
}

#[when("the user lists issues")]
fn user_lists_issues(world: &mut MothWorld) {
    world.last_result = Some(cmd::list::run(None, false, None).map(|_| ()));
}

#[when(expr = "the user lists issues with status {string}")]
fn user_lists_issues_with_status(world: &mut MothWorld, status: String) {
    world.last_result = Some(cmd::list::run(Some(&status), false, None).map(|_| ()));
}

#[when(expr = "the user shows issue {string}")]
fn user_shows_issue(world: &mut MothWorld, id: String) {
    world.last_result = Some(cmd::show::run(Some(&id)).map(|_| ()));
}

#[when("the user shows the current issue")]
fn user_shows_current_issue(world: &mut MothWorld) {
    world.last_result = Some(cmd::show::run(None).map(|_| ()));
}

#[when(expr = "the user starts issue {string}")]
fn user_starts_issue(world: &mut MothWorld, id: String) {
    world.last_result = Some(cmd::start::run(&id).map(|_| ()));
}

#[when("the user starts the last created issue")]
fn user_starts_last_issue(world: &mut MothWorld) {
    let id = world.last_issue_id.clone().expect("No issue ID available");
    world.last_result = Some(cmd::start::run(&id).map(|_| ()));
}

#[when(expr = "the user marks issue {string} as done")]
fn user_marks_done(world: &mut MothWorld, id: String) {
    world.last_result = Some(cmd::done::run(Some(&id)).map(|_| ()));
}

#[when("the user marks the current issue as done")]
fn user_marks_current_done(world: &mut MothWorld) {
    world.last_result = Some(cmd::done::run(None).map(|_| ()));
}

#[when("the user marks the last created issue as done")]
fn user_marks_last_done(world: &mut MothWorld) {
    let id = world.last_issue_id.clone().expect("No issue ID available");
    world.last_result = Some(cmd::done::run(Some(&id)).map(|_| ()));
}

#[when(expr = "the user moves issue {string} to {string}")]
fn user_moves_issue(world: &mut MothWorld, id: String, status: String) {
    world.last_result = Some(cmd::mv::run(&id, &status).map(|_| ()));
}

#[when(expr = "the user moves the last created issue to {string}")]
fn user_moves_last_issue(world: &mut MothWorld, status: String) {
    let id = world.last_issue_id.clone().expect("No issue ID available");
    world.last_result = Some(cmd::mv::run(&id, &status).map(|_| ()));
}

#[when(expr = "the user deletes issue {string}")]
fn user_deletes_issue(world: &mut MothWorld, id: String) {
    world.last_result = Some(cmd::rm::run(&id).map(|_| ()));
}

#[when("the user deletes the last created issue")]
fn user_deletes_last_issue(world: &mut MothWorld) {
    let id = world.last_issue_id.clone().expect("No issue ID available");
    world.last_result = Some(cmd::rm::run(&id).map(|_| ()));
}

#[when(expr = "the user shows issue with partial id {string}")]
fn user_shows_partial_id(world: &mut MothWorld, partial_id: String) {
    world.last_result = Some(cmd::show::run(Some(&partial_id)).map(|_| ()));
}

// Priority commands
#[when(expr = "the user sets priority of the last issue to {string}")]
fn user_sets_priority(world: &mut MothWorld, position: String) {
    let id = world.last_issue_id.clone().expect("No issue ID available");
    world.last_result = Some(cmd::priority::run(&id, &position, None, None).map(|_| ()));
}

#[when("the user compacts priorities")]
fn user_compacts_priorities(world: &mut MothWorld) {
    world.last_result = Some(cmd::priority::compact(None).map(|_| ()));
}

// Severity commands
#[when(expr = "the user changes severity of the last issue to {string}")]
fn user_changes_severity(world: &mut MothWorld, level: String) {
    let id = world.last_issue_id.clone().expect("No issue ID available");
    let severity = level.parse();
    match severity {
        Ok(sev) => {
            world.last_result = Some(cmd::severity::run(&id, sev).map(|_| ()));
        }
        Err(_) => {
            world.last_result = Some(Err(anyhow::anyhow!("Invalid severity: {}", level)));
        }
    }
}

#[when(expr = "the user changes severity of issue {string} to {string}")]
fn user_changes_severity_by_id(world: &mut MothWorld, id: String, level: String) {
    let severity = level.parse();
    match severity {
        Ok(sev) => {
            world.last_result = Some(cmd::severity::run(&id, sev).map(|_| ()));
        }
        Err(_) => {
            world.last_result = Some(Err(anyhow::anyhow!("Invalid severity: {}", level)));
        }
    }
}

// Hook commands
#[when("the user installs the hook")]
fn user_installs_hook(world: &mut MothWorld) {
    world.last_result = Some(cmd::hook::install(false, false).map(|_| ()));
}

#[when("the user installs the hook with force")]
fn user_installs_hook_force(world: &mut MothWorld) {
    world.last_result = Some(cmd::hook::install(true, false).map(|_| ()));
}

#[when("the user uninstalls the hook")]
fn user_uninstalls_hook(world: &mut MothWorld) {
    world.last_result = Some(cmd::hook::uninstall().map(|_| ()));
}
