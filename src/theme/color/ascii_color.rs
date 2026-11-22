use ratatui::style;

#[derive(Debug, Clone, Copy)]
pub enum AsciiColor {
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

impl AsciiColor {
    pub fn to_ratatui_color(&self) -> style::Color {
        match self {
            AsciiColor::Black => style::Color::Black,
            AsciiColor::Red => style::Color::Red,
            AsciiColor::Green => style::Color::Green,
            AsciiColor::Yellow => style::Color::Yellow,
            AsciiColor::Blue => style::Color::Blue,
            AsciiColor::Magenta => style::Color::Magenta,
            AsciiColor::Cyan => style::Color::Cyan,
            AsciiColor::Gray => style::Color::Gray,
            AsciiColor::DarkGray => style::Color::DarkGray,
            AsciiColor::LightRed => style::Color::LightRed,
            AsciiColor::LightGreen => style::Color::LightGreen,
            AsciiColor::LightYellow => style::Color::LightYellow,
            AsciiColor::LightBlue => style::Color::LightBlue,
            AsciiColor::LightMagenta => style::Color::LightMagenta,
            AsciiColor::LightCyan => style::Color::LightCyan,
            AsciiColor::White => style::Color::White,
        }
    }
}

impl TryFrom<String> for AsciiColor {
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
