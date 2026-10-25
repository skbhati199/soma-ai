//! Shell Profile Paths

use anyhow::Result;
use std::path::PathBuf;

pub fn pwsh_profile_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // Current user, current host (most common)
    if let Some(docs) = dirs::document_dir() {
        paths.push(docs.join("PowerShell/Microsoft.PowerShell_profile.ps1"));
        paths.push(docs.join("WindowsPowerShell/Microsoft.PowerShell_profile.ps1"));
    }

    // Fallback to home
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join("Documents/PowerShell/Microsoft.PowerShell_profile.ps1"));
    }

    paths
}

pub fn bash_profile_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".bashrc"));
        paths.push(home.join(".bash_profile"));
        paths.push(home.join(".profile"));
    }

    paths
}

pub fn zsh_profile_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".zshrc"));
        paths.push(home.join(".zprofile"));
    }

    // ZDOTDIR support
    if let Ok(zdotdir) = std::env::var("ZDOTDIR") {
        paths.insert(0, PathBuf::from(zdotdir).join(".zshrc"));
    }

    paths
}

pub fn fish_profile_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(config) = dirs::config_dir() {
        paths.push(config.join("fish/config.fish"));
    }

    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".config/fish/config.fish"));
    }

    paths
}

pub fn find_existing_profile(shell: &crate::shell::detector::DetectedShell) -> Option<PathBuf> {
    for path in shell.profile_paths() {
        if path.exists() {
            return Some(path);
        }
    }
    None
}

pub fn get_or_create_profile(shell: &crate::shell::detector::DetectedShell) -> Result<PathBuf> {
    if let Some(path) = find_existing_profile(shell) {
        return Ok(path);
    }

    // Create the first profile path
    let paths = shell.profile_paths();
    if let Some(path) = paths.first() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, "")?;
        return Ok(path.clone());
    }

    anyhow::bail!("No profile path found for shell: {:?}", shell)
}