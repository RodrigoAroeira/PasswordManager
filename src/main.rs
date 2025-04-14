// Copyright (c) 2024 Rodrigo Aroeira
// SPDX-License-Identifier: MIT

mod cli;
mod models;
mod storage;
mod utils;

use clap::Parser;
use cli::Cli;
use utils::{cli_parse, interactive_mode};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let path = "passwords.json";

    let mut db = storage::PasswordDatabase::new(path);

    if args.interactive {
        return interactive_mode(&mut db);
    }

    cli_parse(&mut db, args)
}
