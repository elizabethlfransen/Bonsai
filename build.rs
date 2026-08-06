#[path = "src/cli.rs"]
mod cli;
#[path = "src/macros.rs"]
mod macros;
use crate::cli::BonsaiCli;
use clap::{Command, CommandFactory};
use std::fs::File;
use std::io::{Result, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cli.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR");
    let commands_dir = PathBuf::from(manifest_dir).join("docs/user/commands");
    let mut generated_commands: Vec<(String, Vec<String>)> = Vec::new();
    if fs::exists(&commands_dir)? {
        fs::remove_dir_all(&commands_dir)?;
    }
    write_command_page(
        &BonsaiCli::command(),
        &Vec::new(),
        &commands_dir,
        &mut generated_commands,
    )?;
    update_sidebar(&commands_dir.join("_sidebar.md"))?;
    Ok(())
}

fn write_command_page(
    cmd: &Command,
    parent_cmd_path: &Vec<&str>,
    output_dir: &Path,
    generated_commands: &mut Vec<(String, Vec<String>)>,
) -> Result<()> {
    let name = cmd
        .get_bin_name()
        .or_else(|| cmd.get_display_name())
        .unwrap_or_else(|| cmd.get_name());
    let mut cmd_path = parent_cmd_path.clone();
    cmd_path.push(&name);
    let output_file_path = output_dir.join(format!("{}.md", cmd_path.join("/")));
    generated_commands.push((
        name.to_string(),
        cmd_path.iter().map(|x| x.to_string()).collect(),
    ));
    if let Some(parent) = output_file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut output_file = File::create(output_file_path)?;
    write_header_and_about(&cmd, &cmd_path, &mut output_file)?;
    write_available_commands(&cmd, &cmd_path, &mut output_file)?;
    write_aliases(&cmd, &parent_cmd_path, &mut output_file)?;
    write_options(&cmd, &mut output_file)?;
    write_examples(&cmd, &mut output_file)?;
    for subcommand in cmd.get_subcommands() {
        write_command_page(subcommand, &cmd_path, output_dir, generated_commands)?;
    }
    Ok(())
}

fn write_header_and_about(
    cmd: &Command,
    path: &Vec<&str>,
    output_file: &mut impl Write,
) -> Result<()> {
    let about = cmd
        .get_long_about()
        .or_else(|| cmd.get_about())
        .map(|x| x.to_string())
        .unwrap_or(String::new());
    writeln!(
        output_file,
        "## {}  <!-- {{docsify-ignore}} -->",
        path.join(" ")
    )?;
    write_usage(cmd, path, output_file)?;
    writeln!(output_file, "### Description")?;
    writeln!(output_file, "{}", about)?;
    writeln!(output_file)?;
    Ok(())
}

fn write_usage(cmd: &Command, path: &Vec<&str>, output_file: &mut impl Write) -> Result<()> {
    let name = cmd
        .get_bin_name()
        .or_else(|| cmd.get_display_name())
        .unwrap_or_else(|| cmd.get_name());
    writeln!(output_file, "### Usage")?;
    let prefix = format!("Usage: {}", name);
    let rendered_usage = cmd.clone().render_usage().to_string();
    let usage = format!("{}{}", path.join(" "), &rendered_usage[prefix.len()..]);
    writeln!(output_file, "```shell\n{usage}\n```",)?;
    Ok(())
}

fn write_available_commands(
    cmd: &Command,
    path: &Vec<&str>,
    output_file: &mut impl Write,
) -> Result<()> {
    if !cmd.has_subcommands() {
        return Ok(());
    }
    writeln!(output_file, "### Available Commands")?;
    for subcommand in cmd.get_subcommands() {
        write_subcommand_link(&subcommand, &path, output_file)?;
    }
    Ok(())
}

fn write_subcommand_link(
    cmd: &Command,
    path: &Vec<&str>,
    output_file: &mut impl Write,
) -> Result<()> {
    let cmd_name = cmd
        .get_bin_name()
        .or_else(|| cmd.get_display_name())
        .unwrap_or_else(|| cmd.get_name());

    let mut full_name_path = path.clone();
    full_name_path.push(cmd_name);
    let full_name = full_name_path.join(" ");
    let full_path = full_name_path.join("/");
    writeln!(
        output_file,
        "* [{full_name}](/user/commands/{full_path}.md)"
    )?;
    Ok(())
}

fn write_aliases(cmd: &Command, path: &Vec<&str>, output_file: &mut impl Write) -> Result<()> {
    let aliases: Vec<&str> = cmd.get_visible_aliases().collect();
    if aliases.is_empty() {
        return Ok(());
    }
    println!("### Aliases");
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
    writeln!(output_file, "{formatted_aliases}")?;
    Ok(())
}

fn write_example(example_block: &str, output_file: &mut impl Write) -> Result<()> {
    let mut title: String = String::new();
    let mut description: String = String::new();
    let mut code: String = String::new();
    example_block
        .lines()
        .map(|line| line.trim())
        .for_each(|line| {
            if line.starts_with("#") {
                let line = line[1..].trim();
                if title.is_empty() {
                    title = line.to_string();
                } else {
                    description.push_str(line);
                    description.push('\n');
                }
            } else if line.starts_with("$ ") {
                let line = &line[2..];
                code.push_str(line);
                code.push('\n');
            }
        });
    writeln!(output_file, "#### {title}")?;
    writeln!(output_file, "{description}")?;
    writeln!(output_file, "```shell\n{code}```\n")?;
    Ok(())
}

fn write_examples(cmd: &Command, output_file: &mut impl Write) -> Result<()> {
    let after_help = cmd
        .get_after_long_help()
        .or(cmd.get_after_help())
        .map(|x| x.to_string())
        .unwrap_or(String::new());
    const EXAMPLE_HEADER: &'static str = "Examples:\n";
    let mut start_index = match after_help.find(EXAMPLE_HEADER) {
        Some(idx) => idx,
        None => return Ok(()),
    };
    start_index += EXAMPLE_HEADER.len();

    writeln!(output_file, "### Examples")?;
    after_help[start_index..]
        .split("\n\n")
        .filter(|example| !example.trim().is_empty())
        .map(|example_block| write_example(example_block, output_file))
        .collect()
}

const INDENT_SIZE: usize = 2;

fn get_command_sidebar_list(
    command: &Command,
    parent_command_path: &Vec<&str>,
    depth: usize,
) -> String {
    let name = command
        .get_bin_name()
        .or_else(|| command.get_display_name())
        .unwrap_or_else(|| command.get_name());
    let mut command_path = parent_command_path.clone();
    command_path.push(name);
    let doc_path = format!("/user/commands/{}.md", command_path.join("/"));
    let display_path = match command_path.len() {
        1 => &command_path,
        2 => &command_path[1..],
        _ => &command_path[2..],
    };
    let mut display_name = display_path.join(" ");
    if command_path.len() == 1 {
        display_name = format!("**{}**", display_name);
    }
    let mut result = Vec::new();
    result.push(format!(
        "{}- [{}]({})",
        " ".repeat(INDENT_SIZE * depth),
        display_name,
        doc_path
    ));
    for subcommand in command.get_subcommands() {
        result.push(get_command_sidebar_list(
            subcommand,
            &command_path,
            depth + 1,
        ))
    }
    result.join("\n")
}

fn update_sidebar(sidebar_path: &Path) -> Result<()> {
    // sort lexographically
    let result = get_command_sidebar_list(&BonsaiCli::command(), &Vec::new(), 0);
    let mut file = File::create(sidebar_path)?;
    writeln!(file, "{result}")?;
    Ok(())
}

fn write_position_arguments(cmd: &Command, output_file: &mut impl Write) -> Result<()> {
    let positional_args = cmd
        .get_arguments()
        .filter(|x| x.is_positional())
        .collect::<Vec<_>>();
    if positional_args.is_empty() {
        return Ok(());
    }
    writeln!(output_file, "### Positional Arguments")?;
    for arg in positional_args {
        let names = if let Some(value_names) = arg.get_value_names() {
            value_names
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        writeln!(output_file, "- {}", names.join(", "))?;
        writeln!(
            output_file,
            "{}",
            arg.get_long_help()
                .or(arg.get_help())
                .map(|help| help.to_string())
                .map(|help| format!("  - {}", help))
                .unwrap_or(String::new())
        )?;
    }
    Ok(())
}

fn write_flags(cmd: &Command, global: bool, output_file: &mut impl Write) -> Result<()> {
    let flags = cmd
        .get_arguments()
        .filter(|arg| !arg.is_positional() && arg.is_global_set() == global)
        .collect::<Vec<_>>();
    if flags.is_empty() {
        return Ok(());
    }
    let initial_header =
        flags[0]
            .get_help_heading()
            .unwrap_or(if global { "Global Options" } else { "Options" });
    writeln!(output_file, "### {}", initial_header)?;
    for flag in flags {
        let names = [
            flag.get_short().map(|name| format!("`-{name}`")),
            flag.get_long().map(|name| format!("`--{name}`")),
        ]
        .iter()
        .filter_map(|item| item.clone())
        .collect::<Vec<_>>()
        .join(", ");
        writeln!(output_file, "- {names}")?;
        writeln!(
            output_file,
            "{}",
            flag.get_long_help()
                .or(flag.get_help())
                .map(|help| help.to_string())
                .map(|help| format!("  - {}", help))
                .unwrap_or(String::new())
        )?;
    }
    Ok(())
}

fn write_options(cmd: &Command, output_file: &mut impl Write) -> Result<()> {
    writeln!(output_file, "<div id=\"commandOptions\">\n")?;
    write_position_arguments(cmd, output_file)?;
    write_flags(cmd, false, output_file)?;
    write_flags(&BonsaiCli::command(), true, output_file)?;
    writeln!(output_file, "\n</div>")?;
    Ok(())
}
