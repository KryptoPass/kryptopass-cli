use std::sync::{Arc, Mutex};

use clap::Subcommand;

use crate::errors::Result;

use super::{AppState, Handle};

#[derive(Subcommand, Debug)]
pub enum GenerateCommands {
    Password,
    Passphrase,
    Token,
}

impl Handle for GenerateCommands {
    fn handle(self, _: Arc<Mutex<AppState>>) -> Result<()> {
        match self {
            GenerateCommands::Password => {
                println!("🔑 Generating secure password...");
            }
            GenerateCommands::Passphrase => {
                println!("🔐 Creating passphrase...");
            }
            GenerateCommands::Token => {
                println!("🎫 Generating secure token...");
            }
        }

        Ok(())
    }
}
