//! Soma - Local AI Terminal Command Assistant
//! Single binary, zero-config, works offline after first model download.

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

mod cli;
mod commands;
mod config;
mod inference;
mod model;
mod output;
mod shell;
mod utils;

use cli::Cli;
use commands::{
    ask, completions, config as config_cmd, insert, install, model as model_cmd, skills,
};
use config::Config;
use utils::logging::init_logging;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging early
    init_logging(cli.verbose, cli.quiet)?;

    // Load configuration (layered: defaults < file < env < cli)
    let mut config = Config::load(&cli.config)?;

    // CLI args override config
    config.apply_cli_overrides(&cli);

    tracing::debug!("Configuration loaded: {:?}", config);

    // Dispatch to subcommand
    let result = match cli.command {
        cli::Commands::Ask(args) => ask::run(args, &config),
        cli::Commands::Insert(args) => insert::run(args, &config),
        cli::Commands::Install(args) => install::run(args, &config),
        cli::Commands::Config(args) => config_cmd::run(args, &config),
        cli::Commands::Model(args) => model_cmd::run(args, &config),
        cli::Commands::Skills(args) => skills::run(args, &config),
        cli::Commands::Completions(args) => completions::run(args),
    };

    // Handle results
    if let Err(e) = result {
        if !cli.quiet {
            eprintln!("Error: {}", e);
            for cause in e.chain().skip(1) {
                eprintln!("  Caused by: {}", cause);
            }
        }
        std::process::exit(1);
    }

    Ok(())
}
