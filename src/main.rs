// Copyright (c) 2024 Rodrigo Aroeira
// SPDX-License-Identifier: MIT

mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let mut db = storage::PasswordDatabase::new("passwords.json");

    match args.command {
        Commands::Add {
            service,
            username,
            password,
        } => {
            db.add(models::PasswordEntry {
                service,
                username,
                password,
            });
            if db.save().is_ok() {
                println!("Password saved!");
            }
        }
        Commands::Get { service } => {
            if let Some(entry) = db.get(&service) {
                println!("Username: {}", entry.username);
                println!("Password: {}", entry.password);
            } else {
                println!("Service not found.");
            }
        }
        Commands::List => {
            if db.entries().is_empty() {
                return Err("There are no entries in the database".into());
            }
            println!("Saved services:");
            for entry in db.entries() {
                println!("- {}", entry.service);
            }
        }
    }
    Ok(())
}
