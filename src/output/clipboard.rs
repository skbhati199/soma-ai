//! Clipboard Operations

use anyhow::Result;

#[cfg(feature = "clipboard")]
use arboard::Clipboard;

/// Copy text to system clipboard
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    #[cfg(feature = "clipboard")]
    {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(text)?;
        Ok(())
    }

    #[cfg(not(feature = "clipboard"))]
    {
        // Fallback: try platform-specific commands
        copy_fallback(text)
    }
}

/// Get text from clipboard
pub fn get_from_clipboard() -> Result<String> {
    #[cfg(feature = "clipboard")]
    {
        let mut clipboard = Clipboard::new()?;
        Ok(clipboard.get_text()?)
    }

    #[cfg(not(feature = "clipboard"))]
    {
        anyhow::bail!("Clipboard feature not enabled")
    }
}

fn copy_fallback(text: &str) -> Result<()> {
    #[cfg(windows)]
    {
        use std::process::Command;
        // Use clip.exe on Windows
        let mut child = Command::new("cmd")
            .args(["/c", "clip"])
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        Ok(())
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        use std::process::Command;
        // Try xclip, then xsel
        for cmd in &["xclip", "xsel"] {
            if which::which(cmd).is_ok() {
                let mut child = Command::new(cmd)
                    .args(if *cmd == "xclip" {
                        &["-selection", "clipboard"]
                    } else {
                        &["-b", "-i"]
                    })
                    .stdin(std::process::Stdio::piped())
                    .spawn()?;

                if let Some(mut stdin) = child.stdin.take() {
                    use std::io::Write;
                    stdin.write_all(text.as_bytes())?;
                }
                child.wait()?;
                return Ok(());
            }
        }
        anyhow::bail!("No clipboard utility found (xclip or xsel)")
    }
}
