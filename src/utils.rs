// utils.rs: Custom serialize/deserialize functions; stringify.
//
// Reduces the size of the stored Themes BTreeMap by 79% (from 76.2 kilobytes as
// JSON to a combined 16.6 kilobytes as binary) without added dependencies.

use std::{
    collections::BTreeMap,
    fs, io, mem,
    process::ExitCode,
    string,
};

use crate::data::{Rgb, Theme, Themes};

#[allow(dead_code)]
fn rgb_to_bytes(rgb: Rgb) -> [u8; 3] {
    [rgb.r, rgb.g, rgb.b]
}

fn bytes_to_rgb(arr: &[u8]) -> Rgb {
    Rgb { r: arr[0], g: arr[1], b: arr[2] }
}

#[allow(dead_code)]
fn serialize(themes: &Themes) -> ExitCode {
    if let Err(e) = serialize_names(themes) {
        eprintln!("[-] Error while writing theme names data: {e}");
        return ExitCode::FAILURE;
    }

    if let Err(e) = serialize_themes(themes) {
        eprintln!("[-] Error while writing theme colors data: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

pub fn deserialize() -> BTreeMap<String, Theme> {
    let names = deserialize_names();
    let themes = deserialize_themes();

    // Expect to deserialize exactly 247 theme names and 247 theme colors.
    assert_eq!(names.len(), 247);
    assert_eq!(themes.len(), 247);

    let mut themes_map: BTreeMap<String, Theme> = BTreeMap::new();

    for (name, theme) in names.into_iter().zip(themes.into_iter()) {
        themes_map.insert(name, theme);
    }

    themes_map
}

#[allow(dead_code)]
fn serialize_names(themes: &Themes) -> io::Result<()> {
    let names = themes.0
                      .keys()
                      .map(string::ToString::to_string)
                      .collect::<Vec<String>>();

    // Expect 247 theme names
    assert_eq!(names.len(), 247);

    let names_string = names.join(",");

    fs::write("src/theme_names.csv", names_string)?;
    Ok(())
}

fn deserialize_names() -> Vec<String> {
    let names_str = include_str!("theme_names.csv");

    names_str.split(',')
             .map(string::ToString::to_string)
             .collect::<Vec<String>>()
}

#[allow(dead_code)]
fn serialize_themes(themes: &Themes) -> io::Result<()> {
    // Initialize array of arrays of bytes (u8) to hold structured data.
    let mut themes_as_bytes: [[[u8; 3]; 19]; 247] = [[[0; 3]; 19]; 247];

    // Serialize the each Theme struct to an array of bytes.
    for (idx, (_, theme)) in themes.0.iter().enumerate() {
        let theme_bytes = [
            rgb_to_bytes(theme.color0),
            rgb_to_bytes(theme.color1),
            rgb_to_bytes(theme.color2),
            rgb_to_bytes(theme.color3),
            rgb_to_bytes(theme.color4),
            rgb_to_bytes(theme.color5),
            rgb_to_bytes(theme.color6),
            rgb_to_bytes(theme.color7),
            rgb_to_bytes(theme.color8),
            rgb_to_bytes(theme.color9),
            rgb_to_bytes(theme.color10),
            rgb_to_bytes(theme.color11),
            rgb_to_bytes(theme.color12),
            rgb_to_bytes(theme.color13),
            rgb_to_bytes(theme.color14),
            rgb_to_bytes(theme.color15),
            rgb_to_bytes(theme.background),
            rgb_to_bytes(theme.foreground),
            rgb_to_bytes(theme.cursor),
        ];

        themes_as_bytes[idx] = theme_bytes;
    }

    // Flatten the array of arrays into a 1-dimensional array of bytes.
    let serialized = unsafe {
        // 247 Theme structs * 19 struct fields per Theme struct * 3 bytes per struct
        // field equals 14079 total bytes.
        mem::transmute::<[[[u8; 3]; 19]; 247], [u8; 14079]>(themes_as_bytes)
    };

    assert_eq!(serialized.len(), 14079);

    fs::write("src/theme_colors.bin", serialized)?;
    Ok(())
}

fn deserialize_themes() -> Vec<Theme> {
    let mut themes: Vec<Theme> = vec![];

    let bytes = include_bytes!("theme_colors.bin");

    // Expect to have read exactly 14079 bytes from file.
    assert_eq!(bytes.len(), 14079);

    // 14079 total bytes / 247 themes = 57 bytes per Theme struct
    for theme_as_bytes in bytes.chunks(57) {
        // 57 bytes / 19 Theme struct fields = 3 bytes per field.
        let theme_fields = theme_as_bytes.chunks(3)
                                // Convert each 3-byte chunk to an Rgb struct.
                                .map(bytes_to_rgb)
                                .collect::<Vec<Rgb>>();

        // Expect 19 Rgb structs.
        assert_eq!(theme_fields.len(), 19);

        let theme = Theme {
            color0: theme_fields[0],
            color1: theme_fields[1],
            color2: theme_fields[2],
            color3: theme_fields[3],
            color4: theme_fields[4],
            color5: theme_fields[5],
            color6: theme_fields[6],
            color7: theme_fields[7],
            color8: theme_fields[8],
            color9: theme_fields[9],
            color10: theme_fields[10],
            color11: theme_fields[11],
            color12: theme_fields[12],
            color13: theme_fields[13],
            color14: theme_fields[14],
            color15: theme_fields[15],
            background: theme_fields[16],
            foreground: theme_fields[17],
            cursor: theme_fields[18],
        };

        themes.push(theme);
    }

    themes
}
