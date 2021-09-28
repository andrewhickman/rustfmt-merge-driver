use std::{
    path::PathBuf,
    process::{exit, Command},
};

use anyhow::{bail, format_err, Result};
use clap::{AppSettings, Clap};
use which::which;

/// A git merge driver which runs rustfmt before attempting to merge.
#[derive(Clap)]
#[clap(global_setting = AppSettings::ColoredHelp)]
#[clap(global_setting = AppSettings::DisableVersionFlag)]
pub struct Args {
    /// Path to the current version of the file.
    current_file: PathBuf,
    /// Path to the common ancestor of the files.
    base_file: PathBuf,
    /// Path to the incoming version of the file.
    other_file: PathBuf,
    /// The marker size to use for conflicts.
    #[clap(long)]
    marker_size: Option<u64>,
}

fn main() -> ! {
    let args = Args::parse();

    if let Err(err) = run_rustfmt(&args) {
        eprintln!("Error running rustfmt: {:#}", err);
    }

    match run_default_merge_driver(&args) {
        Ok(exit_code) => exit(exit_code),
        Err(err) => {
            eprintln!("Error running git merge-file: {:#}", err);
            exit(-1);
        }
    }
}

fn run_rustfmt(args: &Args) -> Result<()> {
    let rustfmt_path = match which("rustfmt") {
        Ok(path) => path,
        Err(which::Error::CannotFindBinaryPath) => bail!("failed to find rustfmt command"),
        Err(err) => return Err(format_err!(err).context("failed to find rustfmt command")),
    };

    let mut command = Command::new(rustfmt_path);

    command
        .arg(&args.current_file)
        .arg(&args.base_file)
        .arg(&args.other_file);

    let status = command.status()?;
    if !status.success() {
        bail!("rustfmt exited unsuccessfully: {}", status);
    }
    Ok(())
}

fn run_default_merge_driver(args: &Args) -> Result<i32> {
    let git_path = match which("git") {
        Ok(path) => path,
        Err(which::Error::CannotFindBinaryPath) => bail!("failed to find git command"),
        Err(err) => return Err(format_err!(err).context("failed to find git command")),
    };

    let mut command = Command::new(git_path);

    command.arg("merge-file");

    command
        .arg("-L")
        .arg("current")
        .arg("-L")
        .arg("base")
        .arg("-L")
        .arg("incoming");

    if let Some(marker_size) = args.marker_size {
        command.arg(format!("--marker-size={}", &marker_size));
    }

    command
        .arg(&args.current_file)
        .arg(&args.base_file)
        .arg(&args.other_file);

    let status = command.status()?;
    Ok(status.code().unwrap_or(-1))
}
