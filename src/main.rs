#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use structopt::StructOpt;

use crate::model::commits::Commits;
use std::process::exit;

mod cli;
mod model;
mod utilities;

const ERROR_EXIT_CODE: i32 = 1;
const SUCCESSFUL_EXIT_CODE: i32 = 0;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    let commits = if let Some(commit_hash) = arguments.from_commit_hash {
        Commits::from_git_commit_hash(commit_hash)
    } else if let Some(reference) = arguments.from_reference {
        Commits::from_git_reference(reference)
    } else {
        unreachable!();
    };

    match commits.is_effected(&arguments.effects) {
        true => exit(SUCCESSFUL_EXIT_CODE),
        false => exit(ERROR_EXIT_CODE),
    }
}
