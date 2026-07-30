use anyhow::{Context, Result, bail};
use clap::Parser;
use git2::Repository;
use log::{debug, error, info};

use crate::cli::Arguments;
use crate::commits::Commits;

mod cli;
mod commits;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let arguments = cli::Arguments::parse();

    // Set up logging. Log level precedence:
    // - RUST_LOG, if set.
    // - info, if --verbose is passed.
    let mut logger = pretty_env_logger::formatted_builder();
    match std::env::var("RUST_LOG") {
        Ok(rust_log) => {
            logger.parse_filters(&rust_log);
        }
        Err(_) if arguments.verbose => {
            logger.filter_level(log::LevelFilter::Info);
        }
        Err(_) => {}
    }
    logger.init();

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
