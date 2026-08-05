use std::io;

use crate::cli::{BonsaiCli, GenerateCompletionsArgs};
use clap::CommandFactory;
use clap_complete::Shell;
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum GenerateCompletionsError {
    #[error("Unable to determine shell from environment")]
    #[diagnostic(
        help("Provide shell with --shell"),
        code(bonsai::completions::unable_to_determine_shell)
    )]
    NoShellFound,
}

pub fn handle_command(args: GenerateCompletionsArgs) -> Result<()> {
    let mut cmd = BonsaiCli::command();
    let bin_name = cmd.get_name().to_string();
    let shell = args
        .shell
        .or(Shell::from_env())
        .ok_or(GenerateCompletionsError::NoShellFound)?;
    clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}
