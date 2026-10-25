//! Configuration Schema

use anyhow::Result;
use figment::{Figment, providers::{Format, Toml, Env, Serialized}};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub model: ModelConfig,
    pub shell: ShellConfig,
    pub prompt: PromptConfig,
    pub output: OutputConfig,
    pub paths: PathsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: ModelConfig::default(),
            shell: ShellConfig::default(),
            prompt: PromptConfig::default(),
            output: OutputConfig::default(),
            paths: PathsConfig::default(),
        }
    }
}

impl Config {
    pub fn load(config_path: &Option<PathBuf>) -> Result<Self> {
        let mut figment = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(PathBuf::from("soma.toml")).nested())
            .merge(Env::prefixed("SOMA_").global());

        if let Some(path) = config_path {
            figment = figment.merge(Toml::file(path).nested());
        } else {
            // Default config locations
            figment = figment
                .merge(Toml::file(dirs::config_dir().unwrap_or_default().join("soma/config.toml")).nested())
                .merge(Toml::file(dirs::home_dir().unwrap_or_default().join(".config/soma/config.toml")).nested())
                .merge(Toml::file("./soma.toml").nested());
        }

        Ok(figment.extract()?)
    }

    pub fn apply_cli_overrides(&mut self, cli: &crate::cli::Cli) {
        if let Some(model) = &cli.command.get_model_override() {
            self.model.name = model.clone();
        }
        // Add more CLI overrides as needed
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let toml = toml::to_string_pretty(self)?;
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, toml)?;
        Ok(())
    }
}

trait ModelOverride {
    fn get_model_override(&self) -> Option<&String>;
}

impl ModelOverride for crate::cli::Commands {
    fn get_model_override(&self) -> Option<&String> {
        match self {
            crate::cli::Commands::Ask(args) => args.model.as_ref(),
            crate::cli::Commands::Insert(args) => args.model.as_ref(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub auto_download: bool,
    pub context_size: usize,
    pub threads: usize,
    pub gpu_layers: usize,
    pub cache_dir: PathBuf,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            name: "qwen2.5-coder-1.5b-instruct-q4_k_m.gguf".to_string(),
            auto_download: true,
            context_size: 4096,
            threads: 0, // 0 = auto
            gpu_layers: 0,
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("soma/models"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub default: ShellType,
    pub keybind: String,
    pub alias: String,
    pub install_completions: bool,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            default: ShellType::Auto,
            keybind: "ctrl+space".to_string(),
            alias: "??".to_string(),
            install_completions: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShellType {
    Auto,
    Pwsh,
    Bash,
    Zsh,
    Fish,
    Cmd,
}

impl Default for ShellType {
    fn default() -> Self {
        ShellType::Auto
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    pub system: SystemPromptType,
    pub max_tokens: u32,
    pub temperature: f32,
    pub stop_sequences: Vec<String>,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            system: SystemPromptType::PowerShell,
            max_tokens: 512,
            temperature: 0.1,
            stop_sequences: vec!["\n\n".to_string(), "```".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SystemPromptType {
    PowerShell,
    Bash,
    Generic,
}

impl Default for SystemPromptType {
    fn default() -> Self {
        SystemPromptType::PowerShell
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub insert_mode: InsertMode,
    pub show_confirmation: bool,
    pub copy_to_clipboard: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            insert_mode: InsertMode::NextLine,
            show_confirmation: true,
            copy_to_clipboard: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsertMode {
    NextLine,
    AtCursor,
    ClipboardOnly,
}

impl Default for InsertMode {
    fn default() -> Self {
        InsertMode::NextLine
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl Default for PathsConfig {
    fn default() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("soma");
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("soma");
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("soma");

        Self {
            config_dir,
            cache_dir,
            data_dir,
        }
    }
}