//! `soma config` command

use crate::cli::args::{ConfigAction, ConfigArgs, ConfigFormat};
use crate::config::loader::{ensure_config_dir, get_config_path};
use crate::config::Config;
use anyhow::Result;

pub fn run(args: ConfigArgs, config: &Config) -> Result<()> {
    match args.action {
        ConfigAction::Show { format } => show_config(config, format),
        ConfigAction::Edit => edit_config(),
        ConfigAction::Reset { force } => reset_config(force),
        ConfigAction::Get { key } => get_config(config, &key),
        ConfigAction::Set { key, value } => set_config(config, &key, &value),
    }
}

fn show_config(config: &Config, format: ConfigFormat) -> Result<()> {
    let output = match format {
        ConfigFormat::Toml => toml::to_string_pretty(config)?,
        ConfigFormat::Json => serde_json::to_string_pretty(config)?,
        ConfigFormat::Yaml => serde_yaml::to_string(config)?,
    };
    println!("{}", output);
    Ok(())
}

fn edit_config() -> Result<()> {
    let path = ensure_config_dir()?;
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
        #[cfg(windows)]
        {
            "notepad".to_string()
        }
        #[cfg(not(windows))]
        {
            "vi".to_string()
        }
    });

    std::process::Command::new(&editor).arg(&path).status()?;

    println!("Edited config at {}", path.display());
    Ok(())
}

fn reset_config(force: bool) -> Result<()> {
    if !force {
        print!("Reset config to defaults? [y/N] ");
        use std::io::{stdout, Write};
        stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled");
            return Ok(());
        }
    }

    let path = ensure_config_dir()?;
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    println!("Config reset. Run `soma config show` to see defaults.");
    Ok(())
}

fn get_config(config: &Config, key: &str) -> Result<()> {
    // Simple dot-notation access
    let value = match key {
        "model.name" => config.model.name.clone(),
        "model.auto_download" => config.model.auto_download.to_string(),
        "model.context_size" => config.model.context_size.to_string(),
        "model.threads" => config.model.threads.to_string(),
        "model.gpu_layers" => config.model.gpu_layers.to_string(),
        "shell.default" => format!("{:?}", config.shell.default).to_lowercase(),
        "shell.keybind" => config.shell.keybind.clone(),
        "shell.alias" => config.shell.alias.clone(),
        "prompt.system" => format!("{:?}", config.prompt.system).to_lowercase(),
        "prompt.max_tokens" => config.prompt.max_tokens.to_string(),
        "prompt.temperature" => config.prompt.temperature.to_string(),
        "output.insert_mode" => format!("{:?}", config.output.insert_mode).to_lowercase(),
        _ => anyhow::bail!("Unknown config key: {}", key),
    };
    println!("{}", value);
    Ok(())
}

fn set_config(config: &Config, key: &str, value: &str) -> Result<()> {
    let mut config = config.clone();

    match key {
        "model.name" => config.model.name = value.to_string(),
        "model.auto_download" => config.model.auto_download = value.parse()?,
        "model.context_size" => config.model.context_size = value.parse()?,
        "model.threads" => config.model.threads = value.parse()?,
        "model.gpu_layers" => config.model.gpu_layers = value.parse()?,
        "shell.keybind" => config.shell.keybind = value.to_string(),
        "shell.alias" => config.shell.alias = value.to_string(),
        "prompt.max_tokens" => config.prompt.max_tokens = value.parse()?,
        "prompt.temperature" => config.prompt.temperature = value.parse()?,
        _ => anyhow::bail!("Unknown or read-only config key: {}", key),
    }

    let path = ensure_config_dir()?;
    config.save(&path)?;
    println!("Set {} = {}", key, value);
    Ok(())
}
