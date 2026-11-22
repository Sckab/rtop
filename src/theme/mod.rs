use std::path::PathBuf;

use crossterm::style::Stylize;
use serde::Deserialize;

use crate::theme::color::{Color, named_color::NamedColor};

pub mod color;

#[derive(Debug, Deserialize)]
#[serde(default)]
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

impl Theme {
    fn print_color(name: &str, color: Color) {
        println!(
            "  {name:<30} {} {}",
            "█████".with(color.to_crossterm_color()),
            color
        );
    }

    pub fn print(&self) {
        println!("{}", "GENERAL".with(self.primary.to_crossterm_color()));
        Self::print_color("Primary", self.primary);
        Self::print_color("Secondary", self.secondary);
        Self::print_color("Error", self.error);

        println!("\n{}", "TABLES".with(self.primary.to_crossterm_color()));
        Self::print_color("Headers", self.table_headers);
        Self::print_color("Selected Foreground", self.selected_fg);
        Self::print_color("Selected Background", self.selected_bg);
        Self::print_color("Not Selected Foreground", self.not_selected_fg);
        Self::print_color("Not Selected Background", self.not_selected_bg);

        println!("\n{}", "MEMORY".with(self.primary.to_crossterm_color()));
        Self::print_color("Used", self.memory_used);
        Self::print_color("Available", self.memory_available);
        Self::print_color("Free", self.memory_free);

        println!("\n{}", "CPU".with(self.primary.to_crossterm_color()));
        Self::print_color("Graph", self.cpu_graph);

        println!("\n{}", "NETWORK".with(self.primary.to_crossterm_color()));
        Self::print_color("RX", self.rx);
        Self::print_color("TX", self.tx);
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Named(NamedColor::Green),
            secondary: Color::Named(NamedColor::Green),
            table_headers: Color::Named(NamedColor::Green),
            error: Color::Named(NamedColor::Red),
            not_selected_fg: Color::Named(NamedColor::Green),
            not_selected_bg: Color::Named(NamedColor::Black),
            selected_fg: Color::Named(NamedColor::Black),
            selected_bg: Color::Named(NamedColor::Green),
            rx: Color::Named(NamedColor::Magenta),
            tx: Color::Named(NamedColor::Yellow),
            memory_used: Color::Named(NamedColor::Red),
            memory_available: Color::Named(NamedColor::Yellow),
            memory_free: Color::Named(NamedColor::Green),
            cpu_graph: Color::Named(NamedColor::White),
        }
    }
}

pub fn get_default_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_default()
        .join("rtop")
        .join("theme.toml")
}
