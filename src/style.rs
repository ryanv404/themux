use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::num::ParseIntError;
use std::process::ExitCode;

use crate::tui::Tui;
use crate::util::{
    deserialize, fail, get_color_settings_file_path,
};

pub const CLR: &str = "\x1b[0m";
pub const RED: &str = "\x1b[38;2;255;0;0m";
pub const GRN: &str = "\x1b[38;2;0;255;145m";
pub const BLUE: &str = "\x1b[38;2;0;170;235m";
pub const CYAN: &str = "\x1b[38;2;0;255;255m";

/// A wrapper around a data structure containing theme palettes.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Themes(pub BTreeMap<String, Theme>);

impl Display for Themes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let (width, _) = Tui::get_terminal_size();

        let mut line_len = 0;
        let max_width = width - 4;
        let max_idx = self.len() - 1;

        for (idx, name) in self.0.keys().enumerate() {
            let is_final_item = idx == max_idx;

            // Add 2 for the ", " separator.
            line_len = line_len + name.len() + 2;
            let do_prepend_newline = line_len >= max_width;

            if name.as_str() == "3024 day" {
                write!(f, "{GRN}{name}{CLR}, ")?;
                continue;
            }

            let is_new_group = matches!(
                name.as_str(),
                "c64" | "fairy floss" | "ibm 3270" | "n0tch2k"
                    | "red alert" | "teerb"
            );

            match (is_new_group, is_final_item, do_prepend_newline) {
                (false, true, false) => write!(f, "{name}")?,
                (false, true, true) => write!(f, "\n{name}")?,
                (false, false, false) => write!(f, "{name}, ")?,
                (false, false, true) => {
                    line_len = name.len() + 2;
                    write!(f, "\n{name}, ")?;
                },
                (true, false, _) => {
                    line_len = name.len() + 2;
                    write!(f, "\n\n{GRN}{name}{CLR}, ")?;
                },
                (true, true, _) => unreachable!(),
            }
        }

        Ok(())
    }
}

impl Themes {
    /// Deserializes themes data from a local file and returns it as a
    /// `Themes` instance.
    pub fn init() -> Self {
        let themes = deserialize();

        // Expect exactly 247 themes.
        assert_eq!(themes.len(), 247);

        themes
    }

    /// Prints a list of all theme names to stdout.
    pub fn list(&self) -> ExitCode {
        println!("{CYAN}Available Themes:{CLR}\n\n{self}");
        ExitCode::SUCCESS
    }

    /// Returns the number of themes in this `Themes` instance.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the `Theme` that matches the given `name` (case insensitively),
    /// if `name` is a valid theme name.
    pub fn get(&self, query: &str) -> Option<&Theme> {
        for (name, theme) in &self.0 {
            if name.eq_ignore_ascii_case(query) {
                return Some(theme);
            }
        }

        None
    }
}

/// A theme palette.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Theme {
    /// Black.
    pub color0: Rgb,
    /// Red.
    pub color1: Rgb,
    /// Green.
    pub color2: Rgb,
    /// Yellow.
    pub color3: Rgb,
    /// Blue.
    pub color4: Rgb,
    /// Purple.
    pub color5: Rgb,
    /// Cyan.
    pub color6: Rgb,
    /// White.
    pub color7: Rgb,
    /// Bright black.
    pub color8: Rgb,
    /// Bright red.
    pub color9: Rgb,
    /// Bright green.
    pub color10: Rgb,
    /// Bright yellow.
    pub color11: Rgb,
    /// Bright blue.
    pub color12: Rgb,
    /// Bright purple.
    pub color13: Rgb,
    /// Bright cyan.
    pub color14: Rgb,
    /// Bright white.
    pub color15: Rgb,
    /// Terminal background color.
    pub background: Rgb,
    /// Terminal foreground color.
    pub foreground: Rgb,
    /// Cursor color.
    pub cursor: Rgb,
//    /// Theme name.
//    pub name: String,
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "\
{BLUE}cursor{CLR}: {}\n{BLUE}color0{CLR}: {}\n{BLUE}color1{CLR}: {}
{BLUE}color2{CLR}: {}\n{BLUE}color3{CLR}: {}\n{BLUE}color4{CLR}: {}
{BLUE}color5{CLR}: {}\n{BLUE}color6{CLR}: {}\n{BLUE}color7{CLR}: {}
{BLUE}color8{CLR}: {}\n{BLUE}color9{CLR}: {}\n{BLUE}color10{CLR}: {}
{BLUE}color11{CLR}: {}\n{BLUE}color12{CLR}: {}\n{BLUE}color13{CLR}: {}
{BLUE}color14{CLR}: {}\n{BLUE}color15{CLR}: {}\n{BLUE}foreground{CLR}: {}
{BLUE}background{CLR}: {}",
            self.cursor, self.color0, self.color1, self.color2, self.color3,
            self.color4, self.color5, self.color6, self.color7, self.color8,
            self.color9, self.color10, self.color11, self.color12,
            self.color13, self.color14, self.color15, self.foreground,
            self.background
        )?;

        Ok(())
    }
}

impl Theme {
    /// Writes this `Theme` to the color settings file.
    pub fn apply(&self, name: &str) -> ExitCode {
        let path = match get_color_settings_file_path() {
            Ok(path) => path,
            Err(msg) => return fail(&format!("{RED}{msg}{CLR}")),
        };

        let settings = self.to_settings_string(name);

        if let Err(e) = fs::write(path, settings) {
            let msg = format!("{RED}Unable to apply theme.\n{e}{CLR}");
            return fail(&msg);
        }

        Tui::reload_settings();

        ExitCode::SUCCESS
    }

    /// Returns the `Theme` as a `String` in the settings file format.
    pub fn to_settings_string(&self, name: &str) -> String {
        format!("\
#===============================================================
# Color Theme: {name}
#===============================================================\n
color0={}\ncolor1={}\ncolor2={}\ncolor3={}\ncolor4={}\ncolor5={}\ncolor6={}
color7={}\ncolor8={}\ncolor9={}\ncolor10={}\ncolor11={}\ncolor12={}
color13={}\ncolor14={}\ncolor15={}\nbackground={}\nforeground={}\ncursor={}\n",
            self.color0.to_hex_str(), self.color1.to_hex_str(),
            self.color2.to_hex_str(), self.color3.to_hex_str(),
            self.color4.to_hex_str(), self.color5.to_hex_str(),
            self.color6.to_hex_str(), self.color7.to_hex_str(),
            self.color8.to_hex_str(), self.color9.to_hex_str(),
            self.color10.to_hex_str(), self.color11.to_hex_str(),
            self.color12.to_hex_str(), self.color13.to_hex_str(),
            self.color14.to_hex_str(), self.color15.to_hex_str(),
            self.background.to_hex_str(), self.foreground.to_hex_str(),
            self.cursor.to_hex_str()
        )
    }
}

/// A color in RGB format.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb {
    /// Red.
    pub r: u8,
    /// Green.
    pub g: u8,
    /// Blue.
    pub b: u8,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{CYAN}#{:02X}{:02X}{:02X}{CLR}", self.r, self.g, self.b)
    }
}

impl Rgb {
    /// Returns an `RGB` instance from a hex string (i.e. "RRGGBB").
    pub fn from_hex_str(s: &str) -> Result<Self, ParseIntError> {
        let r = u8::from_str_radix(&s[0..=1], 16)?;
        let g = u8::from_str_radix(&s[2..=3], 16)?;
        let b = u8::from_str_radix(&s[4..=5], 16)?;
        Ok(Self { r, g, b })
    }

    /// Returns this `RGB` color as a hex string (i.e. "RRGGBB").
    pub fn to_hex_str(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    /// Returns an `RGB` instance from a three element bytes slice.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 3);

        Self { r: bytes[0], g: bytes[1], b: bytes[2] }
    }

    /// Returns this `RGB` instance as a bytes array.
    pub const fn to_bytes(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
