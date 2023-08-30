use std::env;
use std::fs;
use std::process::{ExitCode, Command};

use crate::data::Themes;

const TERMUX_COLORS: &'static str = "/data/data/com.termux/files/home/.termux/colors.properties";
const USAGE_MSG: &'static str = "\
    Usage: themux [OPTION]\n\n\
    Options:\n    \
    	-d, --data    Print the default color values for all themes.\n    \
    	-h, --help    Print this help message.\n    \
    	-l, --list    List all available themes.\n    \
    	-s, --select  Launch the theme selector.\n";

pub fn handle_cli_opts() -> ExitCode {
	let mut args = env::args();

	match args.nth(1).as_deref() {
		Some("-d") | Some("--data") => {
            let themes = Themes::init();
		    println!("{themes}");
            return ExitCode::SUCCESS;
		},
		Some("-h") | Some("--help") => {
		    eprintln!("{USAGE_MSG}");
            return ExitCode::SUCCESS;
		},
		Some("-l") | Some("--list") => {
		    Themes::init().list_themes();
            return ExitCode::SUCCESS;
		},
		Some("-s") | Some("--select") => {
		    let themes = Themes::init();

		    let (name, new_theme) = match themes.get_selection() {
                Ok((name, theme)) => (name.clone(), theme.to_file_format(&name)),
                Err(e) => {
        			eprintln!("{e}");
                    return ExitCode::FAILURE;
                }
            };

            match fs::write(TERMUX_COLORS, &new_theme) {
                Ok(_) => {
                    Command::new("termux-reload-settings")
                        .status()
                        .expect("Error while reloading Termux settings.");

                    Command::new("clear")
                        .status()
                        .expect("Error while clearing the terminal.");

                    println!("[+] Terminal theme has been changed to {name}.");
                    return ExitCode::SUCCESS;
                },
                Err(e) => {
        			eprintln!("{e}");
                    return ExitCode::FAILURE;
                }
            }
        },
		Some(opt) => {
			eprintln!("Invalid option `{opt}`.");
            return ExitCode::FAILURE;
		},
		None => {
			eprintln!("{USAGE_MSG}");
            return ExitCode::FAILURE;
		}
	}
}
