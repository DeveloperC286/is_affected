#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::process::exit;

use git2::Repository;
use is_affected_lib::Commits;
use structopt::StructOpt;

mod cli;

const SUCCESSFUL_EXIT_CODE: i32 = 0;
const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    match Repository::open_from_env() {
        Ok(repository) => {
            let commits = match (arguments.from_commit_hash, arguments.from_reference) {
                (Some(from_commit_hash), None) => {
                    Commits::from_commit_hash(&repository, &from_commit_hash)
                }
                (None, Some(from_reference)) => {
                    Commits::from_reference(&repository, &from_reference)
                }
                (_, _) => {
                    unreachable!(
                        "Invalid combination of from arguments, should have been caught by structopt."
                    );
                }
            };

            match commits {
                Ok(commits) => match (
                    arguments.list,
                    arguments.affects_current_directory,
                    arguments.affects.len(),
                ) {
                    (true, false, 0) => {
                        commits
                            .get_affected_resources()
                            .iter()
                            .for_each(|affected_resource| println!("{}", affected_resource));
                    }
                    (false, true, 0) => match get_current_directory_prefix(&repository) {
                        Ok(current_directory_prefix) => {
                            trace!(
                                "Checking if the current directory prefix {:?} is affected.",
                                current_directory_prefix
                            );
                            let affects: Vec<String> = vec![current_directory_prefix];
                            match commits.is_affected(&affects) {
                                Ok(true) => exit(SUCCESSFUL_EXIT_CODE),
                                _ => exit(ERROR_EXIT_CODE),
                            }
                        }
                        Err(()) => {
                            exit(ERROR_EXIT_CODE);
                        }
                    },
                    (false, false, 0) => {
                        error!("Unsupported configuration of output arguments.");
                        exit(ERROR_EXIT_CODE);
                    }
                    (false, false, _) => match commits.is_affected(&arguments.affects) {
                        Ok(true) => exit(SUCCESSFUL_EXIT_CODE),
                        _ => exit(ERROR_EXIT_CODE),
                    },
                    (_, _, _) => {
                        error!("Unsupported configuration of output arguments.");
                        exit(ERROR_EXIT_CODE);
                    }
                },
                Err(_) => {
                    exit(ERROR_EXIT_CODE);
                }
            }
        }
        Err(_) => {
            error!("Failed to open a Git repository from the current directory or Git environment variables.");
            exit(ERROR_EXIT_CODE);
        }
    }
}

fn get_current_directory_prefix(repository: &Repository) -> Result<String, ()> {
    let mut repository_path = repository.path().to_path_buf();
    // Removing the ".git/" at the end.
    repository_path.pop();

    match std::env::current_dir() {
        Ok(current_directory) => match current_directory.strip_prefix(repository_path) {
            Ok(stripped) => match stripped.to_str() {
                Some(stripped) => match stripped.len() {
                    0 => {
                        error!("The current directory prefix is empty.");
                        Err(())
                    }
                    _ => Ok(format!("^{}/", stripped)),
                },
                None => {
                    error!("Can not convert the current directory prefix into a string.");
                    Err(())
                }
            },
            Err(_) => {
                error!("Can not strip the repositories path from the current directory.");
                Err(())
            }
        },
        Err(error) => {
            error!("{:?}", error);
            Err(())
        }
    }
}
