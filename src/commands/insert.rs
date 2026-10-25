//! `soma insert` command - for keybindings

use crate::cli::args::InsertArgs;
use crate::config::Config;
use crate::inference::engine::InferenceEngine;
use crate::inference::parser::extract_command;
use crate::inference::prompt::build_prompt;
use crate::model::manager::ModelManager;
use crate::shell::detector::DetectedShell;
use anyhow::Result;

pub fn run(args: InsertArgs, config: &Config) -> Result<()> {
    // Determine shell type
    let _shell = args
        .shell
        .map(|s| s.into())
        .unwrap_or_else(|| DetectedShell::detect().unwrap_or(DetectedShell::Pwsh));

    // Get model path
    let model_manager = ModelManager::new(config.model.clone());
    model_manager.ensure_model(args.model.as_deref())?;

    // Build prompt with skill support
    let prompt_type = config.prompt.system;
    let prompt = crate::inference::prompt::build_prompt(config, prompt_type, &args.query, args.skill.as_deref());

    // Run inference
    let mut engine = InferenceEngine::new(config.model.clone())?;

    let max_tokens = args.max_tokens.unwrap_or(config.prompt.max_tokens);
    let temperature = args.temperature.unwrap_or(config.prompt.temperature);

    let raw_output = engine.generate(&prompt, max_tokens, temperature)?;
    let command = extract_command(&raw_output);

    // Output just the command (for keybinding to capture)
    print!("{}", command);

    Ok(())
}
