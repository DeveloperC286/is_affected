#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use structopt::StructOpt;

use crate::model::commits::Commits;
use std::process::exit;

mod cli;
mod model;
mod utilities;

const SUCCESSFUL_EXIT_CODE: i32 = 0;
const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    let repository = match crate::utilities::git::get_repository() {
        Ok(repository) => repository,
        Err(()) => {
            exit(ERROR_EXIT_CODE);
        }
    };

    let commits = match (arguments.from_commit_hash, arguments.from_reference) {
        (Some(commit_hash), None) => Commits::from_git_commit_hash(&repository, commit_hash),
        (None, Some(reference)) => Commits::from_git_reference(&repository, reference),
        (_, _) => {
            error!("Unsupported configuration of from arguments.");
            exit(ERROR_EXIT_CODE);
        }
    };

    match (
        arguments.list,
        arguments.effects_current_directory,
        arguments.effects.len(),
    ) {
        (true, false, 0) => {
            commits
                .get_effected_resources()
                .iter()
                .for_each(|effected_resource| println!("{}", effected_resource));
        }
        (false, true, 0) => {
            match crate::utilities::git::get_current_directory_prefix(&repository) {
                Ok(current_directory_prefix) => {
                    trace!(
                        "Checking if the current directory prefix {:?} is effected",
                        current_directory_prefix
                    );
                    let effects: Vec<String> = vec![current_directory_prefix];
                    match commits.is_effected(&effects) {
                        Ok(true) => exit(SUCCESSFUL_EXIT_CODE),
                        _ => exit(ERROR_EXIT_CODE),
                    }
                }
                Err(()) => {
                    exit(ERROR_EXIT_CODE);
                }
            }
        }
        (false, false, 0) => {
            error!("Unsupported configuration of output arguments.");
            exit(ERROR_EXIT_CODE);
        }
        (false, false, _) => match commits.is_effected(&arguments.effects) {
            Ok(true) => exit(SUCCESSFUL_EXIT_CODE),
            _ => exit(ERROR_EXIT_CODE),
        },
        (_, _, _) => {
            error!("Unsupported configuration of output arguments.");
            exit(ERROR_EXIT_CODE);
        }
    }
}
