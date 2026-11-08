use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Cell, Clear, Row, StatefulWidget, Table},
};

use crate::app::keymap::Key;

impl crate::app::App {
    pub fn render_help_panel(&mut self, area: Rect, buf: &mut Buffer) {
        const COLUMN_SPACING: usize = 2;

        let max_width = |f: fn(&Key) -> usize| self.keys.iter().map(f).max().unwrap_or(0);

        let column_widths = [
            max_width(|key| key.primary_key.len()),
            max_width(|key| key.secondary_key.len()),
            max_width(|key| key.description.len()),
        ];

        let table_width = column_widths.iter().sum::<usize>() + (COLUMN_SPACING * 3) + 2;
        let height = self.keys.len().min((area.height / 2) as usize) + 2;

        let area = area.centered(
            Constraint::Length(table_width as u16),
            Constraint::Length(height as u16),
        );

        ratatui::widgets::Widget::render(Clear, area, buf);

        let popup_area = super::widgets::get_bordered_block(" Help ", false, true, area, buf);

        let rows: Vec<Row> = self
            .keys
            .iter()
            .map(|key| {
                let primary = Span::raw(key.primary_key.clone());
                let secondary = Span::raw(key.secondary_key.clone());
                let description = Span::raw(key.description.clone()).into_right_aligned_line();

                Row::new([
                    Cell::from(primary),
                    Cell::from(secondary),
                    Cell::from(description),
                ])
            })
            .collect();

        Table::new(
            rows,
            [
                Constraint::Length(column_widths[0] as u16),
                Constraint::Length(column_widths[1] as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(COLUMN_SPACING as u16)
        .row_highlight_style(Style::new().bg(Color::Green).fg(Color::Black))
        .render(popup_area, buf, &mut self.states.help_table);
    }
}
