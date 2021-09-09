use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "is_effected",
    about = "A utility to check if a particular file/directory has been effected within a range of commits. Useful for monorepos to check sub-repositories.",
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
        help = "The regex for the resources to check if they are effected within the range of the commits."
    )]
    pub effects: Vec<String>,

    #[structopt(
        long,
        group = "output",
        help = "List all the the resources effected within the range of the commits."
    )]
    pub list: bool,
}
