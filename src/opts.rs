use std::ffi::OsString;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "meas")]
pub struct Opt {
    /// Command to run including its arguments
    #[structopt(name = "command")]
    pub command: Vec<OsString>,
}

pub fn parse_args() -> Opt {
    Opt::from_args()
}

pub fn print_help() {
    Opt::clap().print_help().unwrap();
}
