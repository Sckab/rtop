use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Clear, Row, Table, Widget},
};

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(width),
            Constraint::Fill(1),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(height),
            Constraint::Fill(1),
        ])
        .split(horizontal[1])[1]
}

struct Key {
    pub name: String,
    pub primary_key: String,
    pub secondary_key: String,
    pub description: String,
}

impl Key {
    pub fn new(name: &str, primary_key: &str, secondary_key: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            primary_key: primary_key.to_string(),
            secondary_key: secondary_key.to_string(),
            description: description.to_string(),
        }
    }
}

impl super::App {
    pub fn render_help_panel(&self, area: Rect, buf: &mut Buffer) {
        const COLUMN_SPACING: usize = 2;

        let keys = [
            Key::new("Help", "h", "?", "Shows this help panel"),
            Key::new("Quit", "q", "Esc", "Quit the application (or this panel)"),
        ];

        let column_widths = [
            keys.iter().map(|key| key.name.len()).max().unwrap_or(0),
            keys.iter()
                .map(|key| key.primary_key.len())
                .max()
                .unwrap_or(0),
            keys.iter()
                .map(|key| key.secondary_key.len())
                .max()
                .unwrap_or(0),
            keys.iter()
                .map(|key| key.description.len())
                .max()
                .unwrap_or(0),
        ];

        let table_width = column_widths.iter().sum::<usize>() + (COLUMN_SPACING * 3) + 2;

        let popup_area = centered_rect(area, table_width as u16, (keys.len() + 2) as u16);

        let rows = keys.iter().map(|key| {
            Row::new([
                key.name.as_str(),
                key.primary_key.as_str(),
                key.secondary_key.as_str(),
                key.description.as_str(),
            ])
        });

        Clear.render(popup_area, buf);

        Table::new(
            rows,
            [
                Constraint::Length(column_widths[0] as u16),
                Constraint::Length(column_widths[1] as u16),
                Constraint::Length(column_widths[2] as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(COLUMN_SPACING as u16)
        .block(
            Block::bordered()
                .title(" Help ")
                .border_type(BorderType::Rounded),
        )
        .render(popup_area, buf);
    }
}
