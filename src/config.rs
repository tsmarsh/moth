use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusConfig {
    pub name: String,
    pub dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub statuses: Vec<StatusConfig>,
    pub default_priority: String,
    #[serde(default = "default_editor")]
    pub editor: String,
    #[serde(default = "default_id_length")]
    pub id_length: usize,
    #[serde(skip)]
    pub moth_dir: PathBuf,
}

fn default_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| "vi".to_string())
}

fn default_id_length() -> usize {
    5
}

impl Default for Config {
    fn default() -> Self {
        Config {
            statuses: vec![
                StatusConfig {
                    name: "ready".to_string(),
                    dir: "ready".to_string(),
                },
                StatusConfig {
                    name: "doing".to_string(),
                    dir: "doing".to_string(),
                },
                StatusConfig {
                    name: "done".to_string(),
                    dir: "done".to_string(),
                },
            ],
            default_priority: "med".to_string(),
            editor: default_editor(),
            id_length: 5,
            moth_dir: PathBuf::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let moth_dir = Self::find_moth_dir()?;
        let config_path = moth_dir.join("config.yml");

        if !config_path.exists() {
            return Err(anyhow!(
                "Config file not found at {}. Try running 'moth init' first.",
                config_path.display()
            ));
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let mut config: Config = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;

        config.moth_dir = moth_dir;
        config.validate()?;

        Ok(config)
    }

    fn find_moth_dir() -> Result<PathBuf> {
        let mut current = env::current_dir().context("Failed to get current directory")?;

        loop {
            let moth_dir = current.join(".moth");
            if moth_dir.is_dir() {
                return Ok(moth_dir);
            }

            if !current.pop() {
                return Err(anyhow!(
                    "No .moth directory found. Try running 'moth init' first."
                ));
            }
        }
    }

    fn validate(&self) -> Result<()> {
        if self.statuses.len() < 2 {
            return Err(anyhow!(
                "Config must have at least 2 statuses, found {}",
                self.statuses.len()
            ));
        }

        let valid_priorities = ["crit", "high", "med", "low"];
        if !valid_priorities.contains(&self.default_priority.as_str()) {
            return Err(anyhow!(
                "Invalid default_priority: {}. Must be one of: crit, high, med, low",
                self.default_priority
            ));
        }

        if self.id_length < 3 || self.id_length > 10 {
            return Err(anyhow!(
                "id_length must be between 3 and 10, found {}",
                self.id_length
            ));
        }

        Ok(())
    }

    pub fn first_status(&self) -> &StatusConfig {
        &self.statuses[0]
    }

    pub fn second_status(&self) -> Option<&StatusConfig> {
        self.statuses.get(1)
    }

    pub fn last_status(&self) -> &StatusConfig {
        &self.statuses[self.statuses.len() - 1]
    }

    pub fn get_status(&self, name: &str) -> Option<&StatusConfig> {
        self.statuses.iter().find(|s| s.name == name)
    }

    pub fn status_dir(&self, status: &StatusConfig) -> PathBuf {
        self.moth_dir.join(&status.dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.statuses.len(), 3);
        assert_eq!(config.default_priority, "med");
        assert_eq!(config.id_length, 5);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        assert!(config.validate().is_ok());

        config.statuses = vec![];
        assert!(config.validate().is_err());

        config = Config::default();
        config.default_priority = "invalid".to_string();
        assert!(config.validate().is_err());

        config = Config::default();
        config.id_length = 2;
        assert!(config.validate().is_err());

        config.id_length = 11;
        assert!(config.validate().is_err());
    }
}
