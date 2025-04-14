use inquire::{Password, PasswordDisplayMode, Select, Text, validator::Validation};

use crate::{
    cli::{Cli, Commands},
    models::PasswordEntry,
    storage::PasswordDatabase,
};

fn print_entries(db: &PasswordDatabase) -> anyhow::Result<()> {
    if db.entries().is_empty() {
        anyhow::bail!("There are no entries in the database");
    }
    println!("Saved services:");
    for entry in db.entries() {
        println!("- {}", entry.service);
    }
    Ok(())
}

pub fn cli_parse(db: &mut PasswordDatabase, args: Cli) -> anyhow::Result<()> {
    match args.command.unwrap() {
        Commands::Add {
            service,
            username,
            password,
        } => {
            db.add(PasswordEntry {
                service,
                username,
                password,
            });
            db.save()?;
            println!("Password saved!");
        }
        Commands::Get { service } => {
            if let Some(entry) = db.get(&service) {
                println!("Username: {}", entry.username);
                println!("Password: {}", entry.password);
            } else {
                anyhow::bail!("Service not found.");
            }
        }
        Commands::List => print_entries(db)?,
    }

    Ok(())
}

pub fn interactive_mode(db: &mut PasswordDatabase) -> anyhow::Result<()> {
    let help_message = "Press ctrl-c to cancel";
    let opt = Select::new("What do you want to do?", vec!["Add", "Get", "List"])
        .with_help_message(help_message)
        .with_vim_mode(true)
        .prompt()?;

    match opt {
        "Add" => handle_add(db, help_message)?,
        "Get" => handle_get(db)?,
        "List" => print_entries(db)?,
        s => unreachable!("Unexpected option {s:?} selected"),
    }

    Ok(())
}

fn handle_add(db: &mut PasswordDatabase, help_message: &str) -> anyhow::Result<()> {
    let validator = |input: &str| {
        if input.is_empty() {
            Ok(Validation::Invalid("Empty strings are not allowed".into()))
        } else if input.contains(' ') {
            Ok(Validation::Invalid("Spaces are not allowed".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let service = Text::new("Type in the service: ")
        .with_validator(validator)
        .with_help_message(help_message)
        .prompt()?;

    let username = Text::new("Type in the username: ")
        .with_validator(validator)
        .with_help_message(help_message)
        .prompt()?;

    let password = Password::new("Type in your password: ")
        .with_validator(validator)
        .with_help_message(help_message)
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt()?;

    db.add(PasswordEntry {
        service,
        username,
        password,
    });
    db.save()?;
    println!("Passwords successfully saved");

    Ok(())
}

fn handle_get(db: &PasswordDatabase) -> anyhow::Result<()> {
    let service = Text::new("Type in the service: ").prompt()?;

    let prompt = Select::new(
        "Service not found, would you like to search again?",
        vec!["Yes", "No"],
    );

    if let Some(entry) = db.get(&service) {
        println!("Username: {}", entry.username);
        println!("Password: {}", entry.password);
        Ok(())
    } else if prompt.prompt()? == "Yes" {
        handle_get(db)
    } else {
        anyhow::bail!("Service not found.");
    }
}
