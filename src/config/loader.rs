//! Configuration Loader

use anyhow::Result;
use std::path::PathBuf;

use crate::config::schema::Config;

pub fn load_config(config_path: &Option<PathBuf>) -> Result<Config> {
    Config::load(config_path)
}

pub fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("soma/config.toml")
}

pub fn ensure_config_dir() -> Result<PathBuf> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(path)
}