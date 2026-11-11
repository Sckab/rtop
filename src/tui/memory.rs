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
    fn get_dataset<'a>(
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
        let area = super::widgets::get_bordered_block(" Memory Usage ", false, false, area, buf);

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

        const USED_COLOR: Color = Color::Red;
        const FREE_COLOR: Color = Color::Green;

        #[cfg(not(windows))]
        const AVAILABLE_COLOR: Color = Color::Yellow;

        let used = Span::raw("Used: ").fg(USED_COLOR);
        let free = Span::raw("Free: ").fg(FREE_COLOR);

        Paragraph::new(
            used + (self.system_info.size_formatter)(self.stats.memory_used.value).into(),
        )
        .render(layout[0], buf);
        Paragraph::new(
            free + (self.system_info.size_formatter)(self.stats.memory_free.value).into(),
        )
        .render(layout[FREE_ROW], buf);

        #[cfg(not(windows))]
        {
            let available = Span::raw("Available: ").fg(AVAILABLE_COLOR);

            Paragraph::new(
                available
                    + (self.system_info.size_formatter)(self.stats.memory_available.value).into(),
            )
            .render(layout[1], buf);
        }

        let width = layout[CHART_ROW].width;

        let x_axis = Axis::default().bounds([0.0, width as f64 - 0.5]);
        let y_axis = Axis::default().bounds([0.0, 100.0]);

        let mut used_points = Vec::new();
        let mut free_points = Vec::new();

        #[cfg(not(windows))]
        let mut available_points = Vec::new();

        let mut data =
            vec![self.get_dataset(width, &mut free_points, FREE_COLOR, &self.stats.memory_free)];

        #[cfg(not(windows))]
        data.push(self.get_dataset(
            width,
            &mut available_points,
            AVAILABLE_COLOR,
            &self.stats.memory_available,
        ));

        data.push(self.get_dataset(width, &mut used_points, USED_COLOR, &self.stats.memory_used));

        Chart::new(data)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(layout[CHART_ROW], buf);

        let title = Span::raw("Total: ").fg(Color::Green);
        let size_formatted = Span::raw((self.system_info.size_formatter)(
            self.stats.memory_total.value,
        ));

        Paragraph::new(title + size_formatted).render(layout[TOTAL_ROW], buf);
    }
}
