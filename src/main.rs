// main.rs: Terminal color theme selection tool.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

mod binary;
mod cli;
mod data;
mod tui;

use crate::cli::Cli;

fn main() -> std::process::ExitCode {
    Cli::parse()
}
