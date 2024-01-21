//! # themux
//!
//! A command-line tool for setting a color theme in a Termux terminal emulator.
//!
//! Run `themux set` to launch an interactive list with fuzzy search capability
//! to select and automatically apply the theme.
//!
//! View the available light themes with `themux light` and the available dark
//! themes with `themux dark`.

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

mod cli;
mod data;
mod style;
mod tui;
mod util;

use cli::Cli;
use util::is_termux_env;

fn main() -> std::process::ExitCode {
    // Exit if not a Termux environment.
    if !is_termux_env() {
        fail!("Not a Termux environment. Exiting.");
    }

    // Parse and handle command-line arguments.
    Cli::handle_args()
}
