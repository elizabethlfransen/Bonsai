use crate::{
    adapter::{
        AutoConfirmPromptAdapter, CliClackPromptAdapter, NonInteractivePromptAdapter, PromptAdapter,
    },
    util::io::{RenderMode, set_color_override, set_is_quiet, setup_miettte},
};
use clap::{Args, Parser};
use miette::Result;
mod adapter;
mod util;

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

    /// Formats output without color or special formatting, implies no-input
    #[arg(long, global = true, conflicts_with = "json")]
    plain: bool,

    /// Will not output non-essential output and simplify errors
    #[arg(long, global = true, visible_alias = "silent")]
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
    set_color_override_from_args(&args.global_options);
    set_is_quiet(args.global_options.quiet);
    set_render_mode(&args.global_options);
    setup_miettte()?;
    let prompt_adapter = get_prompt_adapter(&args.global_options);
    prompt_adapter.confirm("test").interact()?;
    cli_println!("test");
    Ok(())
}

fn set_color_override_from_args(global_options: &GlobalOptions) {
    // priority: flag over env, force over no

    if global_options.force_color {
        set_color_override(true);
    } else if global_options.no_color {
        set_color_override(false);
    } else if util::env::force_color() {
        set_color_override(true);
    } else if util::env::no_color() {
        set_color_override(false);
    }
}

fn set_render_mode(global_options: &GlobalOptions) {
    if global_options.plain {
        util::io::set_render_mode(RenderMode::Plain);
    }
    if global_options.json {
        util::io::set_render_mode(RenderMode::Json);
    }
}

fn wrap_with_auto_confirm<T: PromptAdapter + 'static>(
    base_prompt_adapter: T,
    global_options: &GlobalOptions,
) -> Box<dyn PromptAdapter> {
    if global_options.yes {
        Box::new(AutoConfirmPromptAdapter::new(base_prompt_adapter, true))
    } else if global_options.no {
        Box::new(AutoConfirmPromptAdapter::new(base_prompt_adapter, false))
    } else {
        Box::new(base_prompt_adapter)
    }
}

fn get_prompt_adapter(global_options: &GlobalOptions) -> Box<dyn PromptAdapter> {
    if global_options.no_input
        || util::env::no_input()
        || global_options.json
        || global_options.plain
        || global_options.quiet
    {
        wrap_with_auto_confirm(NonInteractivePromptAdapter, global_options)
    } else {
        wrap_with_auto_confirm(CliClackPromptAdapter {}, global_options)
    }
}
