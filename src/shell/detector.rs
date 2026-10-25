//! Shell Detection

use anyhow::Result;
use std::env;
use std::path::PathBuf;
use which::which;

use crate::config::schema::ShellType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectedShell {
    Pwsh,
    Bash,
    Zsh,
    Fish,
    Cmd,
    Unknown(String),
}

impl DetectedShell {
    pub fn detect() -> Result<Self> {
        // 1. Check SOMA_SHELL env var
        if let Ok(shell) = env::var("SOMA_SHELL") {
            return Ok(Self::from_string(&shell));
        }

        // 2. Check parent process (most reliable)
        if let Ok(parent) = Self::get_parent_process() {
            if let Some(shell) = Self::identify_shell(&parent) {
                return Ok(shell);
            }
        }

        // 3. Check SHELL env var (Unix)
        if let Ok(shell) = env::var("SHELL") {
            if let Some(shell) = Self::identify_shell(&shell) {
                return Ok(shell);
            }
        }

        // 4. Check common shells in PATH
        for shell_name in &["pwsh", "powershell", "bash", "zsh", "fish"] {
            if which(shell_name).is_ok() {
                return Ok(Self::from_string(shell_name));
            }
        }

        // 5. Windows default
        #[cfg(windows)]
        {
            return Ok(DetectedShell::Pwsh);
        }

        #[cfg(not(windows))]
        {
            return Ok(DetectedShell::Bash);
        }
    }

    pub fn from_string(s: &str) -> Self {
        let s = s.to_lowercase();
        if s.contains("pwsh") || s.contains("powershell") {
            DetectedShell::Pwsh
        } else if s.contains("bash") {
            DetectedShell::Bash
        } else if s.contains("zsh") {
            DetectedShell::Zsh
        } else if s.contains("fish") {
            DetectedShell::Fish
        } else if s.contains("cmd") {
            DetectedShell::Cmd
        } else {
            DetectedShell::Unknown(s)
        }
    }

    fn get_parent_process() -> Result<String> {
        #[cfg(windows)]
        {
            use std::process::Command;
            let output = Command::new("wmic")
                .args(["process", "where", "ProcessId=", &std::process::id().to_string(), "get", "ParentProcessId"])
                .output()?;
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse parent PID and get its name
            // Simplified for now
            Ok("pwsh".to_string())
        }
        #[cfg(not(windows))]
        {
            use std::fs;
            let stat = fs::read_to_string(format!("/proc/{}/stat", std::process::id()))?;
            let parts: Vec<&str> = stat.split_whitespace().collect();
            if parts.len() > 3 {
                let ppid = parts[3];
                let comm = fs::read_to_string(format!("/proc/{}/comm", ppid))?;
                Ok(comm.trim().to_string())
            } else {
                anyhow::bail!("Could not parse /proc/self/stat")
            }
        }
    }

    fn identify_shell(path: &str) -> Option<Self> {
        let name = PathBuf::from(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        Some(Self::from_string(name))
    }

    pub fn to_shell_type(&self) -> ShellType {
        match self {
            DetectedShell::Pwsh => ShellType::Pwsh,
            DetectedShell::Bash => ShellType::Bash,
            DetectedShell::Zsh => ShellType::Zsh,
            DetectedShell::Fish => ShellType::Fish,
            DetectedShell::Cmd => ShellType::Cmd,
            DetectedShell::Unknown(_) => ShellType::Auto,
        }
    }

    pub fn config_name(&self) -> &'static str {
        match self {
            DetectedShell::Pwsh => "pwsh",
            DetectedShell::Bash => "bash",
            DetectedShell::Zsh => "zsh",
            DetectedShell::Fish => "fish",
            DetectedShell::Cmd => "cmd",
            DetectedShell::Unknown(_) => "unknown",
        }
    }

    pub fn profile_paths(&self) -> Vec<PathBuf> {
        match self {
            DetectedShell::Pwsh => profiles::pwsh_profile_paths(),
            DetectedShell::Bash => profiles::bash_profile_paths(),
            DetectedShell::Zsh => profiles::zsh_profile_paths(),
            DetectedShell::Fish => profiles::fish_profile_paths(),
            DetectedShell::Cmd => vec![],
            DetectedShell::Unknown(_) => vec![],
        }
    }

    pub fn rc_file_name(&self) -> &'static str {
        match self {
            DetectedShell::Pwsh => "Microsoft.PowerShell_profile.ps1",
            DetectedShell::Bash => ".bashrc",
            DetectedShell::Zsh => ".zshrc",
            DetectedShell::Fish => "config.fish",
            DetectedShell::Cmd => "",
            DetectedShell::Unknown(_) => "",
        }
    }
}