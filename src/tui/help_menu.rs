use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Clear, Row, StatefulWidget, Table},
};

pub struct Key {
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
    pub fn render_help_panel(&mut self, area: Rect, buf: &mut Buffer) {
        const COLUMN_SPACING: usize = 2;

        let max_width = |f: fn(&Key) -> usize| self.keys.iter().map(f).max().unwrap_or(0);

        let column_widths = [
            max_width(|key| key.name.len()),
            max_width(|key| key.primary_key.len()),
            max_width(|key| key.secondary_key.len()),
            max_width(|key| key.description.len()),
        ];

        let table_width = column_widths.iter().sum::<usize>() + (COLUMN_SPACING * 3) + 2;
        let height = self.keys.len().min((area.height / 2) as usize) + 2;

        let popup_area = area.centered(
            Constraint::Length(table_width as u16),
            Constraint::Length(height as u16),
        );

        let rows = self.keys.iter().map(|key| {
            Row::new([
                key.name.as_str(),
                key.primary_key.as_str(),
                key.secondary_key.as_str(),
                key.description.as_str(),
            ])
        });

        ratatui::widgets::Widget::render(Clear, popup_area, buf);

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
        .row_highlight_style(Style::new().bg(Color::White).fg(Color::Black))
        .render(popup_area, buf, &mut self.states.help_table);
    }
}
