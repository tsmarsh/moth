use crate::config::Config;
use std::fs;
use std::process::Command;

pub fn run_hooks(command: &str, hook_type: &str) -> Result<(), String> {
    let config = Config::load().map_err(|e| e.to_string())?;
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
