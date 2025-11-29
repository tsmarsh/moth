use crate::config::Config;

#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::process::Command;

/// Run lifecycle hooks for a command.
/// Hooks are shell scripts stored in .moth/hooks/{command}/{before|after}/
/// This only works on Unix systems - on Windows, hooks are silently skipped.
pub fn run_hooks(command: &str, hook_type: &str) -> Result<(), String> {
    #[cfg(unix)]
    {
        run_hooks_unix(command, hook_type)
    }
    #[cfg(not(unix))]
    {
        let _ = (command, hook_type);
        Ok(())
    }
}

#[cfg(unix)]
fn run_hooks_unix(command: &str, hook_type: &str) -> Result<(), String> {
    match Config::load() {
        Ok(config) => {
            let hooks_dir = config.moth_dir.join("hooks");
            let command_hooks_dir = hooks_dir.join(command).join(hook_type);

            if !command_hooks_dir.is_dir() {
                return Ok(());
            }

            let entries = fs::read_dir(&command_hooks_dir)
                .map_err(|e| format!("Failed to read hooks directory: {}", e))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("Failed to read hook entry: {}", e))?;
                let path = entry.path();
                if path.is_file() {
                    let mut cmd = Command::new("sh");
                    cmd.arg(&path);
                    let status = cmd
                        .status()
                        .map_err(|e| format!("Failed to execute hook {:?}: {}", path, e))?;

                    if !status.success() {
                        return Err(format!(
                            "Hook script {:?} failed with status {}",
                            path, status
                        ));
                    }
                }
            }

            Ok(())
        }
        Err(_e) => Ok(()), //ignore if we're not in a moth dir
    }
}
