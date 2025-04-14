use crate::models::PasswordEntry;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct PasswordDatabase {
    #[serde(skip)]
    pub path: String,
    entries: Vec<PasswordEntry>,
}

impl PasswordDatabase {
    pub fn new(path: &str) -> Self {
        let entries = match fs::read_to_string(path) {
            Ok(data) => match serde_json::from_str::<PasswordDatabase>(&data) {
                Ok(db) => db.entries,
                Err(_) => serde_json::from_str(&data).unwrap_or(vec![]),
            },
            Err(_) => vec![],
        };

        Self {
            path: path.to_string(),
            entries,
        }
    }

    pub fn add(&mut self, entry: PasswordEntry) {
        self.entries.push(entry);
    }

    pub fn get(&self, service: &str) -> Option<&PasswordEntry> {
        self.entries.iter().find(|&e| e.service == service)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let contents = serde_json::to_string_pretty(&self)?;
        fs::write(&self.path, &contents)?;
        Ok(())
    }

    pub fn entries(&self) -> &[PasswordEntry] {
        &self.entries
    }
}
