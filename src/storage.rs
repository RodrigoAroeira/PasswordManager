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

#[cfg(test)]
mod tests {

    use tempfile::NamedTempFile;

    use super::*;

    fn get_tmp_file() -> anyhow::Result<(NamedTempFile, String)> {
        let file = NamedTempFile::new()?;
        let path = file.path().to_str().unwrap().to_string();
        Ok((file, path))
    }

    #[test]
    fn test_add_and_get_entry() {
        let (_, path) = get_tmp_file().unwrap();
        let mut db = PasswordDatabase::new(&path);

        let entry = PasswordEntry {
            service: String::from("service"),
            username: String::from("username"),
            password: String::from("password"),
        };

        let service = entry.service.clone();
        db.add(entry);
        assert!(
            db.get(&service).is_some(),
            "Entry was not found in the database"
        );
    }

    #[test]
    fn test_save() {
        let (_, path) = get_tmp_file().expect("Unable to create tempfile");
        let mut db = PasswordDatabase::new(&path);

        let entry = PasswordEntry {
            service: String::from("service"),
            username: String::from("username"),
            password: String::from("password"),
        };
        let service = entry.service.clone();
        db.add(entry);
        assert!(db.save().is_ok());
        let db2 = PasswordDatabase::new(&path);
        assert_eq!(db.get(&service), db2.get(&service));
    }

    #[test]
    fn test_non_existent_service() {
        let (_, path) = get_tmp_file().unwrap();
        let db = PasswordDatabase::new(&path);

        assert!(db.get("Service").is_none(), "Database should be empty");
    }

    #[test]
    fn test_empty_database() {
        let (_, path) = get_tmp_file().unwrap();
        let db = PasswordDatabase::new(&path);

        assert!(db.entries().is_empty(), "Database should be empty");
    }

    #[test]
    fn test_save_and_load_empty_database() {
        let (_, path) = get_tmp_file().unwrap();
        let db = PasswordDatabase::new(&path);

        assert!(db.save().is_ok(), "Saving empty database failed");

        let loaded_db = PasswordDatabase::new(&path);
        assert!(
            loaded_db.entries().is_empty(),
            "Loaded database should be empty"
        );
    }
}
