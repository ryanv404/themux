use std::process::{Command, ExitCode};

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::fail;
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
            Err(e) => fail!("{e}"),
        };

        if let Some(theme) = themes.get(name) {
            if let Err(e) = theme.apply() {
                fail!("{e}");
            }

            return Self::reload_termux_settings();
        }

        fail!("Unable to apply the theme");
    }

    // Reset Termux setting using the `termux-reload-settings` command.
    fn reload_termux_settings() -> ExitCode {
        if Command::new("termux-reload-settings").status().is_err() {
            fail!("Unable to reload Termux settings");
        }

        ExitCode::SUCCESS
    }
}
