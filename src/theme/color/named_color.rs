use std::fmt::Display;

use ratatui::style;

#[derive(Debug, Clone, Copy)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

impl NamedColor {
    pub fn to_ratatui_color(&self) -> style::Color {
        match self {
            NamedColor::Black => style::Color::Black,
            NamedColor::Red => style::Color::Red,
            NamedColor::Green => style::Color::Green,
            NamedColor::Yellow => style::Color::Yellow,
            NamedColor::Blue => style::Color::Blue,
            NamedColor::Magenta => style::Color::Magenta,
            NamedColor::Cyan => style::Color::Cyan,
            NamedColor::Gray => style::Color::Gray,
            NamedColor::DarkGray => style::Color::DarkGray,
            NamedColor::LightRed => style::Color::LightRed,
            NamedColor::LightGreen => style::Color::LightGreen,
            NamedColor::LightYellow => style::Color::LightYellow,
            NamedColor::LightBlue => style::Color::LightBlue,
            NamedColor::LightMagenta => style::Color::LightMagenta,
            NamedColor::LightCyan => style::Color::LightCyan,
            NamedColor::White => style::Color::White,
        }
    }

    pub fn to_crossterm_color(&self) -> crossterm::style::Color {
        match self {
            NamedColor::Black => crossterm::style::Color::Black,
            NamedColor::Red => crossterm::style::Color::DarkRed,
            NamedColor::Green => crossterm::style::Color::DarkGreen,
            NamedColor::Yellow => crossterm::style::Color::DarkYellow,
            NamedColor::Blue => crossterm::style::Color::DarkBlue,
            NamedColor::Magenta => crossterm::style::Color::DarkMagenta,
            NamedColor::Cyan => crossterm::style::Color::DarkCyan,
            NamedColor::Gray => crossterm::style::Color::Grey,
            NamedColor::DarkGray => crossterm::style::Color::DarkGrey,
            NamedColor::LightRed => crossterm::style::Color::Red,
            NamedColor::LightGreen => crossterm::style::Color::Green,
            NamedColor::LightYellow => crossterm::style::Color::Yellow,
            NamedColor::LightBlue => crossterm::style::Color::Blue,
            NamedColor::LightMagenta => crossterm::style::Color::Magenta,
            NamedColor::LightCyan => crossterm::style::Color::Cyan,
            NamedColor::White => crossterm::style::Color::White,
        }
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self {
            NamedColor::Black => (0, 0, 0),
            NamedColor::Red => (170, 0, 0),
            NamedColor::Green => (0, 170, 0),
            NamedColor::Yellow => (170, 85, 0),
            NamedColor::Blue => (0, 0, 170),
            NamedColor::Magenta => (170, 0, 170),
            NamedColor::Cyan => (0, 170, 170),
            NamedColor::Gray => (170, 170, 170),
            NamedColor::DarkGray => (85, 85, 85),
            NamedColor::LightRed => (255, 85, 85),
            NamedColor::LightGreen => (85, 255, 85),
            NamedColor::LightYellow => (255, 255, 85),
            NamedColor::LightBlue => (85, 85, 255),
            NamedColor::LightMagenta => (255, 85, 255),
            NamedColor::LightCyan => (85, 255, 255),
            NamedColor::White => (255, 255, 255),
        }
    }
}

impl Display for NamedColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NamedColor::Black => write!(f, "Black"),
            NamedColor::Red => write!(f, "Red"),
            NamedColor::Green => write!(f, "Green"),
            NamedColor::Yellow => write!(f, "Yellow"),
            NamedColor::Blue => write!(f, "Blue"),
            NamedColor::Magenta => write!(f, "Magenta"),
            NamedColor::Cyan => write!(f, "Cyan"),
            NamedColor::Gray => write!(f, "Gray"),
            NamedColor::DarkGray => write!(f, "Dark Gray"),
            NamedColor::LightRed => write!(f, "Light Red"),
            NamedColor::LightGreen => write!(f, "Light Green"),
            NamedColor::LightYellow => write!(f, "Light Yellow"),
            NamedColor::LightBlue => write!(f, "Light Blue"),
            NamedColor::LightMagenta => write!(f, "Light Magenta"),
            NamedColor::LightCyan => write!(f, "Light Cyan"),
            NamedColor::White => write!(f, "White"),
        }
    }
}

impl TryFrom<String> for NamedColor {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "magenta" => Ok(Self::Magenta),
            "cyan" => Ok(Self::Cyan),
            "gray" => Ok(Self::Gray),
            "dark gray" => Ok(Self::DarkGray),
            "light red" => Ok(Self::LightRed),
            "light green" => Ok(Self::LightGreen),
            "light yellow" => Ok(Self::LightYellow),
            "light blue" => Ok(Self::LightBlue),
            "light magenta" => Ok(Self::LightMagenta),
            "light cyan" => Ok(Self::LightCyan),
            "white" => Ok(Self::White),
            _ => Err(format!("{value} is not a valid color.")),
        }
    }
}
