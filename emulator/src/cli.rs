use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::commands;

#[derive(Parser)]
#[command(version, about = "Chip-8 emulator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute ROM
    Run {
        /// Path to the ROM file
        path: PathBuf,

        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Generate default configuration file
    GenerateConfig {
        /// Path to store the configuration in.
        path: PathBuf,
    },
}

/// Cli entrypoint.
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path, config } => commands::run_rom(&path, config),
        Commands::GenerateConfig { path } => commands::generate_default_config(&path),
    }
}
