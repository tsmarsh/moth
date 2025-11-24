use crate::config::Config;
use crate::store::Store;
use anyhow::Result;

pub fn run(id: &str) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(config)?;

    let issue = store.find(id)?;
    store.delete_issue(&issue)?;

    println!("Deleted {}: {}", issue.id, issue.title());

    Ok(())
}
