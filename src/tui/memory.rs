use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Chart, Dataset, GraphType, Paragraph, Widget},
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
        #[cfg(windows)]
        const FREE_ROW: usize = 1;
        #[cfg(not(windows))]
        const FREE_ROW: usize = 2;

        const CHART_ROW: usize = FREE_ROW + 1;
        const TOTAL_ROW: usize = CHART_ROW + 1;

        let mut constraints = vec![Constraint::Length(1)]; // Used

        #[cfg(not(windows))]
        {
            constraints.push(Constraint::Length(1)); // Available
        }

        constraints.push(Constraint::Length(1)); // Free
        constraints.push(Constraint::Fill(1)); // Graph
        constraints.push(Constraint::Length(1)); // Total

        let layout = Layout::vertical(constraints).split(area);

        let used = Span::raw("Used: ").fg(self.theme.memory_used);
        let free = Span::raw("Free: ").fg(self.theme.memory_free);

        Paragraph::new(used + (self.size_formatter)(self.stats.memory_used.value).into())
            .render(layout[0], buf);
        Paragraph::new(free + (self.size_formatter)(self.stats.memory_free.value).into())
            .render(layout[FREE_ROW], buf);

        #[cfg(not(windows))]
        {
            let available = Span::raw("Available: ").fg(self.theme.memory_available);

            Paragraph::new(
                available + (self.size_formatter)(self.stats.memory_available.value).into(),
            )
            .render(layout[1], buf);
        }

        let width = layout[CHART_ROW].width;

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
            .render(layout[CHART_ROW], buf);

        let title = Span::raw("Total: ").fg(self.theme.secondary);
        let size_formatted = Span::raw((self.size_formatter)(self.stats.memory_total.value));

        Paragraph::new(title + size_formatted).render(layout[TOTAL_ROW], buf);
    }
}
