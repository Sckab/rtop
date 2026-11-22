use crate::theme::color::{Color, ascii_color::AsciiColor};

pub mod color;

#[derive(Debug)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub table_headers: Color,
    pub error: Color,
    pub not_selected_fg: Color,
    pub not_selected_bg: Color,
    pub selected_fg: Color,
    pub selected_bg: Color,

    pub rx: Color,
    pub tx: Color,

    pub memory_used: Color,
    pub memory_available: Color,
    pub memory_free: Color,

    pub cpu_graph: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Ascii(AsciiColor::Green),
            secondary: Color::Ascii(AsciiColor::Green),
            table_headers: Color::Ascii(AsciiColor::Green),
            error: Color::Ascii(AsciiColor::Red),
            not_selected_fg: Color::Ascii(AsciiColor::Green),
            not_selected_bg: Color::Ascii(AsciiColor::Black),
            selected_fg: Color::Ascii(AsciiColor::Black),
            selected_bg: Color::Ascii(AsciiColor::Green),
            rx: Color::Ascii(AsciiColor::Magenta),
            tx: Color::Ascii(AsciiColor::Yellow),
            memory_used: Color::Ascii(AsciiColor::Red),
            memory_available: Color::Ascii(AsciiColor::Yellow),
            memory_free: Color::Ascii(AsciiColor::Green),
            cpu_graph: Color::Ascii(AsciiColor::White),
        }
    }
}
