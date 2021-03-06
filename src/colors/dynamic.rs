use core::fmt;
use crate::{AnsiColors, DynColor};

#[allow(unused_imports)]
use crate::OwoColorize;

/// Available RGB colors for use with [`OwoColorize::color`](OwoColorize::color)
/// or [`OwoColorize::on_color`](OwoColorize::on_color)
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl DynColor for Rgb {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Rgb(r, g, b) = self;
        write!(f, "\x1b[38;2;{};{};{}m", r, g, b)
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Rgb(r, g, b) = self;
        write!(f, "\x1b[48;2;{};{};{}m", r, g, b)
    }
}

impl DynColor for str {
    fn fmt_ansi_fg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: AnsiColors = self.into();
        color.fmt_ansi_fg(f)
    }

    fn fmt_ansi_bg(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color: AnsiColors = self.into();
        color.fmt_ansi_bg(f)
    }
}

/// Implemented for drop-in replacement support for `colored`
impl<'a> From<&'a str> for AnsiColors {
    fn from(color: &'a str) -> Self {
        match color {
            "black" => AnsiColors::Black,
            "red" => AnsiColors::Red,
            "green" => AnsiColors::Green,
            "yellow" => AnsiColors::Yellow,
            "blue" => AnsiColors::Blue,
            "magenta" => AnsiColors::Magenta,
            "purple" => AnsiColors::Magenta,
            "cyan" => AnsiColors::Cyan,
            "white" => AnsiColors::White,
            "bright black" => AnsiColors::BrightBlack,
            "bright red" => AnsiColors::BrightRed,
            "bright green" => AnsiColors::BrightGreen,
            "bright yellow" => AnsiColors::BrightYellow,
            "bright blue" => AnsiColors::BrightBlue,
            "bright magenta" => AnsiColors::BrightMagenta,
            "bright cyan" => AnsiColors::BrightCyan,
            "bright white" => AnsiColors::BrightWhite,
            _ => AnsiColors::White,
        }
    }
}
