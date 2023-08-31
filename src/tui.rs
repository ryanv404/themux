// tui.rs: TUI for theme selection.

use std::process::{Command, ExitCode};

use inquire::{error::InquireError, Select};
use terminal_size::{terminal_size, Height, Width};

use crate::data::{Theme, Themes};

pub struct Tui;

impl Tui {
    pub fn run() -> ExitCode {
        match Self::get_selection() {
            Ok(new_theme) => Theme::apply(&new_theme),
            Err(code) => {
                Self::clear();
                code
            }
        }
    }

    fn fail(msg: &str) -> ExitCode {
        eprintln!("[-] {msg}");
        ExitCode::FAILURE
    }

    fn clear() {
        Command::new("clear")
            .status()
            .expect("Error while clearing the terminal.");
    }

    pub fn reload_termux() -> ExitCode {
        Command::new("termux-reload-settings")
            .status()
            .expect("Error while reloading Termux settings.");

        Self::clear();
        ExitCode::SUCCESS
    }

    pub fn get_size() -> (usize, usize) {
        // Get the terminal's size or default to sensible values
        match terminal_size() {
            Some((Width(w), Height(h))) => (w.into(), h.into()),
            _ => (60, 10),
        }
    }

    pub fn get_selection() -> Result<String, ExitCode> {
        let themes = Themes::init();

        let options = themes.0.keys().collect::<Vec<&String>>();

        let (_, height) = Self::get_size();

        // Adjust height to accommodate prompt and help message.
        let size = height - 3;

        let res: Result<&String, InquireError> = Select::new("Select a theme:", options)
            .with_help_message("↑↓ to move, type to filter, ENTER to select, ESC to exit")
            .with_page_size(size)
            .prompt();

        match res {
            Ok(name) => match themes.0.get(name) {
                Some(theme) => Ok(theme.to_file_format(name)),
                None => Err(Self::fail("Unable to locate the selected theme.")),
            },
            Err(_) => Err(ExitCode::FAILURE),
        }
    }
}
