use crate::issue::parse_issue_prefix;
use anyhow::{Context, Result, anyhow};
use git2::{Commit, Repository};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StoryKey {
    id: String,
    severity: String,
    slug: String,
}

#[derive(Debug, Clone)]
struct StoryState {
    key: StoryKey,
    column: String,
    content: String,
}

#[derive(Debug)]
enum ChangeEvent {
    Created,
    Moved,
    Edited,
    Deleted,
    CodeCommit,
}

impl ChangeEvent {
    fn as_str(&self) -> &'static str {
        match self {
            ChangeEvent::Created => "created",
            ChangeEvent::Moved => "moved",
            ChangeEvent::Edited => "edited",
            ChangeEvent::Deleted => "deleted",
            ChangeEvent::CodeCommit => "code_commit",
        }
    }
}

pub fn run(since: Option<&str>, until: Option<&str>) -> Result<()> {
    let repo = Repository::open(".").context("Failed to open git repository")?;

    // Get the commit range
    let commits = get_commits(&repo, since, until)?;

    // Print CSV header
    println!(
        "commit_sha,commit_date,committer_name,committer_email,story_id,severity,column,event,message"
    );

    // Track previous state
    let mut prev_state: HashMap<String, StoryState> = HashMap::new();

    for commit in commits {
        let current_state = extract_stories(&repo, &commit)?;
        let changes = detect_changes(&prev_state, &current_state);

        let commit_time = commit.committer().when();
        let timestamp = chrono::DateTime::from_timestamp(commit_time.seconds(), 0)
            .unwrap_or_default()
            .format("%Y-%m-%dT%H:%M:%SZ");

        // Output .moth file changes
        for (story_id, event, story) in &changes {
            println!(
                "{},{},{},{},{},{},{},{},",
                commit.id(),
                timestamp,
                escape_csv(commit.committer().name().unwrap_or("")),
                escape_csv(commit.committer().email().unwrap_or("")),
                escape_csv(story_id),
                escape_csv(&story.key.severity),
                escape_csv(&story.column),
                event.as_str()
            );
        }

        // Check for code commit referencing an issue
        if let Some((issue_id, message)) = parse_issue_prefix(commit.message().unwrap_or("")) {
            // Look up issue state (try current, fall back to prev)
            if let Some(story) = current_state
                .get(&issue_id)
                .or_else(|| prev_state.get(&issue_id))
            {
                println!(
                    "{},{},{},{},{},{},{},{},{}",
                    commit.id(),
                    timestamp,
                    escape_csv(commit.committer().name().unwrap_or("")),
                    escape_csv(commit.committer().email().unwrap_or("")),
                    escape_csv(&issue_id),
                    escape_csv(&story.key.severity),
                    escape_csv(&story.column),
                    ChangeEvent::CodeCommit.as_str(),
                    escape_csv(&message)
                );
            }
        }

        prev_state = current_state;
    }

    Ok(())
}

fn get_commits<'a>(
    repo: &'a Repository,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<Vec<Commit<'a>>> {
    let mut revwalk = repo.revwalk()?;

    // Start from the until commit if provided, otherwise HEAD
    let end_oid = match until {
        Some(rev) => repo.revparse_single(rev)?.id(),
        None => repo
            .head()?
            .target()
            .ok_or_else(|| anyhow!("HEAD has no target"))?,
    };

    revwalk.push(end_oid)?;

    // Find the since commit OID if provided
    let since_oid = if let Some(rev) = since {
        Some(repo.revparse_single(rev)?.id())
    } else {
        None
    };

    let mut commits = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        // Stop if we reached the since commit (exclusive)
        if let Some(since_id) = since_oid
            && oid == since_id
        {
            break;
        }

        commits.push(commit);
    }

    // Reverse to get chronological order (oldest first)
    commits.reverse();

    Ok(commits)
}

fn extract_stories(repo: &Repository, commit: &Commit) -> Result<HashMap<String, StoryState>> {
    let mut stories = HashMap::new();

    let tree = commit.tree()?;

    // Try to find the .moth directory
    let moth_entry = match tree.get_name(".moth") {
        Some(entry) => entry,
        None => return Ok(stories), // No .moth directory, return empty
    };

    let moth_tree = repo.find_tree(moth_entry.id())?;

    // Iterate through status directories
    for entry in moth_tree.iter() {
        let name = entry.name().unwrap_or("");
        if name.starts_with('.') {
            continue; // Skip hidden files/directories
        }

        // This should be a directory (status column)
        if entry.kind() != Some(git2::ObjectType::Tree) {
            continue;
        }

        let column = name.to_string();
        let status_tree = repo.find_tree(entry.id())?;

        // Iterate through story files in this status
        for story_entry in status_tree.iter() {
            let filename = story_entry.name().unwrap_or("");

            if !filename.ends_with(".md") {
                continue;
            }

            // Parse the filename: {id}-{severity}-{slug}.md
            if let Some(story) = parse_story_filename(filename) {
                // Get file content
                let blob = match repo.find_blob(story_entry.id()) {
                    Ok(blob) => blob,
                    Err(_) => continue, // Skip if we can't read it
                };

                let content = String::from_utf8_lossy(blob.content()).to_string();

                let state = StoryState {
                    key: story,
                    column: column.clone(),
                    content,
                };

                stories.insert(state.key.id.clone(), state);
            }
        }
    }

    Ok(stories)
}

fn parse_story_filename(filename: &str) -> Option<StoryKey> {
    // Remove .md extension
    let name = filename.strip_suffix(".md")?;

    // Split by hyphen
    let parts: Vec<&str> = name.splitn(3, '-').collect();

    if parts.len() < 3 {
        return None;
    }

    Some(StoryKey {
        id: parts[0].to_string(),
        severity: parts[1].to_string(),
        slug: parts[2].to_string(),
    })
}

fn detect_changes(
    prev: &HashMap<String, StoryState>,
    current: &HashMap<String, StoryState>,
) -> Vec<(String, ChangeEvent, StoryState)> {
    let mut changes = Vec::new();

    // Check for new and modified stories
    for (id, curr_story) in current {
        match prev.get(id) {
            None => {
                // New story
                changes.push((id.clone(), ChangeEvent::Created, curr_story.clone()));
            }
            Some(prev_story) => {
                if prev_story.column != curr_story.column {
                    // Story moved to different column
                    changes.push((id.clone(), ChangeEvent::Moved, curr_story.clone()));
                } else if prev_story.key.severity != curr_story.key.severity
                    || prev_story.key.slug != curr_story.key.slug
                    || prev_story.content != curr_story.content
                {
                    // Story edited (severity, slug, or content changed)
                    changes.push((id.clone(), ChangeEvent::Edited, curr_story.clone()));
                }
            }
        }
    }

    // Check for deleted stories
    for (id, prev_story) in prev {
        if !current.contains_key(id) {
            changes.push((id.clone(), ChangeEvent::Deleted, prev_story.clone()));
        }
    }

    // Sort by story ID for consistent output
    changes.sort_by(|a, b| a.0.cmp(&b.0));

    changes
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_story_filename() {
        let key = parse_story_filename("rxj8y-med-report.md").unwrap();
        assert_eq!(key.id, "rxj8y");
        assert_eq!(key.severity, "med");
        assert_eq!(key.slug, "report");
    }

    #[test]
    fn test_parse_story_filename_with_hyphenated_slug() {
        let key = parse_story_filename("abc123-high-fix-login-bug.md").unwrap();
        assert_eq!(key.id, "abc123");
        assert_eq!(key.severity, "high");
        assert_eq!(key.slug, "fix-login-bug");
    }

    #[test]
    fn test_escape_csv() {
        assert_eq!(escape_csv("simple"), "simple");
        assert_eq!(escape_csv("with,comma"), "\"with,comma\"");
        assert_eq!(escape_csv("with\"quote"), "\"with\"\"quote\"");
        assert_eq!(escape_csv("with\nnewline"), "\"with\nnewline\"");
    }

    #[test]
    fn test_detect_changes_created() {
        let prev = HashMap::new();
        let mut current = HashMap::new();

        let story = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "high".to_string(),
                slug: "test".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        current.insert("abc123".to_string(), story);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].1, ChangeEvent::Created));
    }

    #[test]
    fn test_detect_changes_moved() {
        let mut prev = HashMap::new();
        let mut current = HashMap::new();

        let story_prev = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "high".to_string(),
                slug: "test".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        let mut story_curr = story_prev.clone();
        story_curr.column = "doing".to_string();

        prev.insert("abc123".to_string(), story_prev);
        current.insert("abc123".to_string(), story_curr);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].1, ChangeEvent::Moved));
    }

    #[test]
    fn test_detect_changes_edited() {
        let mut prev = HashMap::new();
        let mut current = HashMap::new();

        let story_prev = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "high".to_string(),
                slug: "test".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        let mut story_curr = story_prev.clone();
        story_curr.content = "Updated content".to_string();

        prev.insert("abc123".to_string(), story_prev);
        current.insert("abc123".to_string(), story_curr);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].1, ChangeEvent::Edited));
    }

    #[test]
    fn test_detect_changes_deleted() {
        let mut prev = HashMap::new();
        let current = HashMap::new();

        let story = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "high".to_string(),
                slug: "test".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        prev.insert("abc123".to_string(), story);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].1, ChangeEvent::Deleted));
    }

    #[test]
    fn test_change_event_as_str() {
        assert_eq!(ChangeEvent::Created.as_str(), "created");
        assert_eq!(ChangeEvent::Moved.as_str(), "moved");
        assert_eq!(ChangeEvent::Edited.as_str(), "edited");
        assert_eq!(ChangeEvent::Deleted.as_str(), "deleted");
        assert_eq!(ChangeEvent::CodeCommit.as_str(), "code_commit");
    }

    #[test]
    fn test_parse_story_filename_invalid() {
        // Too few parts
        assert!(parse_story_filename("invalid.md").is_none());
        assert!(parse_story_filename("only-two.md").is_none());
        // No .md extension
        assert!(parse_story_filename("abc-med-slug").is_none());
    }

    #[test]
    fn test_detect_changes_severity_edit() {
        let mut prev = HashMap::new();
        let mut current = HashMap::new();

        let story_prev = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "med".to_string(),
                slug: "test".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        let mut story_curr = story_prev.clone();
        story_curr.key.severity = "high".to_string();

        prev.insert("abc123".to_string(), story_prev);
        current.insert("abc123".to_string(), story_curr);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].1, ChangeEvent::Edited));
    }

    #[test]
    fn test_detect_changes_slug_edit() {
        let mut prev = HashMap::new();
        let mut current = HashMap::new();

        let story_prev = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "med".to_string(),
                slug: "old_slug".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        let mut story_curr = story_prev.clone();
        story_curr.key.slug = "new_slug".to_string();

        prev.insert("abc123".to_string(), story_prev);
        current.insert("abc123".to_string(), story_curr);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0].1, ChangeEvent::Edited));
    }

    #[test]
    fn test_detect_changes_no_change() {
        let mut prev = HashMap::new();
        let mut current = HashMap::new();

        let story = StoryState {
            key: StoryKey {
                id: "abc123".to_string(),
                severity: "med".to_string(),
                slug: "test".to_string(),
            },
            column: "ready".to_string(),
            content: "Test content".to_string(),
        };

        prev.insert("abc123".to_string(), story.clone());
        current.insert("abc123".to_string(), story);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 0);
    }

    #[test]
    fn test_detect_changes_multiple() {
        let mut prev = HashMap::new();
        let mut current = HashMap::new();

        // Story 1: will be deleted
        let story1 = StoryState {
            key: StoryKey {
                id: "aaa111".to_string(),
                severity: "low".to_string(),
                slug: "deleted".to_string(),
            },
            column: "ready".to_string(),
            content: "Will be deleted".to_string(),
        };
        prev.insert("aaa111".to_string(), story1);

        // Story 2: will be moved
        let story2_prev = StoryState {
            key: StoryKey {
                id: "bbb222".to_string(),
                severity: "med".to_string(),
                slug: "moved".to_string(),
            },
            column: "ready".to_string(),
            content: "Will be moved".to_string(),
        };
        let mut story2_curr = story2_prev.clone();
        story2_curr.column = "doing".to_string();
        prev.insert("bbb222".to_string(), story2_prev);
        current.insert("bbb222".to_string(), story2_curr);

        // Story 3: will be created
        let story3 = StoryState {
            key: StoryKey {
                id: "ccc333".to_string(),
                severity: "high".to_string(),
                slug: "created".to_string(),
            },
            column: "ready".to_string(),
            content: "Newly created".to_string(),
        };
        current.insert("ccc333".to_string(), story3);

        let changes = detect_changes(&prev, &current);
        assert_eq!(changes.len(), 3);

        // Changes are sorted by ID
        assert!(matches!(changes[0].1, ChangeEvent::Deleted)); // aaa111
        assert!(matches!(changes[1].1, ChangeEvent::Moved)); // bbb222
        assert!(matches!(changes[2].1, ChangeEvent::Created)); // ccc333
    }
}
