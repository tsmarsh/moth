use crate::config::Config;
use crate::issue::Severity;
use crate::store::Store;
use anyhow::Result;
use colored::Colorize;

pub fn run(status: Option<&str>, show_all: bool, severity_filter: Option<Severity>) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    if let Some(status_name) = status {
        list_status(&store, status_name, severity_filter)?;
    } else if show_all {
        for status_config in &store.config().statuses {
            list_status(&store, &status_config.name, severity_filter)?;
        }
    } else {
        let num_statuses = store.config().statuses.len();
        for i in 0..num_statuses - 1 {
            let status_name = &store.config().statuses[i].name;
            list_status(&store, status_name, severity_filter)?;
        }
    }

    Ok(())
}

fn list_status(store: &Store, status: &str, severity_filter: Option<Severity>) -> Result<()> {
    let issues = store.issues_by_status(status)?;

    let filtered_issues: Vec<_> = if let Some(sev) = severity_filter {
        issues.into_iter().filter(|i| i.severity == sev).collect()
    } else {
        issues
    };

    if filtered_issues.is_empty() {
        return Ok(());
    }

    println!("{}", status);

    for issue in filtered_issues {
        let severity_str = format_severity(&issue.severity);
        println!("  {} [{}] {}", issue.id, severity_str, issue.title());
    }

    Ok(())
}

fn format_severity(severity: &Severity) -> colored::ColoredString {
    match severity {
        Severity::Crit => "crit".red().bold(),
        Severity::High => "high".yellow(),
        Severity::Med => "med".normal(),
        Severity::Low => "low".blue(),
    }
}
