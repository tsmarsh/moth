use crate::config::Config;
use crate::issue::{Issue, Severity, generate_id};
use anyhow::{Context, Result, anyhow};
use std::fs;

pub struct Store {
    config: Config,
}

impl Store {
    pub fn new(config: Config) -> Result<Self> {
        for status in &config.statuses {
            let dir = config.status_dir(status);
            if !dir.exists() {
                return Err(anyhow!(
                    "Status directory does not exist: {}. Try running 'moth init' first.",
                    dir.display()
                ));
            }
        }

        Ok(Store { config })
    }

    pub fn find(&self, partial_id: &str) -> Result<Issue> {
        let all_issues = self.all_issues()?;
        let matches: Vec<&Issue> = all_issues
            .iter()
            .filter(|issue| issue.id.starts_with(partial_id))
            .collect();

        match matches.len() {
            0 => Err(anyhow!("No issue found with ID: {}", partial_id)),
            1 => Ok(matches[0].clone()),
            _ => {
                let ids: Vec<String> = matches.iter().map(|i| i.id.clone()).collect();
                Err(anyhow!(
                    "Ambiguous ID '{}'. Matches: {}",
                    partial_id,
                    ids.join(", ")
                ))
            }
        }
    }

    pub fn all_issues(&self) -> Result<Vec<Issue>> {
        let mut issues = Vec::new();

        for status in &self.config.statuses {
            let status_issues = self.issues_by_status(&status.name)?;
            issues.extend(status_issues);
        }

        Ok(issues)
    }

    pub fn issues_by_status(&self, status: &str) -> Result<Vec<Issue>> {
        let status_config = self
            .config
            .get_status(status)
            .ok_or_else(|| anyhow!("Unknown status: {}", status))?;

        let dir = self.config.status_dir(status_config);
        let mut issues = Vec::new();

        if !dir.exists() {
            return Ok(issues);
        }

        let entries = fs::read_dir(&dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match Issue::from_path(&path, status) {
                    Ok(issue) => issues.push(issue),
                    Err(e) => eprintln!("Warning: Failed to parse {}: {}", path.display(), e),
                }
            }
        }

        issues.sort_by(|a, b| {
            // First sort by order (if present), then by severity, then by slug
            match (a.order, b.order) {
                (Some(a_order), Some(b_order)) => a_order.cmp(&b_order),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a
                    .severity
                    .cmp(&b.severity)
                    .then_with(|| a.slug.cmp(&b.slug)),
            }
        });

        Ok(issues)
    }

    pub fn move_issue(&self, issue: &Issue, target_status: &str) -> Result<()> {
        let target_config = self
            .config
            .get_status(target_status)
            .ok_or_else(|| anyhow!("Unknown status: {}", target_status))?;

        let target_dir = self.config.status_dir(target_config);

        // Strip priority order if target status is not prioritized
        let mut updated_issue = issue.clone();
        if !target_config.prioritized {
            updated_issue.order = None;
        }

        let new_path = target_dir.join(updated_issue.filename());

        fs::rename(&issue.path, &new_path).with_context(|| {
            format!(
                "Failed to move {} to {}",
                issue.path.display(),
                new_path.display()
            )
        })?;

        Ok(())
    }

    pub fn delete_issue(&self, issue: &Issue) -> Result<()> {
        fs::remove_file(&issue.path)
            .with_context(|| format!("Failed to delete {}", issue.path.display()))?;
        Ok(())
    }

    pub fn create_issue(&self, title: &str, severity: Severity) -> Result<Issue> {
        if title.trim().is_empty() {
            return Err(anyhow!("Issue title cannot be empty"));
        }

        let slug = title_to_slug(title);
        let id = self.generate_unique_id()?;

        let first_status = self.config.first_status();
        let dir = self.config.status_dir(first_status);

        let filename = format!("{}-{}-{}.md", id, severity.as_str(), slug);
        let path = dir.join(&filename);

        fs::write(&path, "")
            .with_context(|| format!("Failed to create issue file: {}", path.display()))?;

        Issue::from_path(&path, &first_status.name)
    }

    fn generate_unique_id(&self) -> Result<String> {
        let max_attempts = 100;
        let all_issues = self.all_issues()?;
        let existing_ids: std::collections::HashSet<String> =
            all_issues.iter().map(|i| i.id.clone()).collect();

        for _ in 0..max_attempts {
            let id = generate_id(self.config.id_length);
            if !existing_ids.contains(&id) {
                return Ok(id);
            }
        }

        Err(anyhow!(
            "Failed to generate unique ID after {} attempts",
            max_attempts
        ))
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn current(&self) -> Result<Option<Issue>> {
        let doing_status = self
            .config
            .get_status("doing")
            .ok_or_else(|| anyhow!("'doing' status not configured"))?;
        let doing_dir = self.config.status_dir(doing_status);

        if !doing_dir.exists() {
            return Ok(None);
        }

        let mut latest_issue: Option<Issue> = None;
        let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

        let entries = fs::read_dir(&doing_dir)
            .with_context(|| format!("Failed to read directory: {}", doing_dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let metadata = fs::metadata(&path)?;
                let modified_time = metadata.modified()?;

                if modified_time > latest_time {
                    latest_time = modified_time;
                    match Issue::from_path(&path, "doing") {
                        Ok(issue) => latest_issue = Some(issue),
                        Err(e) => eprintln!("Warning: Failed to parse {}: {}", path.display(), e),
                    }
                }
            }
        }

        Ok(latest_issue)
    }
}

fn title_to_slug(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_slug() {
        assert_eq!(title_to_slug("Fix Login Bug"), "fix_login_bug");
        assert_eq!(title_to_slug("Add Dark Mode"), "add_dark_mode");
        assert_eq!(
            title_to_slug("Fix   Multiple   Spaces"),
            "fix_multiple_spaces"
        );
        assert_eq!(title_to_slug("API/REST endpoint"), "api_rest_endpoint");
    }

    #[test]
    fn test_title_to_slug_edge_cases() {
        // Leading/trailing whitespace
        assert_eq!(title_to_slug("  trim me  "), "trim_me");
        // Special characters
        assert_eq!(title_to_slug("Fix #123: Bug!"), "fix_123_bug");
        // Mixed case
        assert_eq!(title_to_slug("CamelCase Title"), "camelcase_title");
        // Numbers
        assert_eq!(title_to_slug("Version 2.0 Release"), "version_2_0_release");
        // Underscores preserved
        assert_eq!(title_to_slug("snake_case_title"), "snake_case_title");
        // Empty after processing
        assert_eq!(title_to_slug("   "), "");
        // Unicode (non-ASCII becomes underscore)
        assert_eq!(title_to_slug("Café Mode"), "caf_mode");
    }
}
