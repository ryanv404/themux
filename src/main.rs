// main.rs: Terminal color theme selection tool.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

mod cli;
mod data;
mod tui;
mod utils;

use crate::cli::Cli;

fn main() -> std::process::ExitCode {
    Cli::parse()
}
