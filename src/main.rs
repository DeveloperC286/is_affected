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
    let arguments = cli::Arguments::parse();

    // Set up logging: if verbose is true and RUST_LOG is not set, default to info level
    if arguments.verbose && std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init();

    info!("Version {}.", env!("CARGO_PKG_VERSION"));
    debug!("The command line arguments provided are {arguments:?}.");

    if let Err(err) = run(arguments) {
        error!("{err:?}");
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<()> {
    let repository = Repository::open_from_env().context("Unable to open the Git repository.")?;
    let commits = Commits::from_git(&repository, arguments.from)?;

    match (
        arguments.list,
        arguments.affects_current_directory,
        arguments.affects.len(),
    ) {
        (true, false, 0) => {
            commits
                .get_affected_resources()
                .iter()
                .for_each(|affected_resource| println!("{affected_resource}"));
            Ok(())
        }
        (false, true, 0) => {
            let current_directory_prefix = get_current_directory_prefix(&repository)?;
            debug!(
                "Checking if the current directory prefix {current_directory_prefix:?} is affected."
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
        _ => Ok(format!("^{stripped}/")),
    }
}
