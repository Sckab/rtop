use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Cell, Chart, Dataset, GraphType, Paragraph, Row, Table, Widget},
};

use crate::system::memory::Memory;

impl crate::app::App {
    fn get_dataset_memory<'a>(
        &self,
        width: u16,
        points: &'a mut Vec<(f64, f64)>,
        color: Color,
        memory: &Memory,
    ) -> Dataset<'a> {
        points.clear();

        let values = &memory.history.values;
        let start = values.len().saturating_sub(width as usize);

        points.extend(values.range(start..).enumerate().map(|(i, usage)| {
            let normalized = (*usage as f64 / self.stats.memory_total.value as f64) * 100.0;

            (i as f64, normalized)
        }));

        Dataset::default()
            .marker(Marker::Braille)
            .fg(color)
            .graph_type(GraphType::Line)
            .data(points)
    }

    pub fn render_memory_usage(&self, area: Rect, buf: &mut Buffer) {
        let area = super::widgets::get_bordered_block(
            &self.theme,
            " Memory Usage ",
            false,
            false,
            area,
            buf,
        );

        // On Windows, available memory is always equal to free memory, so the
        // Available row is omitted and its space is given to the graph.

        let [table_area, chart_area, total_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let mut rows = vec![Row::new([
            Cell::new("Used").fg(self.theme.memory_used),
            Cell::new((self.size_formatter)(self.stats.memory_used.value)),
        ])];

        #[cfg(not(windows))]
        {
            rows.push(Row::new([
                Cell::new("Available").fg(self.theme.memory_available),
                Cell::new((self.size_formatter)(self.stats.memory_available.value)),
            ]));
        }

        rows.push(Row::new([
            Cell::new("Free").fg(self.theme.memory_free),
            Cell::new((self.size_formatter)(self.stats.memory_free.value)),
        ]));

        Table::new(
            rows,
            [
                Constraint::Length("Available".len() as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(2)
        .render(table_area, buf);

        let width = chart_area.width;

        let x_axis = Axis::default().bounds([0.0, width as f64 - 1.5]);
        let y_axis = Axis::default().bounds([0.0, 100.0]);

        let mut used_points = Vec::new();
        let mut free_points = Vec::new();

        #[cfg(not(windows))]
        let mut available_points = Vec::new();

        let mut data = vec![self.get_dataset_memory(
            width,
            &mut free_points,
            self.theme.memory_free.into(),
            &self.stats.memory_free,
        )];

        #[cfg(not(windows))]
        data.push(self.get_dataset_memory(
            width,
            &mut available_points,
            self.theme.memory_available.into(),
            &self.stats.memory_available,
        ));

        data.push(self.get_dataset_memory(
            width,
            &mut used_points,
            self.theme.memory_used.into(),
            &self.stats.memory_used,
        ));

        Chart::new(data)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(chart_area, buf);

        let title = Span::raw("Total ").fg(self.theme.secondary);
        let size_formatted = Span::raw((self.size_formatter)(self.stats.memory_total.value));

        Paragraph::new(title + size_formatted).render(total_area, buf);
    }
}
