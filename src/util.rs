use std::env;
use std::path::{Path, PathBuf};

/// Prints the provided message to stderr and exits with the value 1.
#[macro_export]
macro_rules! fail {
    ($($t:tt)*) => {{
        use std::io::{self, IsTerminal, Write};
        use std::process;
        use $crate::style::{CLR, RED};

        let mut out = io::stderr().lock();

        let is_term = out.is_terminal();

        writeln!(
            &mut out,
            "{}Error: {}{}",
            if is_term { RED } else { "" },
            format!($($t)*),
            if is_term { CLR } else { "" }
        ).expect("Failed to write to stderr");

        let _ = out.flush();

        process::exit(1);
    }};
}

/// Returns the path to the color settings file path.
pub fn get_settings_file_path() -> Result<PathBuf, &'static str> {
    let Ok(home) = env::var("HOME") else {
        fail!("'HOME' environment variable must be set");
    };

    let mut path = Path::new(&home).join(".termux");
    path.push("colors.properties");

    if matches!(path.try_exists(), Ok(true)) {
        return Ok(path);
    }

    match path.parent() {
        // Termux dir exists so we will create the file on write.
        Some(termux_dir) if termux_dir.is_dir() => Ok(path),
        Some(_) | None => fail!("Color settings file cannot be created"),
    }
}

/// Checks the environment variables for an indication that we are in Termux.
pub fn is_termux_env() -> bool {
    for (var_name, _) in env::vars_os() {
        if let Some(name) = var_name.as_os_str().to_str() {
            if name.contains("TERMUX") {
                return true;
            }
        }
    }

    false
}
