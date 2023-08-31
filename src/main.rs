// main.rs: Terminal color theme selection tool.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

use std::{env, process::ExitCode};

mod cli;
mod data;
mod tui;

use crate::cli::Cli;
use crate::tui::Tui;

fn main() -> ExitCode {
    let mut args = env::args();

    match args.len() {
        1 => Tui::run(),
        2 => Cli::parse_opts(&mut args),
        _ => Cli::fail("Invalid number of arguments."),
    }
}
