use std::io::{self, Write};
use std::process::{Command, ExitCode};

use inquire::{error::InquireResult, Select};
use terminal_size::{terminal_size, Height, Width};

use crate::style::{Themes, CLR, RED};
use crate::util::fail;

/// A type containing methods for handling the TUI.
pub struct Tui;

impl Tui {
    /// Runs the TUI theme selector.
    pub fn run() -> ExitCode {
        let themes = Themes::init();

        let options = themes.0
            .keys()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        let name = match Self::get_selection(options) {
            Ok(Some(name)) => name,
            Ok(None) => return ExitCode::SUCCESS,
            Err(e) => return fail(&format!("{RED}{e}{CLR}")),
        };

        let Some(theme) = themes.get(name) else {
            return fail(&format!("{RED}Unable to apply the theme.{CLR}"));
        };

        theme.apply(name)
    }

    pub fn clear_screen() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(b"\x1b[2J\x1b[1;1H\r").unwrap();
        stdout.flush().unwrap();
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

    /// Returns the user's theme selection.
    pub fn get_selection(options: Vec<&str>) -> InquireResult<Option<&str>> {
        let prompt = "Select a theme:";

        let height = Self::get_terminal_size().1;
        let page_size = height - 2;

        Select::new(prompt, options)
            .without_help_message()
            .with_page_size(page_size)
            .prompt_skippable()
    }
}
