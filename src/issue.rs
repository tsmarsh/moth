use anyhow::{Result, anyhow};
use rand::Rng;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Crit,
    High,
    Med,
    Low,
}

impl FromStr for Severity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "crit" => Ok(Severity::Crit),
            "high" => Ok(Severity::High),
            "med" => Ok(Severity::Med),
            "low" => Ok(Severity::Low),
            _ => Err(anyhow!(
                "Invalid severity: {}. Must be one of: crit, high, med, low",
                s
            )),
        }
    }
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Crit => "crit",
            Severity::High => "high",
            Severity::Med => "med",
            Severity::Low => "low",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub severity: Severity,
    pub slug: String,
    pub status: String,
    pub path: PathBuf,
    pub order: Option<u32>,
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
                "Invalid filename format. Expected: [{{order}}-]{{id}}-{{severity}}-{{slug}}.md"
            ));
        }

        // Try to parse first part as order number
        let (order, id_idx) = if parts[0].parse::<u32>().is_ok() {
            (Some(parts[0].parse::<u32>().unwrap()), 1)
        } else {
            (None, 0)
        };

        if parts.len() < id_idx + 3 {
            return Err(anyhow!(
                "Invalid filename format. Expected: [{{order}}-]{{id}}-{{severity}}-{{slug}}.md"
            ));
        }

        let id = parts[id_idx].to_string();
        let severity = parts[id_idx + 1].parse()?;

        // Join remaining parts with underscores (new format) or hyphens (backward compat)
        let slug_parts = &parts[id_idx + 2..];
        let slug = slug_parts.join("_");

        Ok(Issue {
            id,
            severity,
            slug,
            status: status.to_string(),
            path: path.to_path_buf(),
            order,
        })
    }

    pub fn filename(&self) -> String {
        if let Some(order) = self.order {
            format!(
                "{:03}-{}-{}-{}.md",
                order,
                self.id,
                self.severity.as_str(),
                self.slug
            )
        } else {
            format!("{}-{}-{}.md", self.id, self.severity.as_str(), self.slug)
        }
    }

    pub fn title(&self) -> String {
        // Support both underscore (new) and hyphen (old) separators
        let separator = if self.slug.contains('_') { '_' } else { '-' };
        self.slug
            .split(separator)
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
    fn test_severity_from_str() {
        assert_eq!("crit".parse::<Severity>().unwrap(), Severity::Crit);
        assert_eq!("high".parse::<Severity>().unwrap(), Severity::High);
        assert_eq!("med".parse::<Severity>().unwrap(), Severity::Med);
        assert_eq!("low".parse::<Severity>().unwrap(), Severity::Low);
        assert!("invalid".parse::<Severity>().is_err());
    }

    #[test]
    fn test_issue_title() {
        let issue = Issue {
            id: "abc123".to_string(),
            severity: Severity::High,
            slug: "fix_login_bug".to_string(),
            status: "ready".to_string(),
            path: PathBuf::from("/test/abc123-high-fix_login_bug.md"),
            order: None,
        };
        assert_eq!(issue.title(), "Fix Login Bug");
    }

    #[test]
    fn test_issue_filename() {
        let issue = Issue {
            id: "x7k2m".to_string(),
            severity: Severity::High,
            slug: "fix_login_bug".to_string(),
            status: "ready".to_string(),
            path: PathBuf::from("/test/x7k2m-high-fix_login_bug.md"),
            order: None,
        };
        assert_eq!(issue.filename(), "x7k2m-high-fix_login_bug.md");
    }

    #[test]
    fn test_issue_filename_with_order() {
        let issue = Issue {
            id: "x7k2m".to_string(),
            severity: Severity::High,
            slug: "fix_login_bug".to_string(),
            status: "ready".to_string(),
            path: PathBuf::from("/test/001-x7k2m-high-fix_login_bug.md"),
            order: Some(1),
        };
        assert_eq!(issue.filename(), "001-x7k2m-high-fix_login_bug.md");
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
