use std::process::{Command, ExitCode};

use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use terminal_size::{terminal_size, Height, Width};

use crate::style::{Themes, CLR, RED};
use crate::util::fail;

/// A type containing methods for handling the TUI.
pub struct Tui;

impl Tui {
    /// Runs the TUI theme selector.
    pub fn get_selection() -> ExitCode {
        let themes = Themes::init();

        let names = themes.0
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
            Err(e) => return fail(&format!("{RED}{e}{CLR}")),
        };

        if let Some(theme) = themes.get(name) {
            theme.apply()
        } else {
            fail(&format!("{RED}Unable to apply the theme.{CLR}"))
        }
    }

    pub fn reload_settings() {
        if Command::new("termux-reload-settings").status().is_err() {
            fail(&format!("{RED}Unable to reload Termux settings.{CLR}"));
        }
    }

    /// Returns the terminal's (width, height) or (60, 10) if the size cannot
    /// be determined.
    pub fn get_terminal_size() -> (usize, usize) {
        let Some((Width(w), Height(h))) = terminal_size() else {
            return (60, 10);
        };

        (w.into(), h.into())
    }
}
