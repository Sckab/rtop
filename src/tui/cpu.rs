use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Cell, Chart, Dataset, GraphType, Paragraph, Row, Table, Widget},
};

#[derive(Debug, Clone)]
pub struct CpuStats {
    pub number: usize,
    pub usage: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pub global_usage: f32,
    pub cpus: Vec<CpuStats>,
    pub history: super::history::History<f64>,
}

impl super::App {
    fn render_cpu_graphic(&self, stats: &Cpu, area: Rect, buf: &mut Buffer) {
        let points: Vec<(f64, f64)> = stats
            .history
            .values
            .iter()
            .enumerate()
            .map(|(i, usage)| {
                let normalized = (*usage / 100.0) * area.height as f64;

                (i as f64, normalized)
            })
            .collect();

        let data = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .data(&points);

        let x_axis = Axis::default().bounds([0.0, area.width as f64]);
        let y_axis = Axis::default().bounds([0.0, area.height as f64]);

        Chart::new(vec![data])
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(area, buf);
    }

    pub fn render_cpu_usage(&self, area: Rect, buf: &mut Buffer) {
        let cpu_stats = &self.stats.cpu;

        let inner_area = super::general::get_borderd_block(" CPU Usage ", false, false, area, buf);

        let mut items: Vec<(String, String)> = Vec::new();

        let mut cpu_name_width = 0;
        let mut cpu_usage_width = 0;

        for cpu in &cpu_stats.cpus {
            let usage = format!("{:.0}%", cpu.usage);

            let name = format!("CPU {}", cpu.number);
            let usage = format!("{:>4}", usage);

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

        let panel_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length((rows + 4) as u16), Constraint::Fill(1)],
        )
        .split(inner_area);

        let table_layout =
            Layout::vertical([Constraint::Length((rows + 2) as u16), Constraint::Fill(1)])
                .split(panel_layout[0]);

        let table_total_layout =
            Layout::vertical([Constraint::Length((rows + 1) as u16), Constraint::Fill(1)])
                .split(table_layout[0]);

        let table_area = table_total_layout[0];
        let total_area = table_total_layout[1];

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

                table_rows.push(Row::new([
                    Cell::from(name.as_str()).style(Style::default().fg(Color::Green)),
                    Cell::from(usage.as_str()),
                ]));
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

        Paragraph::new(
            Span::raw("Total: ").fg(Color::Green)
                + Span::raw(format!("{:.0}%", self.stats.cpu.global_usage)),
        )
        .render(total_area, buf);

        self.render_cpu_graphic(cpu_stats, panel_layout[1], buf);
    }
}
