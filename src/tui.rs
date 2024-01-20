use std::process::{Command, ExitCode};

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use terminal_size::{terminal_size, Width};

use crate::style::Themes;

/// A type containing methods for handling the theme selection TUI.
pub struct Tui;

impl Tui {
    /// Runs the TUI theme selector.
    pub fn get_selection() -> ExitCode {
        let themes = Themes::init();

        let names = themes
            .0
            .iter()
            .map(|theme| theme.name)
            .collect::<Vec<&str>>();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .default(0)
            .report(false)
            .highlight_matches(false)
            .with_prompt("Select a theme:")
            .items(&names[..])
            .interact_opt();

        let name = match selection {
            Ok(Some(idx)) => names[idx],
            // User pressed 'q' or 'ESC'.
            Ok(None) => return ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Error: {e}");
                return ExitCode::FAILURE;
            }
        };

        if let Some(theme) = themes.get(name) {
            if let Err(e) = theme.apply() {
                eprintln!("Error: Unable to apply the theme.\n{e}");
                return ExitCode::FAILURE;
            }

            return Self::reload_termux_settings();
        }

        eprintln!("Error: Unable to apply the theme.");
        ExitCode::FAILURE
    }

    // Reset Termux setting using the `termux-reload-settings` command.
    fn reload_termux_settings() -> ExitCode {
        if let Err(e) = Command::new("termux-reload-settings").status() {
            eprintln!("Error: Unable to reload Termux settings.\n{e}");
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    /// Returns the terminal's width or 60 if the width cannot be determined.
    pub fn get_terminal_width() -> usize {
        if let Some((Width(w), _)) = terminal_size() {
            w.into()
        } else {
            60
        }
    }
}
