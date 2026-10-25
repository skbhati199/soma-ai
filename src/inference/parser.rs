//! Output Parser - Clean AI responses

/// Clean AI output to extract just the command
pub fn clean_output(raw: &str) -> String {
    let mut cleaned = raw.trim().to_string();

    // Remove markdown code fences
    cleaned = remove_code_fences(&cleaned);

    // Take first non-empty line
    let first_line = cleaned
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("");

    first_line.trim().to_string()
}

fn remove_code_fences(text: &str) -> String {
    let mut result = text.to_string();

    // Remove ```language ... ```
    if let Some(start) = result.find("```") {
        if let Some(end) = result[start + 3..].find("```") {
            let before = &result[..start];
            let after = &result[start + 3 + end + 3..];
            result = format!("{}{}", before, after);
        }
    }

    // Remove inline backticks
    result = result.replace('`', "");

    result
}

/// Extract command from various AI output formats
pub fn extract_command(output: &str) -> String {
    let cleaned = clean_output(output);

    // Handle common prefixes
    let prefixes = [
        "PowerShell Command:",
        "Command:",
        "Bash Command:",
        "CMD Command:",
        "$ ",
        "> ",
        "PS> ",
    ];

    for prefix in &prefixes {
        if cleaned.starts_with(prefix) {
            return cleaned[prefix.len()..].trim().to_string();
        }
    }

    cleaned
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
