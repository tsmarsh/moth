use crate::config::Config;
use crate::store::Store;
use anyhow::Result;

pub fn run(id: &str) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let last_status = store.config().last_status();
    let issue = store.find(id)?;
    let target_status = &last_status.name;

    store.move_issue(&issue, target_status)?;

    println!("Moved {} to {}", issue.id, target_status);

    Ok(())
}
