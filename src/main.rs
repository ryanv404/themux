//! # themux
//!
//! A command-line tool for selecting color themes in a Termux terminal emulator.
//!
//! Contains 247 built-in color themes.
//!
//! Apply a theme to the terminal by running `themux set` and selecting an
//! available theme from an interactive list.

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

fn main() -> std::process::ExitCode {
    // Exit if not a Termux environment.
    if !is_termux_env() {
        eprintln!("Error: Not a Termux environment. Exiting.");
        return std::process::ExitCode::FAILURE;
    }

    // Parse and handle command-line arguments.
    Cli::handle_args()
}

// Checks environment variables for indication that we are in Termux.
fn is_termux_env() -> bool {
    for (var_name, _) in std::env::vars_os() {
        if let Some(name) = var_name.as_os_str().to_str() {
            if name.contains("TERMUX") {
                return true;
            }
        }
    }

    false
}
