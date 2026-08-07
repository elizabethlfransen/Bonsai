use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

use crate::{check_cli, examples};

check_cli! {
    /// Build minecraft modpacks for CurseForge and Modrinth from the command line.
    #[derive(Parser, Debug)]
    #[command(
        version,
        about,
        name = "bonsai",
        after_long_help = examples!(
            "Initialize a project" => "bonsai init",
            "Add a mod" => "bonsai mod add test"
        )
    )]
    pub struct BonsaiCli {
        #[command(subcommand)]
        pub command: Commands,
        #[command(flatten, next_help_heading = "Global Options")]
        pub global_options: GlobalOptions,
    }

    #[derive(Subcommand, Debug)]
    pub enum Commands {
        Init,
        /// Generate shell completions.
        ///
        /// Typically you would add the follow line in your .bashrc or equivalent file:
        ///
        /// eval "$(bonsai completions)"
        Completions(GenerateCompletionsArgs),
        /// Generate man pages. If the directory already exists this will fail. If you want to update your man pages use `--force`
        GenerateMan(GenerateManArgs),
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
        pub shell: Option<Shell>,
    }

    #[derive(Args, Debug)]
    pub struct GenerateManArgs {
        /// output directory to generate man args.
        #[arg(default_value = "/usr/local/share/man/man1")]
        pub out: PathBuf,
        #[arg(short, long)]
        pub force: bool,
    }

    #[derive(Args, Debug)]
    pub struct GenerateMarkdownHelpArgs {
        /// The directory to generate markdown files
        pub out_dir: PathBuf,
    }
}
