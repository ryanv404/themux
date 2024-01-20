use std::env;
use std::fs;
use std::process::ExitCode;

use crate::style::{Themes, CLR, GRN, RED};
use crate::tui::Tui;
use crate::util::{fail, get_settings_file_path};

const PROG_NAME: &str = env!("CARGO_PKG_NAME");
const PROG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A type containing methods used for handling CLI options.
pub struct Cli;

impl Cli {
    /// Handle the CLI arguments.
    pub fn handle_args() -> ExitCode {
        let mut args = env::args().skip(1);

        match args.next().as_deref() {
            // Start the theme selector TUI.
            Some("set") => Tui::get_selection(),
            // Print a list of all available themes to stdout.
            Some("all") => Themes::init().print_all(),
            // Print the current theme.
            Some("current") => Self::print_current_theme(),
            // Print the color value settings for a given theme.
            Some("show") => {
                let arg = args.reduce(|mut acc, s| {
                    acc.push(' ');
                    acc.push_str(s.trim());
                    acc
                });

                if let Some(ref name) = arg {
                    Self::print_settings(name.trim())
                } else {
                    fail(&format!("{RED}Missing argument for 'show'{CLR}"))
                }

            },
            // Print the help message.
            Some("-h" | "--help") | None => Self::print_help(),
            // Print the version.
            Some("-v" | "--version") => Self::print_version(),
            // Invalid option.
            Some(opt) => fail(&format!(
                "{RED}Invalid {}: \"{opt}\"{CLR}",
                if opt.trim_start().starts_with('-') {
                    "option"
                } else {
                    "command"
                }
            )),
        }
    }

    /// Print a help message to stdout.
    pub fn print_help() -> ExitCode {
        println!("\
{GRN}USAGE:{CLR} {PROG_NAME} [OPTION] <COMMAND>\n
{GRN}COMMANDS:{CLR}
    all            Print a list of all available themes.
    current        Print the currently set theme.
    set            Set the theme from an interactive list.
    show <THEME>   Print the color value settings for THEME.\n
{GRN}OPTIONS:{CLR}
    -h, --help     Print this help message and exit.
    -v, --version  Print the version.\n");

        ExitCode::SUCCESS
    }

    /// Print a help message to stdout.
    pub fn print_version() -> ExitCode {
        println!("{PROG_NAME} {PROG_VERSION}");

        ExitCode::SUCCESS
    }

    /// Prints the current theme to stdout.
    pub fn print_current_theme() -> ExitCode {
        let path = match get_settings_file_path() {
            Ok(path) if matches!(path.try_exists(), Ok(true)) => path,
            Ok(_) => {
                return fail(&format!("{RED}Settings file not found.{CLR}"));
            },
            Err(e) => return fail(&format!("{RED}{e}{CLR}")),
        };

        let theme = match fs::read_to_string(path) {
            Ok(theme) => theme,
            Err(e) => return fail(&format!("{RED}{e}{CLR}")),
        };

        theme.lines()
            .find_map(|line| {
                line.trim().strip_prefix("# Color Theme: ")
            })
            .map_or_else(
                || fail(&format!(
                    "{RED}Unable to determine the current theme.{CLR}"
                )),
                |name| {
                    println!("Current theme: {name}");
                    ExitCode::SUCCESS
                }
            )
    }

    /// Prints the color value settings for a given theme.
    pub fn print_settings(name: &str) -> ExitCode {
        if name.is_empty() {
            return fail(&format!(
                "{RED}Missing argument for 'show'{CLR}"
            ));
        }

        if let Some(theme) = Themes::init().get(name) {
            theme.print_values().map_or_else(
                |e| fail(&format!("{RED}{e}{CLR}")),
                |_| ExitCode::SUCCESS
            )
        } else {
            fail(&format!("{RED}\"{name}\" is not a valid theme.{CLR}"))
        }
    }
}
