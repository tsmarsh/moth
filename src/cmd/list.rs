use crate::config::Config;
use crate::issue::Priority;
use crate::store::Store;
use anyhow::Result;
use colored::Colorize;

pub fn run(status: Option<&str>, show_all: bool) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    if let Some(status_name) = status {
        list_status(&store, status_name)?;
    } else if show_all {
        for status_config in &store.config().statuses {
            list_status(&store, &status_config.name)?;
        }
    } else {
        let num_statuses = store.config().statuses.len();
        for i in 0..num_statuses - 1 {
            let status_name = &store.config().statuses[i].name;
            list_status(&store, status_name)?;
        }
    }

    Ok(())
}

fn list_status(store: &Store, status: &str) -> Result<()> {
    let issues = store.issues_by_status(status)?;

    if issues.is_empty() {
        return Ok(());
    }

    println!("{}", status);

    for issue in issues {
        let priority_str = format_priority(&issue.priority);
        println!("  {} [{}] {}", issue.id, priority_str, issue.title());
    }

    Ok(())
}

fn format_priority(priority: &Priority) -> colored::ColoredString {
    match priority {
        Priority::Crit => "crit".red().bold(),
        Priority::High => "high".yellow(),
        Priority::Med => "med".normal(),
        Priority::Low => "low".blue(),
    }
}
