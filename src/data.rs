// data.rs: I/O utilities, color theme data structures

use std::{collections::BTreeMap, fmt, num::ParseIntError, str::FromStr};

use bincode::Decode;
use inquire::{error::InquireError, Select};
use terminal_size::{terminal_size, Height, Width};

// Get the terminal's size or default to sensible values
fn get_terminal_size() -> (usize, usize) {
    let size = terminal_size();
    match size {
        Some((Width(w), Height(h))) => (w.into(), h.into()),
        _ => (60, 10),
    }
}

#[derive(Decode)]
pub struct Themes(BTreeMap<String, Theme>);

impl Themes {
    pub fn init() -> Self {
        let bytes = include_bytes!("themes.bin");

        let config = bincode::config::standard();
        let (themes, _): (Self, usize) = bincode::decode_from_slice(&bytes[..], config).unwrap();

        assert_eq!(themes.0.len(), 247);

        themes
    }

    pub fn list_themes(&self) {
        let (width, _) = get_terminal_size();

        let mut line_len = 0;
        let max_idx = self.0.len() - 1;

        let mut output = String::new();

        for (idx, key) in self.0.keys().enumerate() {
            let mut name = key.clone();

            if idx != max_idx {
                name.push_str(", ");
            }

            let name_len = name.len();

            if line_len + name_len >= width {
                output.push('\n');
                line_len = name_len;
            } else {
                line_len += name_len;
            }

            output.push_str(&name);
        }

        println!("{output}");
    }

    pub fn get_selection() -> Result<(String, Theme), &'static str> {
        let themes = Self::init();

        let options = themes.0.keys().collect::<Vec<&String>>();

        let (_, mut height) = get_terminal_size();

        // Adjust height to accommodate prompt and help message.
        height -= 3;

        let res: Result<&String, InquireError> = Select::new("Select a theme:", options)
            .with_help_message("↑↓ to move, type to filter, ENTER to select, ESC to exit")
            .with_page_size(height)
            .prompt();

        if let Ok(name) = res {
            themes.0
                .get(name)
                .map_or(Err("Unable to locate the selected theme."),
                        |theme| Ok((name.to_string(), theme.clone()))
                )
        } else {
            Err("Error while processing user input.")
        }
    }
}

impl fmt::Display for Themes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: Vec<String> = vec![];

        for (name, theme) in &self.0 {
            output.push(format!("[*] {name}\n{theme}"));
        }

        write!(f, "{}", output.join("\n\n"))
    }
}

#[derive(Decode, Clone)]
pub struct Theme {
    color0: Rgb,
    color1: Rgb,
    color2: Rgb,
    color3: Rgb,
    color4: Rgb,
    color5: Rgb,
    color6: Rgb,
    color7: Rgb,
    color8: Rgb,
    color9: Rgb,
    color10: Rgb,
    color11: Rgb,
    color12: Rgb,
    color13: Rgb,
    color14: Rgb,
    color15: Rgb,
    background: Rgb,
    foreground: Rgb,
    cursor: Rgb,
}

impl Theme {
    pub fn to_file_format(&self, name: &str) -> String {
        format!(
            "\
            #===============================================================\n\
            # Color Theme: {}\n\
            #===============================================================\n\n\
            color0={}\ncolor1={}\ncolor2={}\ncolor3={}\n\
            color4={}\ncolor5={}\ncolor6={}\ncolor7={}\n\
            color8={}\ncolor9={}\ncolor10={}\ncolor11={}\n\
            color12={}\ncolor13={}\ncolor14={}\ncolor15={}\n\
            background={}\nforeground={}\ncursor={}\n",
            name,
            self.color0,
            self.color1,
            self.color2,
            self.color3,
            self.color4,
            self.color5,
            self.color6,
            self.color7,
            self.color8,
            self.color9,
            self.color10,
            self.color11,
            self.color12,
            self.color13,
            self.color14,
            self.color15,
            self.background,
            self.foreground,
            self.cursor
        )
    }

    fn get_formatted_list(&self) -> Vec<(String, usize)> {
        let formatted_str = format!(
            "\
            cursor: {}, |foreground: {}, |background: {}, |color0: {}, |\
            color1: {}, |color2: {}, |color3: {}, |color4: {}, |color5: {}, |\
            color6: {}, |color7: {}, |color8: {}, |color9: {}, |color10: {}, |\
            color11: {}, |color12: {}, |color13: {}, |color14: {}, |color15: {}",
            self.cursor,
            self.foreground,
            self.background,
            self.color0,
            self.color1,
            self.color2,
            self.color3,
            self.color4,
            self.color5,
            self.color6,
            self.color7,
            self.color8,
            self.color9,
            self.color10,
            self.color11,
            self.color12,
            self.color13,
            self.color14,
            self.color15
        );

        formatted_str
            .as_str()
            .split('|')
            .map(|entry| (entry.to_string(), entry.len()))
            .collect::<Vec<(String, usize)>>()
    }
}

impl fmt::Display for Theme {
    // Make printing aware of terminal width for better formatting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, _) = get_terminal_size();
        let entries = self.get_formatted_list();

        let mut line_len: usize = 0;
        let mut output = String::new();

        for (entry, entry_len) in entries {
            if line_len + entry_len >= width {
                output.push('\n');
                line_len = entry_len;
            } else {
                line_len += entry_len;
            }

            output.push_str(&entry);
        }

        write!(f, "{output}")
    }
}

// Represents a color in RGB format.
#[derive(Decode, Clone)]
struct Rgb {
    // Red
    r: u8,
    // Green
    g: u8,
    // Blue
    b: u8,
}

impl FromStr for Rgb {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = u8::from_str_radix(&s[0..=1], 16)?;
        let g = u8::from_str_radix(&s[2..=3], 16)?;
        let b = u8::from_str_radix(&s[4..=5], 16)?;

        Ok(Self { r, g, b })
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
