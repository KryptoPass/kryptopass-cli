pub mod generate;
pub mod get;

use clap::{Parser, Subcommand};

use crate::errors::Result;

#[derive(Parser)]
#[command(name = "KryptoPass")]
#[command(bin_name = "kpass")]
#[command(version)]
#[command(about = "Your digital fortress of secrets guarded by cryptographic wizardry! 🌟🔐")]
#[command(
    long_about = "KryptoPass is a powerful and easy-to-use CLI tool that helps you generate and manage secure passwords, encode text, and configure cryptographic settings effortlessly."
)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(subcommand)]
    Generate(generate::GenerateCommands),
    #[command(subcommand)]
    Get(get::GetCommands),
}

impl Handle for Commands {
    fn handle(self, state: &mut AppState) -> Result<()> {
        match self {
            Commands::Generate(subcmd) => subcmd.handle(state),
            Commands::Get(subcmd) => subcmd.handle(state),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        AppState {}
    }
}

pub trait Handle {
    fn handle(self, state: &mut AppState) -> Result<()>;
}
