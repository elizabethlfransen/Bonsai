use crate::{
    adapter::{
        AutoConfirmPromptAdapter, CliClackPromptAdapter, NonInteractivePromptAdapter, PromptAdapter,
    },
    util::io::{RenderMode, set_color_override, set_is_quiet, setup_miettte},
};
use clap::Parser;
use miette::Result;
mod adapter;
mod cli;
mod commands;
mod util;
use cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let BonsaiCli {
        global_options,
        command,
    } = BonsaiCli::parse();
    set_color_override_from_args(&global_options);
    set_is_quiet(global_options.quiet);
    set_render_mode(&global_options);
    setup_miettte()?;
    let prompt_adapter = get_prompt_adapter(&global_options);
    match command {
        Commands::Init => {
            cli_println!("Init called");
            Ok(())
        }
        Commands::Completions(args) => commands::completions::handle_command(args),
        Commands::GenerateMan(args) => commands::generate_man::handle_command(args),
        Commands::Util { command } => match command {
            UtilCommands::GenerateMarkdownHelp(args) => {
                commands::util::generate_markdown_help::handle_command(args)
            }
        },
    }
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
