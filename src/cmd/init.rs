use crate::config::Config;
use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs;

pub fn run() -> Result<()> {
    let cwd = env::current_dir().context("Failed to get current directory")?;
    let moth_dir = cwd.join(".moth");

    if moth_dir.exists() {
        return Err(anyhow!(
            "Moth already initialized in {}",
            moth_dir.display()
        ));
    }

    fs::create_dir(&moth_dir)
        .with_context(|| format!("Failed to create directory: {}", moth_dir.display()))?;

    let config = Config::default();
    let config_path = moth_dir.join("config.yml");
    let yaml = serde_yaml::to_string(&config).context("Failed to serialize config")?;
    fs::write(&config_path, yaml)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    for status in &config.statuses {
        let status_dir = moth_dir.join(&status.dir);
        fs::create_dir(&status_dir).with_context(|| {
            format!("Failed to create status directory: {}", status_dir.display())
        })?;
    }

    println!("Initialized moth in {}", moth_dir.display());

    Ok(())
}
