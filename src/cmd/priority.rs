use crate::config::Config;
use crate::store::Store;
use anyhow::{Result, anyhow};
use std::fs;

pub fn run(id: &str, position: &str, other_id: Option<&str>, compact: Option<bool>) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let mut issue = store.find(id)?;

    // Get the status config to check if it's prioritized
    let status_config = store
        .config()
        .get_status(&issue.status)
        .ok_or_else(|| anyhow!("Unknown status: {}", issue.status))?;

    if !status_config.prioritized {
        return Err(anyhow!(
            "Status '{}' is not configured for prioritization",
            issue.status
        ));
    }

    // Get all issues in the same status
    let issues = store.issues_by_status(&issue.status)?;

    // Calculate new order based on position
    let new_order = match position {
        "top" => {
            // Find lowest existing priority, or 1
            let min_order = issues.iter().filter_map(|i| i.order).min().unwrap_or(1);
            Some(if min_order > 1 { min_order - 1 } else { 1 })
        }
        "bottom" => {
            // Remove priority (None means bottom)
            None
        }
        "above" => {
            let other = other_id.ok_or_else(|| anyhow!("Missing target issue ID for 'above'"))?;
            let target = store.find(other)?;

            if target.status != issue.status {
                return Err(anyhow!(
                    "Target issue is in different status: {} vs {}",
                    target.status,
                    issue.status
                ));
            }

            target.order.map(|o| if o > 0 { o - 1 } else { 1 })
        }
        "below" => {
            let other = other_id.ok_or_else(|| anyhow!("Missing target issue ID for 'below'"))?;
            let target = store.find(other)?;

            if target.status != issue.status {
                return Err(anyhow!(
                    "Target issue is in different status: {} vs {}",
                    target.status,
                    issue.status
                ));
            }

            target.order.map(|o| o + 1)
        }
        number_str => {
            // Try to parse as number
            let num = number_str
                .parse::<u32>()
                .map_err(|_| anyhow!("Invalid position: {}", position))?;
            Some(num)
        }
    };

    // Update the issue's order
    issue.order = new_order;

    // Rename the file
    let status_dir = store.config().status_dir(status_config);
    let new_path = status_dir.join(issue.filename());

    fs::rename(&issue.path, &new_path)?;

    if let Some(order) = new_order {
        println!("Set priority of {} to {}", issue.id, order);
    } else {
        println!("Removed priority from {}", issue.id);
    }

    // Auto-compact if configured
    let should_compact = compact.unwrap_or(store.config().priority.auto_compact);
    if should_compact {
        compact_status(&issue.status, &store)?;
    }

    Ok(())
}

fn compact_status(status: &str, store: &Store) -> Result<()> {
    let status_config = store
        .config()
        .get_status(status)
        .ok_or_else(|| anyhow!("Unknown status: {}", status))?;

    if !status_config.prioritized {
        return Err(anyhow!(
            "Status '{}' is not configured for prioritization",
            status
        ));
    }

    let mut issues = store.issues_by_status(status)?;

    // Filter to only ordered issues and sort them
    let mut ordered: Vec<_> = issues.iter_mut().filter(|i| i.order.is_some()).collect();

    ordered.sort_by_key(|i| i.order.unwrap());

    // Renumber sequentially
    let status_dir = store.config().status_dir(status_config);
    for (idx, issue) in ordered.iter_mut().enumerate() {
        let new_order = (idx + 1) as u32;
        if issue.order != Some(new_order) {
            let old_path = issue.path.clone();
            issue.order = Some(new_order);
            let new_path = status_dir.join(issue.filename());
            fs::rename(&old_path, &new_path)?;
            issue.path = new_path;
        }
    }

    println!(
        "Compacted {} prioritized issues in {}",
        ordered.len(),
        status
    );

    Ok(())
}

pub fn compact(status: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    if let Some(status_name) = status {
        compact_status(status_name, &store)?;
    } else {
        // Compact all prioritized statuses
        for status_config in &store.config().statuses {
            if status_config.prioritized {
                compact_status(&status_config.name, &store)?;
            }
        }
    }

    Ok(())
}
