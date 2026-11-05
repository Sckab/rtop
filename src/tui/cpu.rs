use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Paragraph, Row, Table, Widget},
};

impl super::App {
    pub fn render_cpu_usage(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" CPU Usage ");

        let inner_area = block.inner(area);
        block.render(area, buf);

        let mut items: Vec<(String, String)> = Vec::new();

        let mut cpu_name_width = 0;
        let mut cpu_usage_width = 0;

        for cpu in &self.stats.cpus {
            let name = format!("CPU {}", cpu.number);
            let usage = format!("{:.0}%", cpu.usage);

            cpu_name_width = cpu_name_width.max(name.len());
            cpu_usage_width = cpu_usage_width.max(usage.len());

            items.push((name, usage));
        }

        if items.is_empty() || inner_area.width == 0 || inner_area.height == 0 {
            return;
        }

        let spacing = 1;
        let column_spacing = spacing + 2;

        let item_width = cpu_name_width + cpu_usage_width + spacing;

        let columns =
            ((inner_area.width as usize + column_spacing) / (item_width + column_spacing)).max(1);
        let rows = items.len().div_ceil(columns);

        let layout =
            Layout::vertical([Constraint::Length((rows + 1) as u16), Constraint::Length(1)])
                .split(inner_area);

        let table_area = layout[0];
        let total_area = layout[1];

        let table_layout = Layout::horizontal((0..columns).map(|_| Constraint::Fill(1)))
            .spacing(spacing as u16)
            .split(table_area);

        for i in 0..columns {
            let mut table_rows = Vec::new();

            for j in 0..rows.min(table_area.height as usize) {
                let index = i + (j * columns);

                if index >= items.len() {
                    break;
                }

                let (name, usage) = &items[index];

                table_rows.push(Row::new([name.as_str(), usage.as_str()]));
            }

            Table::new(
                table_rows,
                [
                    Constraint::Length(cpu_name_width as u16),
                    Constraint::Fill(1),
                ],
            )
            .column_spacing(spacing as u16)
            .render(table_layout[i], buf);
        }

        Paragraph::new(format!("Total: {:.0}%", self.stats.global_cpu_usage))
            .render(total_area, buf);
    }
}
