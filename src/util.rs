use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Prints an error message to stderr and returns `ExitCode::FAILURE`.
pub fn fail(msg: &str) -> ExitCode {
    eprintln!("{msg}");

    ExitCode::FAILURE
}

/// Returns the path to the color settings file path.
pub fn get_settings_file_path() -> Result<PathBuf, &'static str> {
    let Ok(home) = env::var("HOME") else {
        return Err("HOME environment variable must be set.");
    };

    let mut path = Path::new(&home).join(".termux");
    path.push("colors.properties");

    if matches!(path.try_exists(), Ok(true)) {
        return Ok(path);
    }

    match path.parent() {
        // Will create the file on write.
        Some(termux_dir) if termux_dir.is_dir() => Ok(path),
        Some(_) | None => Err("Color settings file cannot be created."),
    }
}
