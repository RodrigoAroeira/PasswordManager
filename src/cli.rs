use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about = "A simple password manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

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
