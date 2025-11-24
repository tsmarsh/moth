use crate::config::Config;
use crate::store::Store;
use anyhow::{anyhow, Result};

pub fn run(id: &str, target_status: &str) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    if store.config().get_status(target_status).is_none() {
        return Err(anyhow!("Unknown status: {}", target_status));
    }

    let issue = store.find(id)?;
    store.move_issue(&issue, target_status)?;

    println!("Moved {} to {}", issue.id, target_status);

    Ok(())
}
