use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    widgets::{Block, BorderType, Row, Table, Widget},
};

impl super::App {
    pub fn render_cpu_usage(&self, area: Rect, buf: &mut Buffer) {
        let mut rows: Vec<Row> = vec![];

        for (mut index, cpu) in self.system.cpus().iter().enumerate() {
            index += 1;

            rows.push(Row::new([
                format!("CPU {index}"),
                format!("{:.0}%", cpu.usage()),
            ]));
        }

        // 4 is the length of "CPU ", and this log is to get the number of digits of the cpu
        // numbers, so we get correct spacing
        let mut first_row_length = 4 + rows.len().ilog10() + 1;

        rows.push(Row::new([
            "Total: ".to_string(),
            format!("{:.0}%", self.cpu_usage),
        ]));

        if "Total: ".len() > first_row_length as usize {
            first_row_length = "Total: ".len() as u32;
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
