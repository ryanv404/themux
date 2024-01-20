use std::env;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;

use crate::style::{Themes, CLR, GRN};
use crate::tui::Tui;
use crate::util::get_settings_file_path;

/// A type containing methods used for handling CLI options.
pub struct Cli;

impl Cli {
    /// Handle the CLI arguments.
    pub fn handle_args() -> ExitCode {
        let mut args = env::args().skip(1);

        match args.next().as_deref() {
            // Start the theme selector TUI.
            Some("set") => Tui::get_selection(),
            // Print a list of all dark themes to stdout.
            Some("dark") => Self::print_themes(false, true),
            // Print a list of all light themes to stdout.
            Some("light") => Self::print_themes(true, false),
            // Print a list of all available themes to stdout.
            Some("all") => Self::print_themes(true, true),
            // Print the current theme.
            Some("current") => Self::print_current_theme(),
            // Print the version.
            Some("-v" | "--version") => Self::print_version(),
            // Print the help message.
            Some("-h" | "--help") | None => Self::print_help(),
            // Print the color value settings for a given theme.
            Some("show") => {
                // Remaining args should be the theme name.
                let arg = args.reduce(|mut acc, s| {
                    acc.push(' ');
                    acc.push_str(s.trim());
                    acc
                });

                arg.as_ref().map_or_else(
                    || {
                        eprintln!("Error: Missing argument for 'show'");
                        ExitCode::FAILURE
                    },
                    |name| Self::print_theme_settings(name.trim())
                )
            }
            Some(opt) => {
                if opt.trim_start().starts_with('-') {
                    // Invalid option.
                    eprintln!("Error: Invalid option: \"{opt}\"");
                } else {
                    // Invalid command.
                    eprintln!("Error: Invalid command: \"{opt}\"");
                }

                ExitCode::FAILURE
            }
        }
    }

    // Print the help message to stdout.
    fn print_help() -> ExitCode {
        let mut out = io::stdout().lock();

        let is_term = out.is_terminal();

        writeln!(
            &mut out,
            "\
            {0}USAGE:{1} {2} [OPTION] <COMMAND>\n\n\
            {0}COMMANDS:{1}\n    \
                all            Print a list of all available themes.\n    \
                current        Print the currently set theme.\n    \
                dark           Print a list of all dark themes.\n    \
                light          Print a list of all light themes.\n    \
                set            Set the theme from an interactive list.\n    \
                show <THEME>   Print the color value settings for THEME.\n\n\
            {0}OPTIONS:{1}\n    \
                -h, --help     Print this help message and exit.\n    \
                -v, --version  Print the version.\n",
            if is_term { GRN } else { "" },
            if is_term { CLR } else { "" },
            env!("CARGO_PKG_NAME")
        )
        .unwrap();

        out.flush().unwrap();
        ExitCode::SUCCESS
    }

    // Print the program version to stdout.
    fn print_version() -> ExitCode {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        ExitCode::SUCCESS
    }

    // Prints the current theme to stdout.
    fn print_current_theme() -> ExitCode {
        match get_settings_file_path() {
            Ok(path) if matches!(path.try_exists(), Ok(true)) => match fs::read_to_string(path) {
                Ok(theme) => return Self::find_and_print_name(&theme),
                Err(e) => eprintln!("Error: {e}"),
            },
            Ok(_) => eprintln!("Error: Settings file not found."),
            Err(e) => eprintln!("Error: {e}"),
        }

        ExitCode::FAILURE
    }

    // Parses and prints the theme name from the content of a settings file.
    fn find_and_print_name(theme: &str) -> ExitCode {
        for line in theme.lines() {
            if let Some(name) = line.trim().strip_prefix("# Color Theme: ") {
                println!("Current theme: {name}");
                return ExitCode::SUCCESS;
            }
        }

        eprintln!("Error: Unable to determine the current theme.");
        ExitCode::FAILURE
    }

    // Prints a list of all available themes to stdout.
    fn print_themes(do_light: bool, do_dark: bool) -> ExitCode {
        Themes::init().print(do_light, do_dark).map_or_else(
            |e| {
                eprintln!("Error: {e}");
                ExitCode::FAILURE
            },
            |code| code,
        )
    }

    // Prints the color value settings for a given theme.
    fn print_theme_settings(name: &str) -> ExitCode {
        if name.is_empty() {
            eprintln!("Error: Missing argument for 'show'");
            return ExitCode::FAILURE;
        }

        if let Some(theme) = Themes::init().get(name) {
            if let Err(e) = theme.print_values() {
                eprintln!("Error: {e}");
                return ExitCode::FAILURE;
            }

            return ExitCode::SUCCESS;
        }

        eprintln!("Error: \"{name}\" is not a valid theme.");
        ExitCode::FAILURE
    }
}
