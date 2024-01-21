use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{
    self, BufWriter, IsTerminal, Result as IoResult, StdoutLock, Write,
};
use std::os::raw::c_ushort;
use std::process::ExitCode;
use std::string::ToString;

use crate::data::ALL_THEMES;
use crate::fail;
use crate::util::get_settings_file_path;

extern "C" {
    fn terminal_width() -> c_ushort;
}

pub const CLR: &str = "\x1b[0m";
pub const RED: &str = "\x1b[38;2;255;0;0m";
pub const GRN: &str = "\x1b[38;2;0;255;145m";
pub const BLUE: &str = "\x1b[38;2;0;170;235m";
pub const CYAN: &str = "\x1b[38;2;0;255;255m";

/// A wrapper around a data structure containing theme palettes.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Themes(pub BTreeSet<Theme>);

impl Themes {
    /// Initializes the themes data.
    pub fn init() -> Self {
        let mut set = BTreeSet::<Theme>::new();

        for data in &ALL_THEMES {
            let theme = Theme::from((data.0, &data.1[..]));

            if !set.insert(theme) {
                fail!("Unable to initialize built-in themes data");
            }
        }

        if set.len() != 247 {
            fail!("Unable to initialize built-in themes data");
        }

        Self(set)
    }

    // Returns the terminal width.
    fn get_terminal_width() -> usize {
        let width = unsafe { terminal_width() };

        usize::from(width)
    }

    /// Prints a list of all theme names to stdout.
    pub fn print(&self, do_light: bool, do_dark: bool) -> IoResult<ExitCode> {
        let stdout = io::stdout().lock();

        let is_term = stdout.is_terminal();

        let mut out = BufWriter::new(stdout);

        // Filter themes and collect names.
        let items = self
            .0
            .iter()
            .filter(|t| match (do_light, do_dark) {
                (true, true) => true,
                (false, true) => t.background.is_dark(),
                (true, false) => !t.background.is_dark(),
                (false, false) => unreachable!(),
            })
            .map(|t| t.name)
            .enumerate()
            .collect::<Vec<(usize, &str)>>();

        let max_idx = items.len() - 1;
        let max_width = Self::get_terminal_width() - 4;

        let mut line_len = 0;

        for (idx, name) in items {
            // Update line length; add 2 for the ", " separator.
            line_len = line_len + name.len() + 2;

            // Handle start of new group.
            if idx % 36 == 0 {
                write!(
                    &mut out,
                    "{}{}{name}{}, ",
                    if idx == 0 { "" } else { "\n\n" },
                    if is_term { CYAN } else { "" },
                    if is_term { CLR } else { "" }
                )?;

                line_len = name.len() + 2;
                continue;
            }

            // Handle theme name.
            write!(
                &mut out,
                "{}{name}{}",
                if line_len >= max_width { "\n" } else { "" },
                if idx == max_idx { "" } else { ", " }
            )?;

            if line_len >= max_width {
                line_len = name.len() + 2;
            }
        }

        writeln!(&mut out)?;

        out.flush()?;

        Ok(ExitCode::SUCCESS)
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
    /// Theme name.
    pub name: &'static str,
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
impl From<(&'static str, &[u8])> for Theme {
    fn from((name, bytes): (&'static str, &[u8])) -> Self {
        if bytes.len() != 57 {
            fail!("Unable to initialize the built-in themes");
        }

        Self {
            color0: Rgb::from(&bytes[9..=11]),
            color1: Rgb::from(&bytes[12..=14]),
            color2: Rgb::from(&bytes[15..=17]),
            color3: Rgb::from(&bytes[18..=20]),
            color4: Rgb::from(&bytes[21..=23]),
            color5: Rgb::from(&bytes[24..=26]),
            color6: Rgb::from(&bytes[27..=29]),
            color7: Rgb::from(&bytes[30..=32]),
            color8: Rgb::from(&bytes[33..=35]),
            color9: Rgb::from(&bytes[36..=38]),
            color10: Rgb::from(&bytes[39..=41]),
            color11: Rgb::from(&bytes[42..=44]),
            color12: Rgb::from(&bytes[45..=47]),
            color13: Rgb::from(&bytes[48..=50]),
            color14: Rgb::from(&bytes[51..=53]),
            color15: Rgb::from(&bytes[54..=56]),
            background: Rgb::from(&bytes[6..=8]),
            foreground: Rgb::from(&bytes[0..=2]),
            cursor: Rgb::from(&bytes[3..=5]),
            name,
        }
    }
}

impl Theme {
    /// Writes this `Theme` to the color settings file.
    pub fn apply(&self) -> Result<(), String> {
        get_settings_file_path()
            .map_err(ToString::to_string)
            .and_then(|path| fs::write(path, self.to_settings_string())
                .map_err(|e| e.to_string()))
    }

    /// Returns the `Theme` as a `String` in the settings file format.
    pub fn to_settings_string(self) -> String {
        format!(
            "\
#===============================================================
# Color Theme: {}
#
# Credit: https://github.com/Gogh-Co/Gogh/graphs/contributors
#===============================================================\n
color0={}\ncolor1={}\ncolor2={}\ncolor3={}\ncolor4={}\ncolor5={}\ncolor6={}
color7={}\ncolor8={}\ncolor9={}\ncolor10={}\ncolor11={}\ncolor12={}
color13={}\ncolor14={}\ncolor15={}\nbackground={}\nforeground={}\ncursor={}\n",
            self.name,
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

    /// Prints the color values to stdout using color formatting.
    pub fn print_values(&self) -> IoResult<()> {
        let stdout = io::stdout().lock();
        let is_term = stdout.is_terminal();

        let mut out = BufWriter::new(stdout);

        self.color0.print(&mut out, "color0", is_term)?;
        self.color1.print(&mut out, "color1", is_term)?;
        self.color2.print(&mut out, "color2", is_term)?;
        self.color3.print(&mut out, "color3", is_term)?;
        self.color4.print(&mut out, "color4", is_term)?;
        self.color5.print(&mut out, "color5", is_term)?;
        self.color6.print(&mut out, "color6", is_term)?;
        self.color7.print(&mut out, "color7", is_term)?;
        self.color8.print(&mut out, "color8", is_term)?;
        self.color9.print(&mut out, "color9", is_term)?;
        self.color10.print(&mut out, "color10", is_term)?;
        self.color11.print(&mut out, "color11", is_term)?;
        self.color12.print(&mut out, "color12", is_term)?;
        self.color13.print(&mut out, "color13", is_term)?;
        self.color14.print(&mut out, "color14", is_term)?;
        self.color15.print(&mut out, "color15", is_term)?;
        self.cursor.print(&mut out, "cursor", is_term)?;
        self.foreground.print(&mut out, "foreground", is_term)?;
        self.background.print(&mut out, "background", is_term)?;

        out.flush()?;

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
        if bytes.len() != 3 {
            fail!("Unable to initialize built-in themes data");
        }

        Self {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
        }
    }
}

impl Rgb {
    /// Calculates the perceived brightness from the RGB value and returns
    /// true if it is considered dark.
    //
    // See: https://www.nbdtech.com/Blog/archive/2008/04/27/Calculating-the-
    // Perceived-Brightness-of-a-Color.aspx
    pub fn is_dark(self) -> bool {
        let r = f32::from(self.r) * f32::from(self.r) * 0.241_f32;
        let g = f32::from(self.g) * f32::from(self.g) * 0.691_f32;
        let b = f32::from(self.b) * f32::from(self.b) * 0.068_f32;

        (r + g + b).sqrt().floor() < 130.0
    }

    /// Writes the formatted RBG value to stdout.
    pub fn print(
        self,
        out: &mut BufWriter<StdoutLock<'_>>,
        name: &str,
        is_terminal: bool
    ) -> IoResult<()> {
        assert!(name.len() <= 12);

        let dots = "............";
        let dots_slice = &dots[..(12 - name.len())];

        if is_terminal {
            write!(out, "{BLUE}{name}{CLR}{dots_slice}{CYAN}{self}{CLR} ")?;
            writeln!(out, "\x1b[48;2;{};{};{}m  {CLR}", self.r, self.g, self.b)?;
        } else {
            writeln!(out, "{name}{dots_slice}{self}")?;
        }

        Ok(())
    }
}
