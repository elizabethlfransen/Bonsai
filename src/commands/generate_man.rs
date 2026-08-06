use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use crate::cli::{BonsaiCli, GenerateManArgs};
use clap::{Command, CommandFactory};
use clap_mangen::Man;
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum GenerateManError {
    #[error("The target directory already exists")]
    #[diagnostic(
        code(bonsai::generate_man::directory_already_exists),
        help("Run with `--force` to overwrite existing contents")
    )]
    DirectoryAlreadyExists,
    #[error("Insufficient permissions to create directory")]
    #[diagnostic(
        code(bonsai::generate_man::no_access),
        help("Run the command with sudo")
    )]
    NoAccess,
}

pub fn handle_command(GenerateManArgs { force, out }: GenerateManArgs) -> Result<()> {
    let exists = fs::exists(&out).map_err(|_| GenerateManError::NoAccess)?;
    if !force && exists {
        return Err(GenerateManError::DirectoryAlreadyExists.into());
    }
    fs::create_dir_all(&out).map_err(|_| GenerateManError::NoAccess)?;
    generate_man_page_for_command_and_subcommands(&out, BonsaiCli::command(), None)?;
    Ok(())
}

fn generate_man_page_for_command_and_subcommands(
    path: &PathBuf,
    command: Command,
    prefix: Option<&str>,
) -> Result<()> {
    // build the file_name
    let prefix = prefix.map(|x| format!("{x} ")).unwrap_or_default();
    let file_name_prefix = prefix.replace(' ', "-");
    let filename_without_ext = file_name_prefix
        + command
            .get_display_name()
            .unwrap_or_else(|| command.get_name());
    let filename = filename_without_ext.clone() + ".1";
    let file_path = path.join(filename);
    let bin_name = command
        .get_bin_name()
        .or_else(|| command.get_display_name())
        .unwrap_or_else(|| command.get_name());
    let command = command.clone().bin_name(prefix + bin_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .map_err(|_| GenerateManError::NoAccess)?;
    let man = Man::new(command.clone());
    man.render(&mut file)
        .map_err(|_| GenerateManError::NoAccess)?;
    for subcommand in command.get_subcommands() {
        generate_man_page_for_command_and_subcommands(
            path,
            subcommand.clone(),
            Some(&filename_without_ext),
        )?;
    }
    Ok(())
}
