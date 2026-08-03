use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct BonsaiCli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten, next_help_heading = "Global Options")]
    pub global_options: GlobalOptions,
}

#[derive(Args, Debug)]
pub struct GlobalOptions {
    /// Disable color in the terminal
    #[arg(long, global = true, conflicts_with = "force_color")]
    pub no_color: bool,

    /// Enables color in the terminal
    #[arg(long, global = true, conflicts_with = "no_color")]
    pub force_color: bool,

    /// Disabled interactivity
    #[arg(long, global = true)]
    pub no_input: bool,
    /// Formats output as json, implies no-input
    #[arg(long, global = true, conflicts_with = "plain")]
    pub json: bool,

    /// Formats output without color or special formatting, implies no-input
    #[arg(long, global = true, conflicts_with = "json")]
    pub plain: bool,

    /// Will not output non-essential output and simplify errors
    #[arg(long, global = true, visible_alias = "silent")]
    pub quiet: bool,

    /// answer yes to all confirms
    #[arg(long, short, global = true, conflicts_with = "no")]
    pub yes: bool,

    /// answer no to all confirms
    #[arg(long, short, global = true, conflicts_with = "yes")]
    pub no: bool,
}

#[derive(Args, Debug)]
pub struct GenerateCompletionsArgs {
    /// The target shell
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Args, Debug)]
pub struct GenerateManArgs {
    /// Subcommand to inspect
    #[arg(num_args=0..)]
    pub subcommand: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    /// Generate shell completions
    GenerateCompletions(GenerateCompletionsArgs),
    /// Generate man pages
    GenerateMan(GenerateManArgs),
}
