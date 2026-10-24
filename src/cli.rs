mod subcommand;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Deserialize file
    Deserialize {
        /// File to deserialize
        file: PathBuf,
    },
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Commands::Deserialize { file } => subcommand::deserialize(&file)?,
    }
    Ok(())
}
