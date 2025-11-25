use cucumber::World;
use std::path::PathBuf;
use tempfile::TempDir;

#[derive(Debug, Default, World)]
pub struct MothWorld {
    pub temp_dir: Option<TempDir>,
    pub last_result: Option<Result<(), anyhow::Error>>,
    pub last_issue_id: Option<String>,
}

impl MothWorld {
    pub fn moth_path(&self) -> PathBuf {
        self.temp_dir
            .as_ref()
            .expect("temp_dir not initialized")
            .path()
            .to_path_buf()
    }

    pub fn set_current_dir(&self) {
        std::env::set_current_dir(self.moth_path()).expect("Failed to set current dir");
    }
}
