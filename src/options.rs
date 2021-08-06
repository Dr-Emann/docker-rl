//! Options for CLI

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// gets ratelimit from docker hub
pub struct Opts {
    #[structopt(short, long, about = "user for basic authentication")]
    pub user: Option<String>,

    #[structopt(
        short,
        long,
        about = "password for basic authentication",
        requires("user")
    )]
    pub pass: Option<String>,
}

impl Opts {
    /// Parses arguments and returns `Opts` struct
    pub fn parse_args() -> Opts {
        Opts::from_args()
    }
}
