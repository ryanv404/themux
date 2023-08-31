// cli.rs: Command-line options processing.

use std::{env::Args, process::ExitCode};

use crate::data::Themes;

pub const USAGE_MSG: &str = "\
    Usage:\n    \
        themux\n    \
        themux [OPTION]\n\n\
    Options:\n    \
    	-h, --help    Print this help message.\n    \
    	-t, --themes  List the available themes.";

pub struct Cli;

impl Cli {
    fn help() -> ExitCode {
        eprintln!("{USAGE_MSG}");
        ExitCode::SUCCESS
    }

    pub fn fail(msg: &str) -> ExitCode {
        eprintln!("[-] {msg}");
        ExitCode::FAILURE
    }

    pub fn parse_opts(args: &mut Args) -> ExitCode {
        match args.nth(1).as_deref() {
            Some("-h" | "--help") => Self::help(),
            Some("-t" | "--themes") => Themes::list(),
            Some(opt) => Self::fail(&format!("Invalid option: `{opt}`")),
            _ => Self::fail("Unknown option."),
        }
    }
}
