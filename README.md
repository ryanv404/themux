# themux

A command-line tool for setting a color theme in a Termux terminal emulator.

Contains 247 built-in color themes.

Run `themux set` to launch an interactive list with fuzzy search capability
to select and automatically apply the theme.

View the available light themes with `themux light` and the available dark
themes with `themux dark`.

## Usage

```
USAGE: themux [OPTION] <COMMAND>

COMMANDS:
    all            Print a list of all available themes.
    current        Print the currently set theme.
    dark           Print a list of all dark themes.
    light          Print a list of all light themes.
    set            Set the theme from an interactive list.
    show <THEME>   Print the color value settings for THEME.

OPTIONS:
    -h, --help     Print this help message and exit.
    -v, --version  Print the version.
```
