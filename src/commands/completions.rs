//! `soma completions` command

use anyhow::Result;
use crate::cli::args::CompletionsArgs;
use crate::cli::completions::run as run_completions;

pub fn run(args: CompletionsArgs) -> Result<()> {
    run_completions(args)
}