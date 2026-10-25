//! `soma install` command

use crate::cli::args::{InstallArgs, ShellType};
use crate::config::Config;
use crate::shell::detector::DetectedShell;
use crate::shell::installer::{install, uninstall};
use anyhow::Result;

pub fn run(args: InstallArgs, config: &Config) -> Result<()> {
    // Resolve shell
    let shell = match args.shell {
        ShellType::Auto => DetectedShell::detect()?,
        ShellType::Pwsh => DetectedShell::Pwsh,
        ShellType::Bash => DetectedShell::Bash,
        ShellType::Zsh => DetectedShell::Zsh,
        ShellType::Fish => DetectedShell::Fish,
        ShellType::Cmd => DetectedShell::Cmd,
    };

    println!("Installing for shell: {}", shell.config_name());

    if args.uninstall {
        uninstall(&shell, args.dry_run)?;
    } else {
        install(&shell, config, args.force, args.dry_run)?;
    }

    if !args.dry_run {
        println!("\nRestart your shell or run:");
        match shell {
            DetectedShell::Pwsh => println!("  . $PROFILE"),
            DetectedShell::Bash => println!("  source ~/.bashrc"),
            DetectedShell::Zsh => println!("  source ~/.zshrc"),
            DetectedShell::Fish => println!("  source ~/.config/fish/config.fish"),
            _ => println!("  Restart your terminal"),
        }
    }

    Ok(())
}
