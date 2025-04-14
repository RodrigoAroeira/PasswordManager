use crate::models::PasswordEntry;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct PasswordDatabase {
    #[serde(skip)]
    pub path: String,
    entries: Vec<PasswordEntry>,
}

impl PasswordDatabase {
    pub fn new(path: &str) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|data| serde_json::from_str::<PasswordDatabase>(&data).ok())
            .unwrap_or(Self {
                path: path.to_string(),
                entries: vec![],
            })
    }

    pub fn add(&mut self, entry: PasswordEntry) {
        self.entries.push(entry);
    }

    pub fn get(&self, service: &str) -> Option<&PasswordEntry> {
        self.entries.iter().find(|&e| e.service == service)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Path::new(&self.path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let contents = serde_json::to_string_pretty(&self)?;
        fs::write(&self.path, &contents)?;
        Ok(())
    }

    pub fn entries(&self) -> &[PasswordEntry] {
        &self.entries
    }
}
