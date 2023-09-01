use std::{collections::BTreeMap, fs, mem, process::ExitCode, string};

use crate::data::{Rgb, Theme, Themes};

#[allow(unused)]
fn rgb_to_array(rgb: Rgb) -> [u8; 3] {
    [rgb.r, rgb.g, rgb.b]
}

fn array_to_rgb(arr: &[u8]) -> Rgb {
    Rgb { r: arr[0], g: arr[1], b: arr[2] }
}

#[allow(unused)]
fn encode(themes: &Themes) -> ExitCode {
    encode_names(themes);
    encode_themes();

    ExitCode::SUCCESS
}

#[allow(unused)]
fn encode_names(themes: &Themes) {
    let names = themes.0.keys()
                        .map(string::ToString::to_string)
                        .collect::<Vec<String>>();

    let enc_string = names.join("|");
    let encoded = enc_string.as_bytes();

    fs::write("src/theme_names.bin", encoded).unwrap();
}

#[allow(unused)]
fn encode_themes() {
    let themes = Themes::init();

    let mut enc_themes: [[[u8; 3]; 19]; 247] = [[[0; 3]; 19]; 247];

    for (idx, (_, th)) in themes.0.into_iter().enumerate() {
        let theme_arr = [
            rgb_to_array(th.color0),
            rgb_to_array(th.color1),
            rgb_to_array(th.color2),
            rgb_to_array(th.color3),
            rgb_to_array(th.color4),
            rgb_to_array(th.color5),
            rgb_to_array(th.color6),
            rgb_to_array(th.color7),
            rgb_to_array(th.color8),
            rgb_to_array(th.color9),
            rgb_to_array(th.color10),
            rgb_to_array(th.color11),
            rgb_to_array(th.color12),
            rgb_to_array(th.color13),
            rgb_to_array(th.color14),
            rgb_to_array(th.color15),
            rgb_to_array(th.background),
            rgb_to_array(th.foreground),
            rgb_to_array(th.cursor)
        ];

        enc_themes[idx] = theme_arr;
    }

    // Flatten the array of arrays into a 1-dimensional array of bytes.
    let encoded = unsafe {
        // 3 * 19 * 247 = 14079
        mem::transmute::<[[[u8; 3]; 19]; 247], [u8; 14079]>(enc_themes)
    };

    fs::write("src/theme_colors.bin", encoded).unwrap();
}

pub fn decode() -> BTreeMap<String, Theme> {
    let names = decode_names();
    let themes = decode_themes();

    let mut decoded: BTreeMap<String, Theme> = BTreeMap::new();

    for (name, theme) in names.into_iter().zip(themes.into_iter()) {
        decoded.insert(name, theme);
    }

    decoded
}

fn decode_names() -> Vec<String> {
    let bytes = include_bytes!("theme_names.bin");
    let enc_string = String::from_utf8(bytes.to_vec()).unwrap();

    let dec_names = enc_string.split('|')
                              .map(string::ToString::to_string)
                              .collect::<Vec<String>>();

    assert_eq!(dec_names.len(), 247);
    dec_names
}

fn decode_themes() -> Vec<Theme> {
    let bytes = include_bytes!("theme_colors.bin");

    let mut dec_themes: Vec<Theme> = vec![];

    // Expect to have read 14079 total bytes from file.
    assert_eq!(bytes.len(), 14079);

    // 14079 total bytes / 247 themes = 57 bytes per theme
    for theme_bytes in bytes.chunks(57) {
        // 57 bytes / 19 struct fields = 3 bytes per field
        let rgb_chunks = theme_bytes.chunks(3)
                                    .map(array_to_rgb)
                                    .collect::<Vec<Rgb>>();

        // Expect Vec<Rgb> of length 19 here.
        assert_eq!(rgb_chunks.len(), 19);

        let theme = Theme {
            color0: rgb_chunks[0],
            color1: rgb_chunks[1],
            color2: rgb_chunks[2],
            color3: rgb_chunks[3],
            color4: rgb_chunks[4],
            color5: rgb_chunks[5],
            color6: rgb_chunks[6],
            color7: rgb_chunks[7],
            color8: rgb_chunks[8],
            color9: rgb_chunks[9],
            color10: rgb_chunks[10],
            color11: rgb_chunks[11],
            color12: rgb_chunks[12],
            color13: rgb_chunks[13],
            color14: rgb_chunks[14],
            color15: rgb_chunks[15],
            background: rgb_chunks[16],
            foreground: rgb_chunks[17],
            cursor: rgb_chunks[18]
        };

        dec_themes.push(theme);
    }

    // Expect 247 entries here.
    assert_eq!(dec_themes.len(), 247);

    dec_themes
}
