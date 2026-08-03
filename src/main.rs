use clap::{Args, Parser};
use miette::Result;
mod adapter;

#[derive(Args, Debug)]
struct GlobalOptions {
    /// Disable color in the terminal
    #[arg(long, global = true, conflicts_with = "force_color")]
    no_color: bool,

    /// Enables color in the terminal
    #[arg(long, global = true, conflicts_with = "no_color")]
    force_color: bool,

    /// Disabled interactivity
    #[arg(long, global = true)]
    no_input: bool,
    /// Formats output as json, implies no-input
    #[arg(long, global = true, conflicts_with = "plain")]
    json: bool,

    /// Formats output without color or special formatting, does not imply no-input.
    #[arg(long, global = true, conflicts_with = "json")]
    plain: bool,

    /// Will not output non-essential output
    #[arg(long, global = true)]
    quiet: bool,

    /// answer yes to all confirms
    #[arg(long, short, global = true, conflicts_with = "no")]
    yes: bool,

    #[arg(long, short, global = true, conflicts_with = "yes")]
    /// answer no to all confirms
    no: bool,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct BonsaiCli {
    #[command(flatten, next_help_heading = "Global Options")]
    global_options: GlobalOptions,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = BonsaiCli::parse();
    if args.global_options.no_color {
        owo_colors::set_override(false);
    }
    if args.global_options.force_color {
        owo_colors::set_override(true);
    }
    Ok(())
}
