use std::env::{self, Args};
use std::fs;
use std::iter::Skip;
use std::process::ExitCode;

use crate::style::{Themes, CLR, GRN, RED};
use crate::tui::Tui;
use crate::util::{get_color_settings_file_path, fail};

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
            Some("select") => Tui::run(),
            // Print a list of all available themes to stdout.
            Some("all") => Themes::init().list(),
            // Print the current theme.
            Some("current") => Self::print_current_theme(),
            // Print the color value settings for a given theme.
            Some("settings") => Self::print_color_settings(args),
            // Print the version.
            Some("-v" | "--version") => Self::print_version(),
            // Print the help message.
            Some("-h" | "--help") | None => Self::print_help(),
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
    all               Print a list of all available themes.
    current           Print the currently set theme.
    select            Launch the theme selector.
    settings <THEME>  Print the color value settings for THEME.\n
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
        let path = match get_color_settings_file_path() {
            Ok(path) if matches!(path.try_exists(), Ok(true)) => path,
            Ok(_) => {
                let msg = format!("{RED}Color settings file not found.{CLR}");
                return fail(&msg);
            },
            Err(e) => return fail(&format!("{RED}{e}{CLR}")),
        };

        match fs::read_to_string(path) {
            Ok(theme) => {
                theme
                    .lines()
                    .find_map(|line| {
                        line.trim().strip_prefix("# Color Theme: ")
                    })
                    .map_or_else(
                        || fail(&format!(
                            "{RED}Unable to determine the current theme.{CLR}"
                        )),
                        |name| {
                            println!("{name}");
                            ExitCode::SUCCESS
                        }
                    )
            },
            Err(e) => fail(&format!("{RED}{e}{CLR}")),
        }
    }

    /// Prints the color value settings for a given theme.
    pub fn print_color_settings(args: Skip<Args>) -> ExitCode {
        let mut name = args
            .enumerate()
            .fold(String::with_capacity(100),
                |mut acc, (idx, s)| {
                    if idx != 0 {
                        acc.push(' ');
                    }

                    acc.push_str(s.trim());
                    acc
                }
            );

        if name.is_empty() {
            return fail(&format!(
                "{RED}Missing argument for 'settings'{CLR}"
            ));
        }

        if let Some(theme) = Themes::init().get(&name) {
            println!("{GRN}{name}{CLR}:\n\n{theme}");
            ExitCode::SUCCESS
        } else {
            name.make_ascii_lowercase();
            fail(&format!("{RED}\"{name}\" is not a valid theme.{CLR}"))
        }
    }
}
