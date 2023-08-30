// main.rs: Themux color theme selection tool.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

mod cli;
mod data;

use crate::cli::handle_cli_opts;

fn main() -> std::process::ExitCode {
    handle_cli_opts()
}
