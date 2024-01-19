//! themux
//!
//! A CLI tool for setting the color theme in a Termux terminal emulator.

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![allow(dead_code)]

use std::process::ExitCode;

mod cli;
mod style;
mod tui;
mod util;

use cli::Cli;

fn main() -> ExitCode {
    Cli::handle_args()
}
