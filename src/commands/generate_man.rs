use std::{
    fs::{self, OpenOptions},
    io,
    path::PathBuf,
};

use crate::cli::{BonsaiCli, GenerateManArgs};
use clap::{Command, CommandFactory};
use clap_mangen::Man;
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum GenerateManError {
    #[error("Man pages with those names already exist")]
    #[diagnostic(
        code(bonsai::generate_man::already_exist),
        help("Run with `--force` to overwrite existing contents")
    )]
    ManPagesAlreadyExist,
    #[error("Insufficient permissions to create directory")]
    #[diagnostic(
        code(bonsai::generate_man::no_access),
        help("Run the command with sudo")
    )]
    NoAccess,
}

pub fn handle_command(GenerateManArgs { force, out }: GenerateManArgs) -> Result<()> {
    fs::create_dir_all(&out).map_err(|_| GenerateManError::NoAccess)?;
    generate_man_page_for_command_and_subcommands(&out, BonsaiCli::command(), None, force, true)?;
    generate_man_page_for_command_and_subcommands(&out, BonsaiCli::command(), None, force, false)?;
    Ok(())
}

fn generate_man_page_for_command_and_subcommands(
    path: &PathBuf,
    command: Command,
    prefix: Option<&str>,
    force: bool,
    dry_run: bool,
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
    if dry_run {
        let exists = fs::exists(path).map_err(|_| GenerateManError::NoAccess)?;
        if exists && !force {
            return Err(GenerateManError::ManPagesAlreadyExist.into());
        }
    } else {
        let mut open_options = OpenOptions::new();
        open_options.write(true);
        if force {
            open_options.create(true).truncate(true);
        } else {
            open_options.create_new(true);
        }
        let mut file = open_options.open(file_path).map_err(|e| {
            if e.kind() == io::ErrorKind::AlreadyExists {
                GenerateManError::ManPagesAlreadyExist
            } else {
                GenerateManError::NoAccess
            }
        })?;
        let man = Man::new(command.clone());
        man.render(&mut file)
            .map_err(|_| GenerateManError::NoAccess)?;
    }
    for subcommand in command.get_subcommands() {
        generate_man_page_for_command_and_subcommands(
            path,
            subcommand.clone(),
            Some(&filename_without_ext),
            force,
            dry_run,
        )?;
    }
    Ok(())
}
