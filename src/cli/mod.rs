use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("from")
        .required(true)
))]
#[clap(group(
    ArgGroup::new("output")
        .required(true)
))]
pub(crate) struct Arguments {
    #[arg(
        long,
        group = "from",
        help = "The Git commit hash from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided Git commit hash."
    )]
    pub(crate) from_commit_hash: Option<String>,

    #[arg(
        long,
        group = "from",
        help = "The Git reference from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub(crate) from_reference: Option<String>,

    #[arg(
        long,
        group = "output",
        help = "Check if any of the affected resources within the range of the commits match any of these regexes. If any match then exit with a zero status code, otherwise exit with a non-zero status code."
    )]
    pub(crate) affects: Vec<String>,

    #[arg(
        long,
        group = "output",
        help = "Check if the current directory contains any of the affected resources within the range of the commits. If any match then exit with a zero status code, otherwise exit with a non-zero status code."
    )]
    pub(crate) affects_current_directory: bool,

    #[arg(
        long,
        group = "output",
        help = "List all the the affected resources within the range of the commits."
    )]
    pub(crate) list: bool,
}
