use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, BorderType, Row, Table, Widget},
};

impl super::App {
    pub fn render_cpu_usage(&self, area: Rect, buf: &mut Buffer) {
        let mut rows: Vec<Row> = vec![];

        for cpu in &self.stats.cpus {
            rows.push(Row::new([
                format!("CPU {}", cpu.number),
                format!("{:.0}%", cpu.usage),
            ]));
        }

        // 4 is the length of "CPU ", and this log is to get the number of digits of the cpu
        // numbers, so we get correct spacing
        let mut first_row_length = 4 + rows.len().max(1).ilog10() + 1;

        let total_string = "Total".to_string();

        rows.push(Row::new([
            total_string.to_string(),
            format!("{:.0}%", self.stats.global_cpu_usage),
        ]));

        if total_string.len() > first_row_length as usize {
            first_row_length = total_string.len() as u32;
        }

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" CPU Usage ");

        let inner = block.inner(area);

        block.render(area, buf);

        Table::new(
            rows,
            [
                Constraint::Length(first_row_length as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(3)
        .render(inner, buf);
    }
}
