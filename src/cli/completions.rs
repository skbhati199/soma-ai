//! Shell Completions Generation

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;
use std::path::PathBuf;

use crate::cli::Cli;

pub fn run(args: super::args::CompletionsArgs) -> Result<()> {
    let mut cmd = Cli::command();
    let shell: Shell = args.shell.into();
    let name = "soma";

    match args.output {
        Some(path) => {
            let mut file = std::fs::File::create(&path)?;
            generate(shell, &mut cmd, name, &mut file);
            eprintln!("Completions written to {}", path.display());
        }
        None => {
            let mut stdout = io::stdout();
            generate(shell, &mut cmd, name, &mut stdout);
        }
    }
    Ok(())
}

impl From<super::args::ShellType> for Shell {
    fn from(s: super::args::ShellType) -> Self {
        match s {
            super::args::ShellType::Bash => Shell::Bash,
            super::args::ShellType::Zsh => Shell::Zsh,
            super::args::ShellType::Fish => Shell::Fish,
            super::args::ShellType::Pwsh => Shell::PowerShell,
            super::args::ShellType::Cmd => Shell::Cmd,
            super::args::ShellType::Auto => Shell::Bash, // fallback
        }
    }
}