use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, BufWriter, Result as IoResult, StdoutLock, Write};
use std::num::ParseIntError;
use std::process::ExitCode;

use crate::data::ALL_THEMES;
use crate::tui::Tui;
use crate::util::{fail, get_settings_file_path};

pub const CLR: &str = "\x1b[0m";
pub const RED: &str = "\x1b[38;2;255;0;0m";
pub const GRN: &str = "\x1b[38;2;0;255;145m";
pub const BLUE: &str = "\x1b[38;2;0;170;235m";
pub const CYAN: &str = "\x1b[38;2;0;255;255m";

type Writer<'a> = BufWriter<StdoutLock<'a>>;

/// A wrapper around a data structure containing theme palettes.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Themes(pub BTreeSet<Theme>);

impl Themes {
    /// Initializes the themes data.
    pub fn init() -> Self {
        let mut set = BTreeSet::<Theme>::new();

        for data in &ALL_THEMES {
            let theme = Theme {
                name: data.0,
                cursor: data.1[3..=5].into(),
                foreground: data.1[0..=2].into(),
                background: data.1[6..=8].into(),
                color0: data.1[9..=11].into(),
                color1: data.1[12..=14].into(),
                color2: data.1[15..=17].into(),
                color3: data.1[18..=20].into(),
                color4: data.1[21..=23].into(),
                color5: data.1[24..=26].into(),
                color6: data.1[27..=29].into(),
                color7: data.1[30..=32].into(),
                color8: data.1[33..=35].into(),
                color9: data.1[36..=38].into(),
                color10: data.1[39..=41].into(),
                color11: data.1[42..=44].into(),
                color12: data.1[45..=47].into(),
                color13: data.1[48..=50].into(),
                color14: data.1[51..=53].into(),
                color15: data.1[54..=56].into()
            };

            assert!(set.insert(theme));
        }

        assert_eq!(set.len(), 247);

        Self(set)
    }

    /// Prints a list of all theme names to stdout.
    pub fn print_all(&self) -> ExitCode {
        let width = Tui::get_terminal_size().0;

        let mut line_len = 0;
        let max_width = width - 4;
        let max_idx = self.len() - 1;

        println!("{CYAN}Available Themes{CLR}\n");

        for (idx, name) in self.0.iter().map(|t| t.name).enumerate() {
            // Add 2 for the ", " separator.
            line_len = line_len + name.len() + 2;

            if name == "3024 Day" {
                print!("{GRN}{name}{CLR}, ");
                continue;
            }

            let is_final_item = idx == max_idx;

            let do_prepend_newline = line_len >= max_width;

            let is_new_group = matches!(
                name,
                "C64"
                | "Fairy Floss"
                | "Ibm 3270"
                | "N0Tch2K"
                | "Red Alert"
                | "Teerb"
            );

            match (is_new_group, is_final_item, do_prepend_newline) {
                (false, true, false) => println!("{name}"),
                (false, true, true) => println!("\n{name}"),
                (false, false, false) => print!("{name}, "),
                (false, false, true) => {
                    line_len = name.len() + 2;
                    print!("\n{name}, ");
                },
                (true, false, _) => {
                    line_len = name.len() + 2;
                    print!("\n\n{GRN}{name}{CLR}, ");
                },
                (true, true, _) => unreachable!(),
            }
        }

        ExitCode::SUCCESS
    }

    /// Returns the number of themes in this `Themes` instance.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the `Theme` that matches the given `name` (case insensitively),
    /// if `name` is a valid theme name.
    pub fn get(&self, query: &str) -> Option<&Theme> {
        self.0
            .iter()
            .find(|theme| theme.name.eq_ignore_ascii_case(query))
    }
}

/// A theme palette.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    /// Theme name.
    pub name: &'static str,
    /// Black.
    pub color0: Rgb,
    /// Dim red.
    pub color1: Rgb,
    /// Dim green.
    pub color2: Rgb,
    /// Dim yellow.
    pub color3: Rgb,
    /// Dim blue.
    pub color4: Rgb,
    /// Dim purple.
    pub color5: Rgb,
    /// Dim cyan.
    pub color6: Rgb,
    /// Dim white.
    pub color7: Rgb,
    /// Medium gray.
    pub color8: Rgb,
    /// Bright red.
    pub color9: Rgb,
    /// Bright green.
    pub color10: Rgb,
    /// Bright yellow.
    pub color11: Rgb,
    /// Light blue.
    pub color12: Rgb,
    /// Bright purple.
    pub color13: Rgb,
    /// Bright cyan.
    pub color14: Rgb,
    /// Bright white.
    pub color15: Rgb,
    /// Background color.
    pub background: Rgb,
    /// Foreground color.
    pub foreground: Rgb,
    /// Cursor color.
    pub cursor: Rgb,
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(other.name)
    }
}

impl Eq for Theme {}

impl PartialOrd for Theme {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(other.name))
    }
}

impl Ord for Theme {
    fn cmp(&self, other: &Self) -> Ordering {
        let lower1 = self.name.to_ascii_lowercase();
        let lower2 = other.name.to_ascii_lowercase();

        lower1.cmp(&lower2)
    }
}

impl Hash for Theme {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let lower = self.name.to_ascii_lowercase();
        lower.to_ascii_lowercase().hash(state);
    }
}

impl Theme {
    /// Writes this `Theme` to the color settings file.
    pub fn apply(&self) -> ExitCode {
        let path = match get_settings_file_path() {
            Ok(path) => path,
            Err(msg) => return fail(&format!("{RED}{msg}{CLR}")),
        };

        let settings = self.to_settings_string();

        if let Err(e) = fs::write(path, settings) {
            let msg = format!("{RED}Unable to apply theme.\n{e}{CLR}");
            return fail(&msg);
        }

        Tui::reload_settings();

        ExitCode::SUCCESS
    }

    /// Returns the `Theme` as a `String` in the settings file format.
    pub fn to_settings_string(self) -> String {
        format!("\
#===============================================================
# Color Theme: {}
#===============================================================\n
color0={}\ncolor1={}\ncolor2={}\ncolor3={}\ncolor4={}\ncolor5={}\ncolor6={}
color7={}\ncolor8={}\ncolor9={}\ncolor10={}\ncolor11={}\ncolor12={}
color13={}\ncolor14={}\ncolor15={}\nbackground={}\nforeground={}\ncursor={}\n",
            self.name, self.color0, self.color1,
            self.color2, self.color3, self.color4,
            self.color5, self.color6, self.color7,
            self.color8, self.color9, self.color10,
            self.color11, self.color12,
            self.color13, self.color14,
            self.color15, self.background,
            self.foreground, self.cursor
        )
    }

    /// Prints the color values to stdout using color formatting.
    pub fn print_values(&self) -> IoResult<()> {
        let stdout = io::stdout().lock();
        let mut w = BufWriter::new(stdout);

        writeln!(&mut w, "{GRN}{}{CLR}", self.name)?;
        self.color0.print(&mut w, "color0")?;
        self.color1.print(&mut w, "color1")?;
        self.color2.print(&mut w, "color2")?;
        self.color3.print(&mut w, "color3")?;
        self.color4.print(&mut w, "color4")?;
        self.color5.print(&mut w, "color5")?;
        self.color6.print(&mut w, "color6")?;
        self.color7.print(&mut w, "color7")?;
        self.color8.print(&mut w, "color8")?;
        self.color9.print(&mut w, "color9")?;
        self.color10.print(&mut w, "color10")?;
        self.color11.print(&mut w, "color11")?;
        self.color12.print(&mut w, "color12")?;
        self.color13.print(&mut w, "color13")?;
        self.color14.print(&mut w, "color14")?;
        self.color15.print(&mut w, "color15")?;
        self.cursor.print(&mut w, "cursor")?;
        self.foreground.print(&mut w, "foreground")?;
        self.background.print(&mut w, "background")?;
        w.flush()?;
        Ok(())
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
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl From<&[u8]> for Rgb {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 3);

        Self { r: bytes[0], g: bytes[1], b: bytes[2] }
    }
}

impl Rgb {
    /// Returns an `RGB` instance from a hex string (i.e. "RRGGBB").
    #[allow(dead_code)]
    pub fn from_hex_str(s: &str) -> Result<Self, ParseIntError> {
        let r = u8::from_str_radix(&s[0..=1], 16)?;
        let g = u8::from_str_radix(&s[2..=3], 16)?;
        let b = u8::from_str_radix(&s[4..=5], 16)?;

        Ok(Self { r, g, b })
    }

    /// Calculates the perceived darkness from the RGB value and returns true
    /// if it is considered dark.
    ///
    /// https://www.nbdtech.com/Blog/archive/2008/04/27/Calculating-the-
    /// Perceived-Brightness-of-a-Color.aspx
    #[allow(dead_code)]
    pub fn is_dark(&self) -> bool {
        let r = (self.r as u16 * self.r as u16) as f32 * 0.241_f32;
        let g = (self.g as u16 * self.g as u16) as f32 * 0.691_f32;
        let b = (self.b as u16 * self.b as u16) as f32 * 0.068_f32;

        let brightness = (r + g + b).sqrt().floor();

        brightness < 130.0
    }

    /// Writes the formatted RBG value to `Writer`.
    pub fn print(&self, w: &mut Writer<'_>, label: &str) -> IoResult<()> {
        write!(w, "{BLUE}{label}{CLR}: {CYAN}{self}{CLR}")?;
        write!(w, "{:1$}", " ", 11 - label.len())?;
        writeln!(
            w,
            "\x1b[48;2;{};{};{}m  {CLR}",
            self.r,
            self.g,
            self.b
        )?;
        Ok(())
    }
}
