use std::sync::{Arc, Mutex};

use clap::{Args, Subcommand};
use kryptopass_utils::getpass::getpass_masked;

use super::{AppState, Handle};
use crate::errors::Result;

#[derive(Subcommand, Debug)]
pub enum GetCommands {
    Password(GetArgs),
    Passphrase(GetArgs),
    Token(GetArgs),
}

impl Handle for GetCommands {
    fn handle(self, _: Arc<Mutex<AppState>>) -> Result<()> {
        match self {
            GetCommands::Password(_) => {
                let _password = getpass_masked("Enter password: ", || '*').unwrap();
            }
            GetCommands::Passphrase(args) => args.handle("passphrase"),
            GetCommands::Token(args) => args.handle("token"),
        };

        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct GetArgs {
    #[arg(value_name = "TITLE")]
    title: String,
    #[arg(long)]
    reveal: bool,
    #[arg(long)]
    copy: bool,
}

impl GetArgs {
    pub fn handle(&self, entry_type: &str) {
        println!("🔍 Retrieving {}: {}", entry_type, self.title);

        if self.copy {
            println!("🔒 Copying {} to clipboard...", self.title);
        } else if self.reveal {
            println!("🔓 Revealing {}...", self.title);
        } else {
            println!("ℹ️ No action specified. Use --reveal or --copy.");
        }
    }
}
