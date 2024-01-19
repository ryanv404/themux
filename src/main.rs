//! themux
//!
//! A CLI tool for setting the color theme in a Termux terminal emulator.

#![deny(clippy::all)]

use std::env;
use std::process::ExitCode;

mod cli;
mod data;
mod style;
mod tui;
mod util;

use cli::Cli;
use style::{CLR, RED};

fn main() -> ExitCode {
    if !is_termux() {
        eprintln!("{RED}Not a Termux environment.{CLR}");
        return ExitCode::FAILURE;
    }

    Cli::handle_args()
}

fn is_termux() -> bool {
    env::vars_os().any(|(var, _)| {
        if let Some(v) = var.as_os_str().to_str() {
            v.contains("TERMUX")
        } else {
            false
        }
    })
}
