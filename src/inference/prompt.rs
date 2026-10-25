//! Prompt Templates with Skill Support

use crate::config::schema::SystemPromptType;
use crate::config::Config;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub content: String,
    pub triggers: Vec<String>,
}

pub fn build_prompt(config: &Config, system: SystemPromptType, query: &str, forced_skill: Option<&str>) -> String {
    let skills = if system == SystemPromptType::Auto {
        load_skills_for_query(config, query, forced_skill)
    } else {
        Vec::new()
    };
    
    let skills_text = if skills.is_empty() {
        String::new()
    } else {
        format!("\n\n{}\n", skills.join("\n\n"))
    };
    
    let system_prompt = if system == SystemPromptType::Auto {
        format!("{}{}", get_base_prompt(SystemPromptType::PowerShell), skills_text)
    } else {
        get_base_prompt(system).to_string()
    };
    
    format!(
        "<|system|>\n{}\n<|user|>\nRequest: {}\nPowerShell Command:\n<|assistant|>",
        system_prompt, query
    )
}

fn get_base_prompt(system: SystemPromptType) -> &'static str {
    match system {
        SystemPromptType::PowerShell => POWERSHELL_SYSTEM_PROMPT,
        SystemPromptType::Bash => BASH_SYSTEM_PROMPT,
        SystemPromptType::Generic => GENERIC_SYSTEM_PROMPT,
        SystemPromptType::Auto => POWERSHELL_SYSTEM_PROMPT, // base for auto
    }
}

fn load_skills_for_query(config: &Config, query: &str, forced_skill: Option<&str>) -> Vec<String> {
    let skills_dir = config.prompt.skills_dir.as_deref().unwrap_or("skills");
    let max_skills = config.prompt.max_skills.min(2);
    
    let skills_dir = PathBuf::from(skills_dir);
    if !skills_dir.exists() {
        return Vec::new();
    }
    
    let all_skills = load_all_skills(&skills_dir);
    if all_skills.is_empty() {
        return Vec::new();
    }
    
    let query_lower = query.to_lowercase();
    let mut matched_skills = Vec::new();
    
    // If a skill is forced, use it
    if let Some(forced) = forced_skill {
        if let Some(skill) = all_skills.iter().find(|s| s.name == forced) {
            matched_skills.push(skill.clone());
            return vec![skill.content.clone()];
        }
    }
    
    // Match by triggers
    for skill in &all_skills {
        for trigger in &skill.triggers {
            if query_lower.contains(trigger) {
                matched_skills.push(skill.clone());
                break;
            }
        }
    }
    
    // Limit to max_skills
    matched_skills.truncate(max_skills);
    
    matched_skills.into_iter().map(|s| s.content).collect()
}

pub fn load_all_skills(skills_dir: &Path) -> Vec<Skill> {
    let mut skills = Vec::new();
    
    if let Ok(entries) = fs::read_dir(skills_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(skill) = parse_skill(&content) {
                        skills.push(skill);
                    }
                }
            }
        }
    }
    
    skills
}

pub fn parse_skill(content: &str) -> Option<Skill> {
    // Parse YAML frontmatter
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() || lines[0] != "---" {
        return None;
    }
    
    let mut frontmatter_end = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if *line == "---" {
            frontmatter_end = Some(i);
            break;
        }
    }
    
    let frontmatter_end = frontmatter_end?;
    let frontmatter = lines[1..frontmatter_end].join("\n");
    let skill_content = lines[frontmatter_end + 1..].join("\n").trim().to_string();
    
    // Simple YAML parsing for name and triggers
    let mut name = String::new();
    let mut triggers = Vec::new();
    
    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("name:") {
            name = val.trim().trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("triggers:") {
            let val = val.trim();
            // Parse array like [item1, item2, ...]
            if val.starts_with('[') && val.ends_with(']') {
                let inner = &val[1..val.len()-1];
                for item in inner.split(',') {
                    let item = item.trim().trim_matches('"').trim_matches('\'').trim();
                    if !item.is_empty() {
                        triggers.push(item.to_string());
                    }
                }
            }
        }
    }
    
    if name.is_empty() {
        return None;
    }
    
    Some(Skill {
        name,
        content: skill_content,
        triggers,
    })
}


const POWERSHELL_SYSTEM_PROMPT: &str = r#"You are a PowerShell 7+ command expert on Windows. Output ONLY the PowerShell command(s) to run.

CRITICAL RULES:
- Use PowerShell cmdlets ONLY: Get-ChildItem, Where-Object, Select-Object, Sort-Object, Measure-Object, Select-String, Compress-Archive, Get-NetTCPConnection, Stop-Process, Invoke-WebRequest
- NEVER use Unix commands: ls, grep, find, awk, sed, tar, du, df, cat, head, tail
- Use -Filter for simple patterns (fast, filesystem-level): Get-ChildItem -Filter '*.txt'
- Use -Include for multiple patterns: Get-ChildItem -Include '*.txt','*.log'
- Use Where-Object for complex logic: Where-Object { $_.Extension -eq '.txt' -and $_.LastWriteTime -gt (Get-Date).AddDays(-1) }
- For recursive: Get-ChildItem -Recurse -File
- For files only: -File  |  For directories only: -Directory
- Pipeline | for chaining
- NO explanations, NO markdown, NO formatting, NO comments. Just raw command.

STOP. Output ONLY the single PowerShell command for this request. Nothing else. No examples, no explanations, no extra text."#;

const BASH_SYSTEM_PROMPT: &str = r#"You are a Bash command expert on Linux/macOS. Output ONLY the Bash command(s) to run.

CRITICAL RULES:
- Use standard Bash commands: find, grep, ls, awk, sed, tar, du, df, cat, head, tail, xargs
- Use pipelines | for chaining
- Use -exec or xargs for find actions
- NO explanations, NO markdown, NO formatting, NO comments. Just raw command.

STOP. Output ONLY the single Bash command for this request. Nothing else."#;

const GENERIC_SYSTEM_PROMPT: &str = r#"You are a terminal command expert. Output ONLY the command to run.

CRITICAL RULES:
- Use appropriate commands for the target shell
- NO explanations, NO markdown, NO formatting, NO comments. Just raw command.

STOP. Output ONLY the single command for this request. Nothing else."#;