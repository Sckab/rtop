use ratatui::style;
use serde::Deserialize;

pub(super) mod ascii_color;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Ascii(ascii_color::AsciiColor),
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn to_ratatui_color(&self) -> style::Color {
        match self {
            Color::Ascii(color) => color.to_ratatui_color(),
            Color::Rgb(r, g, b) => style::Color::Rgb(*r, *g, *b),
        }
    }
}

impl From<Color> for ratatui::style::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Rgb(r, g, b) => Self::Rgb(r, g, b),
            Color::Ascii(color) => color.to_ratatui_color(),
        }
    }
}

impl From<&Color> for ratatui::style::Color {
    fn from(value: &Color) -> Self {
        match value {
            Color::Rgb(r, g, b) => Self::Rgb(*r, *g, *b),
            Color::Ascii(color) => color.to_ratatui_color(),
        }
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self::Rgb(value[0], value[1], value[2])
    }
}

impl TryFrom<String> for Color {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(hex) = value.strip_prefix('#') {
            if hex.len() != 6 || !hex.is_ascii() {
                anyhow::bail!("Invalid HEX color: #{hex}");
            }

            let r = u8::from_str_radix(&hex[0..2], 16)?;
            let g = u8::from_str_radix(&hex[2..4], 16)?;
            let b = u8::from_str_radix(&hex[4..6], 16)?;

            Ok(Self::Rgb(r, g, b))
        } else if let Some(color) = value.strip_prefix("rgb") {
            let value = color
                .strip_prefix('(')
                .and_then(|value| value.strip_suffix(')'))
                .ok_or_else(|| anyhow::anyhow!("Invalid RGB syntax"))?;

            let colors: [u8; 3] = value
                .split(',')
                .map(|color| color.trim().parse::<u8>())
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|_| anyhow::anyhow!("Expected 3 color components"))?;

            Ok(Self::Rgb(colors[0], colors[1], colors[2]))
        } else {
            Ok(Self::Ascii(
                ascii_color::AsciiColor::try_from(value).map_err(anyhow::Error::msg)?,
            ))
        }
    }
}
