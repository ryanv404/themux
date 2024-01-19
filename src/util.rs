use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{Result as IoResult};
use std::mem;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::string::ToString;

use crate::style::{Rgb, Theme, Themes, CLR, RED};

/// Prints an error message to stderr and returns `ExitCode::FAILURE`.
pub fn fail(msg: &str) -> ExitCode {
    eprintln!("{msg}");
    ExitCode::FAILURE
}

/// Serializes and writes the themes data to a local file.
///
/// This serialization reduces the size of the stored data by 79% (from
/// 76.2kB as JSON to 16.6kB as binary).
pub fn serialize(themes: &Themes) -> ExitCode {
    if let Err(e) = serialize_names(themes) {
        return fail(&format!(
            "{RED}Unable to write themes data.\n{e}{CLR}"
        ));
    }

    if let Err(e) = serialize_themes(themes) {
        return fail(&format!(
            "{RED}Unable to write themes data.\n{e}{CLR}"
        ));
    }

    ExitCode::SUCCESS
}

/// Deserializes a local data file into a `Themes` instance.
pub fn deserialize() -> Themes {
    let names = deserialize_names();

    // Expect exactly 247 theme names.
    assert_eq!(names.len(), 247);

    let themes = deserialize_themes();

    // Expect exactly 247 themes.
    assert_eq!(themes.len(), 247);

    let mut themes_map: BTreeMap<String, Theme> = BTreeMap::new();

    for (name, theme) in names.into_iter().zip(themes.into_iter()) {
        themes_map.insert(name, theme);
    }

    Themes(themes_map)
}

pub fn serialize_names(themes: &Themes) -> IoResult<()> {
    let names = themes.0
        .keys()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    // Expect exactly 247 theme names.
    assert_eq!(names.len(), 247);

    let names_string = names.join(",");

    fs::write("./src/theme_names.csv", names_string)
}

pub fn deserialize_names() -> Vec<String> {
    let names = include_str!("./theme_names.csv");

    names.split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}

pub fn serialize_themes(themes: &Themes) -> IoResult<()> {
    // Initialize array of arrays of bytes to hold the `Themes` data.
    let mut themes_data: [[[u8; 3]; 19]; 247] = [[[0; 3]; 19]; 247];

    // Serialize the each Theme struct to an array of bytes.
    for (idx, (_, theme)) in themes.0.iter().enumerate() {
        let theme_bytes = [
            theme.color0.to_bytes(),
            theme.color1.to_bytes(),
            theme.color2.to_bytes(),
            theme.color3.to_bytes(),
            theme.color4.to_bytes(),
            theme.color5.to_bytes(),
            theme.color6.to_bytes(),
            theme.color7.to_bytes(),
            theme.color8.to_bytes(),
            theme.color9.to_bytes(),
            theme.color10.to_bytes(),
            theme.color11.to_bytes(),
            theme.color12.to_bytes(),
            theme.color13.to_bytes(),
            theme.color14.to_bytes(),
            theme.color15.to_bytes(),
            theme.background.to_bytes(),
            theme.foreground.to_bytes(),
            theme.cursor.to_bytes(),
        ];

        themes_data[idx] = theme_bytes;
    }

    // Flatten the array of arrays into a 1-dimensional array of bytes.
    let serialized = unsafe {
        // 247 `Theme` structs * 19 colors per `Theme` * 3 bytes per color
        // equals 14079 total bytes.
        mem::transmute::<[[[u8; 3]; 19]; 247], [u8; 14079]>(themes_data)
    };

    assert_eq!(serialized.len(), 14079);

    fs::write("./src/theme_colors.bin", serialized)
}

pub fn deserialize_themes() -> Vec<Theme> {
    let mut themes: Vec<Theme> = vec![];

    let bytes = include_bytes!("./theme_colors.bin");

    // Expect to have read exactly 14079 bytes from file.
    assert_eq!(bytes.len(), 14079);

    // 14079 total bytes / 247 themes = 57 bytes per `Theme`.
    for theme in bytes.chunks(57) {
        // 57 bytes / 19 `Theme` colors = 3 bytes per color.
        let colors = theme
            .chunks(3)
            .map(Rgb::from_bytes)
            .collect::<Vec<Rgb>>();

        // Expect 19 `Rgb` colors.
        assert_eq!(colors.len(), 19);

        let theme = Theme {
            color0: colors[0],
            color1: colors[1],
            color2: colors[2],
            color3: colors[3],
            color4: colors[4],
            color5: colors[5],
            color6: colors[6],
            color7: colors[7],
            color8: colors[8],
            color9: colors[9],
            color10: colors[10],
            color11: colors[11],
            color12: colors[12],
            color13: colors[13],
            color14: colors[14],
            color15: colors[15],
            background: colors[16],
            foreground: colors[17],
            cursor: colors[18],
        };

        themes.push(theme);
    }

    themes
}

/// Returns the path to the color settings file path.
pub fn get_color_settings_file_path() -> Result<PathBuf, &'static str> {
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
