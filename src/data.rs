// data.rs: I/O utilities, theme data structures

use std::collections::BTreeMap;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use inquire::{error::InquireError, Select};
use serde::Deserialize;
use terminal_size::{Width, Height, terminal_size};

// Get the terminal's size or default to sensible values
fn get_terminal_size() -> (usize, usize) {
    let size = terminal_size();
    match size {
        Some((Width(w), Height(h))) => (w.into(), h.into()),
        _ => (80, 10)
    }
}

#[derive(Deserialize)]
pub struct Themes {
    pub data: BTreeMap<String, Theme>,
}

impl Themes {
    #[allow(dead_code)]
	fn new() -> Self {
		Self { data: BTreeMap::new() }
	}

	pub fn init() -> Self {
        let bytes = include_bytes!("themes.bin");
        let themes: Self = bincode::deserialize(bytes).unwrap();
        themes
	}

    pub fn list_themes(&self) {
        let (width, _) = get_terminal_size();
        let mut line_len = 0;

        // From testing, 2957 is the final output string length
        let mut output = String::with_capacity(3000);

        for name in self.data.keys() {
            let name = format!("{}, ", name);

            let name_len = name.len();

            if line_len + name_len <= width {
                line_len += name_len;
            } else {
                output.push('\n');
                line_len = name_len;
            }

            output.push_str(&name);
        }

        // Remove the final two characters (", ") using a slice
        println!("[> ALL THEMES <]\n\n{}\n", &output[..output.len() - 2]);
    }

    pub fn get_selection(&self) -> Result<(String, Theme), &str> {
        let themes = Themes::init();
        let options = themes.data.keys().collect::<Vec<&String>>();

        let (_, mut height) = get_terminal_size();
        height -= 3; // Adjust available height due to prompt and help message.

        let res: Result<&String, InquireError> = Select::new("Select a theme:", options)
            .with_help_message("↑↓ to move, type to filter, ENTER to select, ESC to exit")
            .with_page_size(height)
            .prompt();

        if let Ok(name) = res {
            match themes.data.get(name) {
                Some(theme) => Ok((name.to_string(), theme.clone())),
                None => Err("Unable to locate the selected theme.")
            }
        } else {
            Err("Error while processing user input.")
        }
    }
}

impl fmt::Display for Themes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: Vec<String> = vec![];

        for (name, theme) in &self.data {
            output.push(format!("[{}]\n{}", name, theme));
        }

        write!(f, "[> ALL THEMES <]\n\n{}\n", output.join("\n\n"))
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct Theme {
	foreground: Rgb,
    background: Rgb,
    cursor:     Rgb,
	color0:     Rgb,
	color1:     Rgb,
	color2:     Rgb,
	color3:     Rgb,
	color4:     Rgb,
	color5:     Rgb,
	color6:     Rgb,
	color7:     Rgb,
	color8:     Rgb,
	color9:     Rgb,
	color10:    Rgb,
	color11:    Rgb,
	color12:    Rgb,
	color13:    Rgb,
	color14:    Rgb,
	color15:    Rgb,
}

impl Theme {
    #[allow(dead_code)]
    fn new() -> Self {
        Default::default()
    }

    pub fn to_file_format(&self, name: &str) -> String {
        format!("\
            #===============================================================\n\
            # Color Theme: {}\n\
            #===============================================================\n\n\
            color0={}\ncolor1={}\ncolor2={}\ncolor3={}\n\
            color4={}\ncolor5={}\ncolor6={}\ncolor7={}\n\
            color8={}\ncolor9={}\ncolor10={}\ncolor11={}\n\
            color12={}\ncolor13={}\ncolor14={}\ncolor15={}\n\
            background={}\nforeground={}\ncursor={}\n",
            name, self.color0, self.color1, self.color2, self.color3, self.color4,
            self.color5, self.color6, self.color7, self.color8, self.color9,
            self.color10, self.color11, self.color12, self.color13, self.color14,
            self.color15, self.cursor, self.foreground, self.background 
            // TODO: Background and cursor colors are swapped in themes.bin
        )
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cursor: {},\n\
                   foreground: {},\n\
                   background: {},\n\
                   colors: [{}, {}, {}, {}, {}, {}, {}, {},\n         \
                            {}, {}, {}, {}, {}, {}, {}, {}]",
			self.background, self.foreground, self.cursor, self.color0,
			self.color1, self.color2, self.color3, self.color4, self.color5,
			self.color6, self.color7, self.color8, self.color9, self.color10,
			self.color11, self.color12, self.color13, self.color14, self.color15
            // TODO: Background and cursor colors are swapped in themes.bin
		)
	}
}

#[derive(Clone, Default, Deserialize)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    #[allow(dead_code)]
	fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
	}
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
