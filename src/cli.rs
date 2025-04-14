use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "A simple password manager", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(long, short, group = "mode")]
    pub interactive: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Commands for managing passwords
#[derive(Subcommand)]
pub enum Commands {
    /// Add a new password entry
    Add {
        /// Service
        service: String,

        /// Username or email for the service
        username: String,

        /// Password to store
        password: String,
    },
    /// Retrieve a stored password
    Get {
        // Service name to look up
        service: String,
    },
    /// Display all stored services
    List,
}
