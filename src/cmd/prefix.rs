use crate::issue::extract_issue_id;
use anyhow::Result;

/// Check if a message has an issue prefix, output the ID if found.
/// Exit code 0 if prefix found, 1 if not.
pub fn check(message: &str) -> Result<()> {
    match extract_issue_id(message) {
        Some(id) => {
            println!("{}", id);
            Ok(())
        }
        None => {
            std::process::exit(1);
        }
    }
}
