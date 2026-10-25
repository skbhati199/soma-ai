//! `soma skills` command

use crate::cli::args::{SkillsAction, SkillsArgs};
use crate::config::Config;
use crate::inference::prompt::{load_all_skills, parse_skill};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn run(args: SkillsArgs, config: &Config) -> Result<()> {
    match args.action {
        SkillsAction::List => list_skills(config),
        SkillsAction::Show { name } => show_skill(config, &name),
    }
}

fn list_skills(config: &Config) -> Result<()> {
    let skills_dir = config.prompt.skills_dir.as_deref().unwrap_or("skills");
    let skills_dir = std::path::PathBuf::from(skills_dir);

    if !skills_dir.exists() {
        println!("No skills directory found at: {}", skills_dir.display());
        return Ok(());
    }

    let skills = crate::inference::prompt::load_all_skills(&skills_dir);

    if skills.is_empty() {
        println!("No skills found in: {}", skills_dir.display());
        return Ok(());
    }

    println!("Available skills:");
    for skill in skills {
        println!(
            "  {} - triggers: [{}]",
            skill.name,
            skill.triggers.join(", ")
        );
    }

    println!("\nUsage: soma ask --skill <name> \"query\"");
    println!("       soma config set prompt.system auto  # Enable auto skill loading");

    Ok(())
}

fn show_skill(config: &Config, name: &str) -> Result<()> {
    let skills_dir = config.prompt.skills_dir.as_deref().unwrap_or("skills");
    let skills_dir = std::path::PathBuf::from(skills_dir);

    if !skills_dir.exists() {
        println!("No skills directory found at: {}", skills_dir.display());
        return Ok(());
    }

    let skills = crate::inference::prompt::load_all_skills(&skills_dir);

    if let Some(skill) = skills.iter().find(|s| s.name == name) {
        println!("Skill: {}", skill.name);
        println!("Triggers: [{}]", skill.triggers.join(", "));
        println!("\nContent:\n{}", skill.content);
    } else {
        println!("Skill '{}' not found", name);
    }

    Ok(())
}
