#[path = "src/cli.rs"]
mod cli;
use crate::cli::{BonsaiCli, GenerateMarkdownHelpArgs, GlobalOptions};
use clap::{Arg, Args, Command, CommandFactory};
use std::fs::{File, OpenOptions};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/cli.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR");
    let docs_dir = PathBuf::from(manifest_dir).join("docs");
    let commands_dir = docs_dir.join("user/commands");
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
    update_sidebar(&docs_dir.join("_sidebar.md"), &mut generated_commands)?;
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
    writeln!(output_file, "{}", about)?;
    writeln!(output_file)?;
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

fn update_sidebar(sidebar_path: &Path, commands: &mut Vec<(String, Vec<String>)>) -> Result<()> {
    const START_TAG: &'static str = "<!-- !COMMANDS START -->";
    const END_TAG: &'static str = "<!-- !COMMANDS END -->";
    let indent_size = 2;
    let base_identation = 2;
    // sort lexographically
    commands.sort();
    let list = commands
        .iter()
        .map(|(name, path)| {
            format!(
                "{}- <a class=\"cmd\" href=\"#/user/commands/{}\">{}</a>",
                " ".repeat(indent_size * (base_identation + path.len())),
                path.join("/"),
                name,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut sidebar_contents = fs::read_to_string(sidebar_path)?;
    if let (Some(start_idx), Some(end_idx)) = (
        sidebar_contents.find(START_TAG),
        sidebar_contents.find(END_TAG),
    ) {
        let content_start = start_idx + START_TAG.len();
        // trailing space is required for indentation
        sidebar_contents.replace_range(content_start..end_idx, &format!("\n{list}\n    "));
        fs::write(sidebar_path, sidebar_contents)?;
        return Ok(());
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Could not find start and stop tags for commands",
    ))
}
