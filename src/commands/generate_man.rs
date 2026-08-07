use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use crate::{
    cli::{BonsaiCli, GenerateManArgs},
    util::example::ToExamples,
};
use clap::{Command, CommandFactory};
use clap_mangen::{
    Man,
    roff::{Roff, roman},
};
use miette::{Diagnostic, IntoDiagnostic, Result};
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
    if !out.exists() {
        fs::create_dir_all(&out).map_err(|_| GenerateManError::NoAccess)?;
    }
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
        let exists = fs::exists(file_path).map_err(|_| GenerateManError::NoAccess)?;
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
        man.render_with_examples(&command, &mut file)?
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

trait RenderWithExamples {
    fn render_with_examples(&self, cmd: &Command, w: &mut dyn io::Write) -> Result<()>;

    fn render_examples_section(&self, cmd: &Command, roff: &mut dyn Write) -> Result<()>;
}

impl RenderWithExamples for Man {
    fn render_with_examples(&self, cmd: &Command, w: &mut dyn io::Write) -> Result<()> {
        self.render_title(w).into_diagnostic()?;
        self.render_name_section(w).into_diagnostic()?;
        self.render_synopsis_section(w).into_diagnostic()?;
        self.render_description_section(w).into_diagnostic()?;

        if app_has_arguments(cmd) {
            self.render_options_section(w).into_diagnostic()?;
        }

        if app_has_subcommands(cmd) {
            self.render_subcommands_section(w).into_diagnostic()?;
        }

        // this is where it starts to differ... extract the examples
        if cmd.get_after_long_help().is_some() || cmd.get_after_help().is_some() {
            self.render_examples_section(cmd, w)?;
        }

        if app_has_version(cmd) {
            self.render_version_section(w).into_diagnostic()?;
        }

        if cmd.get_author().is_some() {
            self.render_authors_section(w).into_diagnostic()?;
        }
        Ok(())
    }

    fn render_examples_section(&self, cmd: &Command, w: &mut dyn Write) -> Result<()> {
        let mut roff = Roff::default();
        roff.control("SH", ["EXAMPLES"]);
        let examples = cmd.get_examples().into_diagnostic()?;
        for example in examples {
            let mut result = vec![Vec::new()];
            let mut is_bolding = true;
            let mut last_part_arg = false;
            for mut part in shlex::split(&example.usage).unwrap() {
                if part.contains(' ') {
                    part = format!("\"{part}\"");
                }

                if part.starts_with("--") {
                    if !is_bolding {
                        result.last_mut().unwrap().push(String::new());
                        result.push(Vec::new());
                    }
                    is_bolding = true;
                    last_part_arg = true;
                } else if part.starts_with("-") {
                    if !is_bolding {
                        result.last_mut().unwrap().push(String::new());
                        result.push(Vec::new());
                    }
                    is_bolding = true;
                    last_part_arg = false;
                } else if last_part_arg {
                    if is_bolding {
                        result.last_mut().unwrap().push(String::new());
                        result.push(Vec::new());
                    }
                    is_bolding = false;
                    last_part_arg = false;
                } else {
                    if !is_bolding {
                        result.last_mut().unwrap().push(String::new());
                        result.push(Vec::new());
                    }
                    is_bolding = true;
                    last_part_arg = false;
                }
                result.last_mut().unwrap().push(part);
            }
            let result = result
                .into_iter()
                .map(|item| item.join(" "))
                .collect::<Vec<_>>();
            let elements = result.iter().map(|item| item.as_ref());
            roff.control("TP", []);
            roff.control("BI", elements);
            let description = roman(example.description);
            roff.text([description]);
        }
        roff.to_writer(w).into_diagnostic()?;
        Ok(())
    }
}

// Does the application have a version?
fn app_has_version(cmd: &clap::Command) -> bool {
    cmd.get_version()
        .or_else(|| cmd.get_long_version())
        .is_some()
}

// Does the application have any command line arguments?
fn app_has_arguments(cmd: &clap::Command) -> bool {
    cmd.get_arguments().any(|i| !i.is_hide_set())
}

// Does the application have any subcommands?
fn app_has_subcommands(cmd: &clap::Command) -> bool {
    cmd.get_subcommands().any(|i| !i.is_hide_set())
}
