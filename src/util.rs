use std::env;
use std::path::{Path, PathBuf};

/// Returns the path to the color settings file path.
pub fn get_settings_file_path() -> Result<PathBuf, &'static str> {
    let Ok(home) = env::var("HOME") else {
        return Err("HOME environment variable is not set.");
    };

    let mut path = Path::new(&home).join(".termux");
    path.push("colors.properties");

    if matches!(path.try_exists(), Ok(true)) {
        return Ok(path);
    }

    match path.parent() {
        // Termux dir exists so we will create the file on write.
        Some(termux_dir) if termux_dir.is_dir() => Ok(path),
        Some(_) | None => Err("Color settings file cannot be found or created."),
    }
}
