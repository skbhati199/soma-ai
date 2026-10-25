//! `soma completions` command

use crate::cli::args::CompletionsArgs;
use crate::cli::completions::run as run_completions;
use anyhow::Result;

pub fn run(args: CompletionsArgs) -> Result<()> {
    run_completions(args)
}
