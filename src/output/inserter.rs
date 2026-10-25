//! Command Inserter - Insert commands into shell

use crate::shell::detector::DetectedShell;
use anyhow::Result;

/// Insert command at cursor position (for keybindings)
pub fn insert_at_cursor(shell: &DetectedShell, command: &str) -> Result<()> {
    match shell {
        DetectedShell::Pwsh => insert_pwsh(command),
        DetectedShell::Bash => insert_bash(command),
        DetectedShell::Zsh => insert_zsh(command),
        DetectedShell::Fish => insert_fish(command),
        _ => {
            // Fallback to clipboard
            crate::output::clipboard::copy_to_clipboard(command)?;
            Ok(())
        }
    }
}

/// Insert command on next line
pub fn insert_next_line(shell: &DetectedShell, command: &str) -> Result<()> {
    let cmd_with_newline = format!("\n{}", command);
    insert_at_cursor(shell, &cmd_with_newline)
}

// PowerShell: Use PSReadLine API
#[cfg(windows)]
fn insert_pwsh(command: &str) -> Result<()> {
    // This is called from within PowerShell via the soma-insert function
    // The actual insertion is done by the PowerShell function using
    // [Microsoft.PowerShell.PSConsoleReadLine]::Insert()
    // Here we just output the command to stdout for the function to capture
    print!("{}", command);
    Ok(())
}

#[cfg(not(windows))]
fn insert_pwsh(_command: &str) -> Result<()> {
    anyhow::bail!("PowerShell insertion only available on Windows")
}

// Bash: Use READLINE variables
fn insert_bash(command: &str) -> Result<()> {
    // When called from bash function, we can set READLINE_LINE and READLINE_POINT
    // But this only works when sourced, not as a subprocess
    // The bash function handles this directly
    print!("{}", command);
    Ok(())
}

// Zsh: Use ZLE widgets
fn insert_zsh(command: &str) -> Result<()> {
    print!("{}", command);
    Ok(())
}

// Fish: Use commandline builtin
fn insert_fish(command: &str) -> Result<()> {
    print!("{}", command);
    Ok(())
}

/// Generate shell function for insertion
pub fn generate_insert_function(shell: &DetectedShell) -> String {
    match shell {
        DetectedShell::Pwsh => r#"
function soma-insert {
    $query = Read-Host "soma"
    if ($query) {
        $cmd = soma insert $query
        if ($cmd) { [Microsoft.PowerShell.PSConsoleReadLine]::Insert("`n$cmd") }
    }
}
"#
        .to_string(),

        DetectedShell::Bash => r#"
_soma_insert() {
    local query
    read -erp "soma> " query
    if [[ -n "$query" ]]; then
        local cmd=$(soma insert "$query")
        if [[ -n "$cmd" ]]; then
            READLINE_LINE="${READLINE_LINE}$'\n'$cmd"
            READLINE_POINT=${#READLINE_LINE}
        fi
    fi
}
"#
        .to_string(),

        DetectedShell::Zsh => r#"
_soma_insert() {
    local query
    vared -p 'soma> ' query
    if [[ -n "$query" ]]; then
        local cmd=$(soma insert "$query")
        if [[ -n "$cmd" ]]; then
            LBUFFER+=$'\n'$cmd
            zle reset-prompt
        fi
    fi
}
zle -N _soma_insert
"#
        .to_string(),

        DetectedShell::Fish => r#"
function _soma_insert
    set -l query (commandline -p "soma> ")
    if test -n "$query"
        set -l cmd (soma insert "$query")
        if test -n "$cmd"
            commandline -i "$cmd"
            commandline -f repaint
        end
    end
end
"#
        .to_string(),

        _ => String::new(),
    }
}
