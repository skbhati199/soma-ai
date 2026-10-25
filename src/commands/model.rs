//! `soma model` command

use anyhow::Result;
use crate::config::Config;
use crate::cli::args::{ModelArgs, ModelAction};
use crate::model::manager::ModelManager;

pub fn run(args: ModelArgs, config: &Config) -> Result<()> {
    let manager = ModelManager::new(config.model.clone());

    match args.action {
        ModelAction::Pull { name, force } => pull_model(&manager, &name, force),
        ModelAction::List => list_models(&manager),
        ModelAction::Remove { name } => remove_model(&manager, &name),
        ModelAction::Info { name } => show_model_info(&manager, name),
    }
}

fn pull_model(manager: &ModelManager, name: &str, force: bool) -> Result<()> {
    if !force {
        let path = manager.model_path(Some(name));
        if path.exists() {
            println!("Model already exists at {}", path.display());
            println!("Use --force to re-download");
            return Ok(());
        }
    }

    println!("Downloading model: {}", name);
    let path = manager.ensure_model(Some(name))?;
    println!("✓ Model saved to {}", path.display());
    Ok(())
}

fn list_models(manager: &ModelManager) -> Result<()> {
    let models = manager.list_models()?;

    if models.is_empty() {
        println!("No models downloaded.");
        println!("Run `soma model pull <name>` to download a model.");
        return Ok(());
    }

    println!("{:<40} {:>10}  {}", "NAME", "SIZE", "PATH");
    println!("{}", "-".repeat(80));

    for model in models {
        println!("{:<40} {:>10}  {}", model.name, model.size_human(), model.path.display());
    }
    Ok(())
}

fn remove_model(manager: &ModelManager, name: &str) -> Result<()> {
    print!("Remove model '{}'? [y/N] ", name);
    use std::io::{stdout, Write};
    stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Cancelled");
        return Ok(());
    }

    manager.remove_model(name)?;
    println!("✓ Model removed");
    Ok(())
}

fn show_model_info(manager: &ModelManager, name: Option<String>) -> Result<()> {
    let info = manager.get_model_info(name.as_deref())?;
    
    println!("Name: {}", info.name);
    println!("Path: {}", info.path.display());
    println!("Size: {}", info.size_human());
    
    if let Some(modified) = info.modified {
        println!("Modified: {:?}", modified);
    }
    
    Ok(())
}