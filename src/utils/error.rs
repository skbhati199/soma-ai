//! Error Types

use anyhow::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SomaError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Model download failed: {0}")]
    ModelDownloadFailed(String),

    #[error("Model checksum mismatch: {0}")]
    ChecksumMismatch(String),

    #[error("Inference failed: {0}")]
    InferenceFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Shell not supported: {0}")]
    ShellNotSupported(String),

    #[error("Installation failed: {0}")]
    InstallationFailed(String),

    #[error("Clipboard error: {0}")]
    ClipboardError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("Figment error: {0}")]
    FigmentError(#[from] figment::Error),
}

pub type Result<T> = std::result::Result<T, SomaError>;