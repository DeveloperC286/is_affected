use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "is_effected",
    about = "A utility for checking and listing the effected resources across a range of commits, useful when working with monorepos.",
    group = ArgGroup::with_name("from").required(true),
    group = ArgGroup::with_name("output").required(true),
)]
pub struct Arguments {
    #[structopt(
        long,
        group = "from",
        help = "The Git commit hash from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided Git commit hash."
    )]
    pub from_commit_hash: Option<String>,

    #[structopt(
        long,
        group = "from",
        help = "The Git reference from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub from_reference: Option<String>,

    #[structopt(
        long,
        group = "output",
        help = "Check if any of the effected resources within the range of the commits match any of these regexes. If any match then exit with a zero status code, otherwise exit with a non-zero status code."
    )]
    pub effects: Vec<String>,

    #[structopt(
        long,
        group = "output",
        help = "Check if the current directory contains any of the effected resources within the range of the commits. If any match then exit with a zero status code, otherwise exit with a non-zero status code."
    )]
    pub effects_current_directory: bool,

    #[structopt(
        long,
        group = "output",
        help = "List all the the effected resources within the range of the commits."
    )]
    pub list: bool,
}
