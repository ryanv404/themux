// data.rs: I/O utilities, color theme data structures

use std::{
    collections::BTreeMap,
    env, fmt, fs, 
    num::ParseIntError, 
    path::Path,
    process::ExitCode, 
    str::FromStr,
};

use crate::{
    tui::Tui,
    utils::deserialize,
};

#[derive(Debug)]
pub struct Themes(pub BTreeMap<String, Theme>);

impl Themes {
    pub fn init() -> Self {
        let themes = deserialize();

        // Expect to deserialize exactly 247 themes.
        assert_eq!(themes.len(), 247);

        Self(themes)
    }

    pub fn list() -> ExitCode {
        let (width, _) = Tui::get_size();

        let themes = Self::init();
        let max_idx = themes.0.len() - 1;

        let mut line_len = 0;
        let mut output = String::new();

        for (idx, key) in themes.0.keys().enumerate() {
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
        ExitCode::SUCCESS
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

#[derive(Debug)]
pub struct Theme {
    pub color0: Rgb,
    pub color1: Rgb,
    pub color2: Rgb,
    pub color3: Rgb,
    pub color4: Rgb,
    pub color5: Rgb,
    pub color6: Rgb,
    pub color7: Rgb,
    pub color8: Rgb,
    pub color9: Rgb,
    pub color10: Rgb,
    pub color11: Rgb,
    pub color12: Rgb,
    pub color13: Rgb,
    pub color14: Rgb,
    pub color15: Rgb,
    pub background: Rgb,
    pub foreground: Rgb,
    pub cursor: Rgb,
}

impl Theme {
    pub fn apply(new_theme: &str) -> ExitCode {
        // Build up native path representation of the Termux directory's location.
        let termux_dir = match env::var("HOME") {
            Ok(home) => Path::new(&home).join(".termux"),
            Err(e) => return Self::fail( &format!("Unable to determine the location \
                                         of the HOME directory. {e}") )
        };

        // Ensure the path points to a directory creating it if it does not exist
        if !termux_dir.is_dir() {
            match fs::create_dir(&termux_dir) {
                Ok(_) => {},
                Err(e) => return Self::fail( &format!("Directory not found at \
                    `HOME/.termux` and cannot be created. {e}") )
            }
        }

        // Write the new theme to the Termux color settings file (.termux/colors.properties).
        match fs::write(termux_dir.join("colors.properties"), new_theme) {
            Ok(_) => Tui::reload_termux(),
            Err(e) => Self::fail( &format!("Unable to apply new theme. {e}") ),
        }
    }

    fn fail(msg: &str) -> ExitCode {
        eprintln!("[-] {msg}");
        ExitCode::FAILURE
    }

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

    fn get_entries(&self) -> Vec<(String, usize)> {
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
        let (width, _) = Tui::get_size();

        let entries = self.get_entries();

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
#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    // Red
    pub r: u8,
    // Green
    pub g: u8,
    // Blue
    pub b: u8,
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
