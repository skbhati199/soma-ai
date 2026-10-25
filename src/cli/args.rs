//! CLI Argument Definitions

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "soma",
    version,
    about = "Local AI terminal command assistant",
    long_about = "Soma provides AI-powered command suggestions for your terminal.\n\
                  Uses a local LLM (GGUF) - works offline after first model download.\n\
                  \n\
                  Examples:\n\
                    soma ask \"list all txt files\"\n\
                    soma insert \"disk usage\"\n\
                    soma install --shell pwsh\n\
                    soma model pull qwen2.5-coder-1.5b",
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get AI command suggestion (prints to stdout, copies to clipboard)
    Ask(AskArgs),

    /// Insert AI command on next line (for keybindings like Ctrl+Space)
    Insert(InsertArgs),

    /// Install shell integration (keybindings, aliases, completions)
    Install(InstallArgs),

    /// Manage configuration
    Config(ConfigArgs),

    /// Manage models (pull, list, remove)
    Model(ModelArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),
}

#[derive(Parser, Debug)]
pub struct AskArgs {
    /// Natural language query
    #[arg(required = true, value_name = "QUERY")]
    pub query: String,

    /// Shell type for prompt (auto-detect if not specified)
    #[arg(short, long, value_enum)]
    pub shell: Option<ShellType>,

    /// Model to use (overrides config)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Temperature (0.0-1.0)
    #[arg(long, value_parser = clap::value_parser!(f32).range(0.0..=1.0))]
    pub temperature: Option<f32>,

    /// Max tokens to generate
    #[arg(long)]
    pub max_tokens: Option<u32>,

    /// Don't copy to clipboard
    #[arg(long)]
    pub no_clipboard: bool,

    /// Raw output (no formatting)
    #[arg(long)]
    pub raw: bool,
}

#[derive(Parser, Debug)]
pub struct InsertArgs {
    /// Natural language query
    #[arg(required = true, value_name = "QUERY")]
    pub query: String,

    /// Shell type for prompt (auto-detect if not specified)
    #[arg(short, long, value_enum)]
    pub shell: Option<ShellType>,

    /// Model to use (overrides config)
    #[arg(short, long)]
    pub model: Option<String>,

    /// Temperature (0.0-1.0)
    #[arg(long, value_parser = clap::value_parser!(f32).range(0.0..=1.0))]
    pub temperature: Option<f32>,

    /// Max tokens to generate
    #[arg(long)]
    pub max_tokens: Option<u32>,
}

#[derive(Parser, Debug)]
pub struct InstallArgs {
    /// Target shell
    #[arg(short, long, value_enum, default_value = "auto")]
    pub shell: ShellType,

    /// Keybinding for insert (e.g., ctrl+space, alt+enter)
    #[arg(long, default_value = "ctrl+space")]
    pub keybind: String,

    /// Also create alias (??) for ask command
    #[arg(long, default_value = "true")]
    pub alias: bool,

    /// Force overwrite existing installation
    #[arg(long)]
    pub force: bool,

    /// Dry run - show what would be done
    #[arg(long)]
    pub dry_run: bool,

    /// Uninstall instead of install
    #[arg(long)]
    pub uninstall: bool,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show current configuration
    Show {
        /// Output format
        #[arg(short, long, value_enum, default_value = "toml")]
        format: ConfigFormat,
    },
    /// Edit configuration in $EDITOR
    Edit,
    /// Reset to defaults
    Reset {
        #[arg(long)]
        force: bool,
    },
    /// Get a specific value
    Get { key: String },
    /// Set a specific value
    Set { key: String, value: String },
}

#[derive(Parser, Debug)]
pub struct ModelArgs {
    #[command(subcommand)]
    pub action: ModelAction,
}

#[derive(Subcommand, Debug)]
pub enum ModelAction {
    /// Download a model
    Pull {
        /// Model name (e.g., qwen2.5-coder-1.5b-instruct-q4_k_m)
        #[arg(value_name = "MODEL")]
        name: String,

        /// Force re-download
        #[arg(long)]
        force: bool,
    },
    /// List downloaded models
    List,
    /// Remove a model
    Remove {
        #[arg(value_name = "MODEL")]
        name: String,
    },
    /// Show model info
    Info {
        #[arg(value_name = "MODEL")]
        name: Option<String>,
    },
}

#[derive(Parser, Debug)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: ShellType,

    /// Output file (stdout if not specified)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ShellType {
    Auto,
    Pwsh,
    Bash,
    Zsh,
    Fish,
    Cmd,
}

impl std::fmt::Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellType::Auto => write!(f, "auto"),
            ShellType::Pwsh => write!(f, "pwsh"),
            ShellType::Bash => write!(f, "bash"),
            ShellType::Zsh => write!(f, "zsh"),
            ShellType::Fish => write!(f, "fish"),
            ShellType::Cmd => write!(f, "cmd"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ConfigFormat {
    Toml,
    Json,
    Yaml,
}