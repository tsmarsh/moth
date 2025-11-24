use anyhow::{Result, anyhow};
use rand::Rng;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Crit,
    High,
    Med,
    Low,
}

impl FromStr for Priority {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "crit" => Ok(Priority::Crit),
            "high" => Ok(Priority::High),
            "med" => Ok(Priority::Med),
            "low" => Ok(Priority::Low),
            _ => Err(anyhow!(
                "Invalid priority: {}. Must be one of: crit, high, med, low",
                s
            )),
        }
    }
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Crit => "crit",
            Priority::High => "high",
            Priority::Med => "med",
            Priority::Low => "low",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub priority: Priority,
    pub slug: String,
    pub status: String,
    pub path: PathBuf,
}

impl Issue {
    pub fn from_path(path: &Path, status: &str) -> Result<Self> {
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid filename"))?;

        let parts: Vec<&str> = filename.split('-').collect();
        if parts.len() < 3 {
            return Err(anyhow!(
                "Invalid filename format. Expected: {{id}}-{{priority}}-{{slug}}.md"
            ));
        }

        let id = parts[0].to_string();
        let priority = parts[1].parse()?;
        let slug = parts[2..].join("-");

        Ok(Issue {
            id,
            priority,
            slug,
            status: status.to_string(),
            path: path.to_path_buf(),
        })
    }

    pub fn filename(&self) -> String {
        format!("{}-{}-{}.md", self.id, self.priority.as_str(), self.slug)
    }

    pub fn title(&self) -> String {
        self.slug
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub fn generate_id(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_from_str() {
        assert_eq!("crit".parse::<Priority>().unwrap(), Priority::Crit);
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("med".parse::<Priority>().unwrap(), Priority::Med);
        assert_eq!("low".parse::<Priority>().unwrap(), Priority::Low);
        assert!("invalid".parse::<Priority>().is_err());
    }

    #[test]
    fn test_issue_title() {
        let issue = Issue {
            id: "abc123".to_string(),
            priority: Priority::High,
            slug: "fix-login-bug".to_string(),
            status: "ready".to_string(),
            path: PathBuf::from("/test/abc123-high-fix-login-bug.md"),
        };
        assert_eq!(issue.title(), "Fix Login Bug");
    }

    #[test]
    fn test_issue_filename() {
        let issue = Issue {
            id: "x7k2m".to_string(),
            priority: Priority::High,
            slug: "fix-login-bug".to_string(),
            status: "ready".to_string(),
            path: PathBuf::from("/test/x7k2m-high-fix-login-bug.md"),
        };
        assert_eq!(issue.filename(), "x7k2m-high-fix-login-bug.md");
    }

    #[test]
    fn test_generate_id() {
        let id = generate_id(5);
        assert_eq!(id.len(), 5);
        assert!(
            id.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        );
    }
}
