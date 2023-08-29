// main.rs: Themux

mod cli;
mod data;

use crate::cli::handle_cli_opts;

fn main() -> std::process::ExitCode {
    handle_cli_opts()
}
