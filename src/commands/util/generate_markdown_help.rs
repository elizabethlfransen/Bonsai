use clap::{Arg, Args, Command, CommandFactory, builder::StyledStr};
use miette::Result;

use crate::cli::{BonsaiCli, GenerateMarkdownHelpArgs, GlobalOptions};

pub fn handle_command(
    GenerateMarkdownHelpArgs { out_dir }: GenerateMarkdownHelpArgs,
) -> Result<()> {
    print_command_page(BonsaiCli::command(), Vec::new());
    Ok(())
}

fn print_command_page(cmd: Command, parent_path: Vec<&str>) {
    let name = cmd
        .get_bin_name()
        .or_else(|| cmd.get_display_name())
        .unwrap_or_else(|| cmd.get_name());
    let mut cmd_path = parent_path.clone();
    cmd_path.push(&name);
    print_header_and_about(&cmd, &cmd_path);
    print_available_commands(&cmd, &cmd_path);
    print_aliases(&cmd, &parent_path);
}

fn print_header_and_about(cmd: &Command, path: &Vec<&str>) {
    let about = cmd
        .get_long_about()
        .or_else(|| cmd.get_about())
        .map(|x| x.to_string())
        .unwrap_or(String::new());
    println!("# {}", path.join(" "));
    println!("{}", about);
}

fn print_available_commands(cmd: &Command, path: &Vec<&str>) {
    if !cmd.has_subcommands() {
        return;
    }
    println!("## Available Commands");
    for subcommand in cmd.get_subcommands() {
        print_subcommand_link(&subcommand, &path);
    }
}

fn print_subcommand_link(cmd: &Command, path: &Vec<&str>) {
    let cmd_name = cmd
        .get_bin_name()
        .or_else(|| cmd.get_display_name())
        .unwrap_or_else(|| cmd.get_name());

    let mut full_name_path = path.clone();
    full_name_path.push(cmd_name);
    let full_name = full_name_path.join(" ");
    println!("* [{full_name}](./{cmd_name}.md)");
}

fn print_aliases(cmd: &Command, path: &Vec<&str>) {
    let aliases: Vec<&str> = cmd.get_visible_aliases().collect();
    if aliases.is_empty() {
        return;
    }
    println!("## Aliases");
    let formatted_aliases = aliases
        .into_iter()
        .map(|item| {
            let mut alias_path = path.clone();
            alias_path.push(item);
            let fully_qualified_alias = alias_path.join(" ");
            format!("`{fully_qualified_alias}`")
        })
        .collect::<Vec<_>>()
        .join(", ");
    println!("{formatted_aliases}");
}
