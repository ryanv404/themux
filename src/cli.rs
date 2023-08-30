use std::{
    env, fs,
    process::{Command, ExitCode},
};

use crate::data::Themes;

// Standard location of the Termux color settings file.
const TERMUX_COLORS_FILE: &str = "/data/data/com.termux/files/home/.termux/colors.properties";

const USAGE_MSG: &str = "\
    Usage: themux [OPTION]\n\n\
    Options:\n    \
    	-h, --help    Print this help message.\n    \
    	-l, --list    List all available themes.\n    \
    	-t, --theme   Launch the theme selector.\n    \
    	-v, --values  Print the default color values for all themes.";

pub fn handle_cli_opts() -> ExitCode {
    let mut args = env::args();

    match args.nth(1).as_deref() {
        Some("-h" | "--help") => {
            eprintln!("{USAGE_MSG}");
            ExitCode::SUCCESS
        }
        Some("-l" | "--list") => {
            let themes = Themes::init();
            themes.list_themes();
            ExitCode::SUCCESS
        }
        Some("-t" | "--theme") => {
            let (name, new_theme) = match Themes::get_selection() {
                Ok((name, theme)) => (name.clone(), theme.to_file_format(&name)),
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::FAILURE;
                }
            };

            match fs::write(TERMUX_COLORS_FILE, new_theme) {
                Ok(_) => {
                    Command::new("termux-reload-settings")
                        .status()
                        .expect("Error while reloading Termux settings.");

                    Command::new("clear")
                        .status()
                        .expect("Error while clearing the terminal.");

                    println!("[+] The terminal theme is now {name}.");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("{e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("-v" | "--values") => {
            let themes = Themes::init();
            println!("{themes}");
            ExitCode::SUCCESS
        }
        Some(opt) => {
            eprintln!("[-] Invalid option: `{opt}`");
            ExitCode::FAILURE
        }
        None => {
            eprintln!("{USAGE_MSG}");
            ExitCode::FAILURE
        }
    }
}
