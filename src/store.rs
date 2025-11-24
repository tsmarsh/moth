use crate::config::Config;
use crate::issue::{Issue, Priority, generate_id};
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
            a.priority
                .cmp(&b.priority)
                .then_with(|| a.slug.cmp(&b.slug))
        });

        Ok(issues)
    }

    pub fn move_issue(&self, issue: &Issue, target_status: &str) -> Result<()> {
        let target_config = self
            .config
            .get_status(target_status)
            .ok_or_else(|| anyhow!("Unknown status: {}", target_status))?;

        let target_dir = self.config.status_dir(target_config);
        let new_path = target_dir.join(issue.filename());

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

    pub fn create_issue(&self, title: &str, priority: Priority) -> Result<Issue> {
        if title.trim().is_empty() {
            return Err(anyhow!("Issue title cannot be empty"));
        }

        let slug = title_to_slug(title);
        let id = self.generate_unique_id()?;

        let first_status = self.config.first_status();
        let dir = self.config.status_dir(first_status);

        let filename = format!("{}-{}-{}.md", id, priority.as_str(), slug);
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
}

fn title_to_slug(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_slug() {
        assert_eq!(title_to_slug("Fix Login Bug"), "fix-login-bug");
        assert_eq!(title_to_slug("Add Dark Mode"), "add-dark-mode");
        assert_eq!(
            title_to_slug("Fix   Multiple   Spaces"),
            "fix-multiple-spaces"
        );
        assert_eq!(title_to_slug("API/REST endpoint"), "api-rest-endpoint");
    }
}
