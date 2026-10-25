//! Output Parser - Clean AI responses

/// Clean AI output to extract just the command
pub fn clean_output(raw: &str) -> String {
    // If there is a code block ```lang\ncommand\n```, extract inside the fence
    if let Some(start) = raw.find("```") {
        let after_start = &raw[start + 3..];
        let content_start = if let Some(newline) = after_start.find('\n') {
            &after_start[newline + 1..]
        } else {
            after_start
        };

        if let Some(end) = content_start.find("```") {
            let inside = &content_start[..end];
            let first_line = inside.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
            return first_line.trim().trim_matches('`').to_string();
        }
    }

    // Otherwise, find first non-empty line that isn't purely markdown fence
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("```") {
            continue;
        }
        return trimmed.trim_matches('`').to_string();
    }

    String::new()
}

/// Extract command from various AI output formats
pub fn extract_command(output: &str) -> String {
    let prefixes = [
        "PowerShell Command:",
        "Command:",
        "Bash Command:",
        "CMD Command:",
        "$ ",
        "> ",
        "PS> ",
    ];

    let mut current = output.trim();
    for prefix in &prefixes {
        if let Some(rest) = current.strip_prefix(prefix) {
            current = rest.trim();
        }
    }

    clean_output(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_simple() {
        assert_eq!(
            clean_output("Get-ChildItem -Recurse"),
            "Get-ChildItem -Recurse"
        );
    }

    #[test]
    fn test_clean_markdown() {
        let input = "```powershell\nGet-ChildItem -Recurse\n```";
        assert_eq!(clean_output(input), "Get-ChildItem -Recurse");
    }

    #[test]
    fn test_clean_with_prefix() {
        let input = "PowerShell Command:\nGet-ChildItem -Recurse";
        assert_eq!(extract_command(input), "Get-ChildItem -Recurse");
    }

    #[test]
    fn test_clean_multiline() {
        let input = "Some explanation\n```powershell\nGet-ChildItem -Recurse\n```\nMore text";
        assert_eq!(clean_output(input), "Get-ChildItem -Recurse");
    }
}
