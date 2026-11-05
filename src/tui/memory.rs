use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, BorderType, Row, Table, Widget},
};

impl super::App {
    pub fn render_memory_usage(&self, area: Rect, buf: &mut Buffer) {
        let rows = [
            Row::new([
                "Used".to_string(),
                (self.system_info.size_formatter)(self.stats.memory_used),
            ]),
            Row::new([
                "Available".to_string(),
                (self.system_info.size_formatter)(self.stats.memory_available),
            ]),
            Row::new([
                "Free".to_string(),
                (self.system_info.size_formatter)(self.stats.memory_free),
            ]),
            Row::new([
                "Total".to_string(),
                (self.system_info.size_formatter)(self.stats.memory_total),
            ]),
        ];

        let table = Table::new(
            rows,
            [
                Constraint::Length("Available".len() as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(3)
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" Memory Usage "),
        );

        table.render(area, buf);
    }
}
