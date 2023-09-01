// cli.rs: Command-line options processing.

use std::{env::{self, Args}, process::ExitCode};

use crate::data::Themes;
use crate::tui::Tui;

pub const USAGE_MSG: &str = "\
    Usage: themux [OPTION] <COMMAND>\n\n\
    Commands:\n    \
        run      Launch the theme selector.\n    \
        list     List the available themes.\n\n\
    Options:\n    \
    	-h, --help\n             \
                 Print this help message.\n    \
    	-p, --print <THEME>\n             \
                 Print the color values for a given THEME.\n";

pub struct Cli;

impl Cli {
    pub fn parse() -> ExitCode {
        let mut args = env::args();

        match args.nth(1).as_deref() {
            Some("-h" | "--help") | None => Self::help(),
            Some("list") => Themes::list(),
            Some("-p" | "--print") => Self::get_opt_arg(args),
            Some("run") => Tui::run(),
            Some(opt) => Self::invalid_opt(opt)
        }
    }

    fn get_opt_arg(args: Args) -> ExitCode {
        let Some(key) = args.reduce(|acc, s| format!("{acc} {s}")) else {
            return Self::fail("Missing required argument.")
        };

        let themes = Themes::init();

        // All keys are lowercase so convert search term to lowercase.
        let needle = key.as_str().to_lowercase();

        if let Some(theme) = themes.0.get(&needle) {
            println!("[*] {needle}\n{theme}");
            ExitCode::SUCCESS
        } else {
            eprintln!("[-] No theme by the name \"{key}\".");
            ExitCode::FAILURE
        }
    }

    fn help() -> ExitCode {
        eprintln!("{USAGE_MSG}");
        ExitCode::SUCCESS
    }

    fn fail(msg: &str) -> ExitCode {
        eprintln!("[-] {msg}");
        ExitCode::FAILURE
    }

    fn invalid_opt(opt: &str) -> ExitCode {
        eprintln!("[-] Invalid option: `{opt}`");
        ExitCode::FAILURE
    }
}
