#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use anyhow::{bail, Context, Result};
use clap::Parser;
use git2::Repository;

use crate::cli::Arguments;
use crate::commits::Commits;

mod cli;
mod commits;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::parse();
    trace!("The command line arguments provided are {:?}.", arguments);

    if let Err(err) = run(arguments) {
        error!("{:?}", err);
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<()> {
    let repository = Repository::open_from_env().context(
        "Failed to open a Git repository from the current directory or Git environment variables.",
    )?;

    let commits = match (arguments.from_commit_hash, arguments.from_reference) {
        (Some(from_commit_hash), None) => Commits::from_commit_hash(&repository, from_commit_hash),
        (None, Some(from_reference)) => Commits::from_reference(&repository, from_reference),
        (_, _) => {
            bail!("Invalid combination of arguments.");
        }
    }?;

    match (
        arguments.list,
        arguments.affects_current_directory,
        arguments.affects.len(),
    ) {
        (true, false, 0) => {
            commits
                .get_affected_resources()
                .iter()
                .for_each(|affected_resource| println!("{}", affected_resource));
            Ok(())
        }
        (false, true, 0) => {
            let current_directory_prefix = get_current_directory_prefix(&repository)?;
            trace!(
                "Checking if the current directory prefix {:?} is affected.",
                current_directory_prefix
            );
            let affects: Vec<String> = vec![current_directory_prefix];
            match commits.is_affected(&affects) {
                Ok(true) => Ok(()),
                _ => bail!("Unaffected."),
            }
        }
        (false, false, 0) => {
            bail!("Unsupported configuration of output arguments.");
        }
        (false, false, _) => match commits.is_affected(&arguments.affects) {
            Ok(true) => Ok(()),
            _ => bail!("Unaffected."),
        },
        (_, _, _) => {
            bail!("Unsupported configuration of output arguments.");
        }
    }
}

fn get_current_directory_prefix(repository: &Repository) -> Result<String> {
    let mut repository_path = repository.path().to_path_buf();
    // Removing the ".git/" at the end.
    repository_path.pop();

    let current_directory = std::env::current_dir()?;
    let stripped = current_directory
        .strip_prefix(repository_path)
        .context("Can not strip the repositories path from the current directory.")?;
    let stripped = stripped
        .to_str()
        .context("Can not convert the current directory prefix into a string.")?;
    match stripped.len() {
        0 => {
            bail!("The current directory prefix is empty.");
        }
        _ => Ok(format!("^{}/", stripped)),
    }
}
