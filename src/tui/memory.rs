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

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

        const USED_COLOR: Color = Color::Red;
        const AVAILABLE_COLOR: Color = Color::Yellow;
        const FREE_COLOR: Color = Color::Green;

        let used = Span::raw("Used: ").fg(USED_COLOR);
        let available = Span::raw("Available: ").fg(AVAILABLE_COLOR);
        let free = Span::raw("Free: ").fg(FREE_COLOR);

        Paragraph::new(
            used + (self.system_info.size_formatter)(self.stats.memory_used.value).into(),
        )
        .render(layout[0], buf);
        Paragraph::new(
            available + (self.system_info.size_formatter)(self.stats.memory_available.value).into(),
        )
        .render(layout[1], buf);
        Paragraph::new(
            free + (self.system_info.size_formatter)(self.stats.memory_free.value).into(),
        )
        .render(layout[2], buf);

        let width = layout[3].width;

        let x_axis = Axis::default().bounds([0.0, width as f64 - 0.5]);
        let y_axis = Axis::default().bounds([0.0, 100.0]);

        let mut used_points = Vec::new();
        let mut available_points = Vec::new();
        let mut free_points = Vec::new();

        Chart::new(vec![
            self.get_dataset(width, &mut used_points, USED_COLOR, &self.stats.memory_used),
            self.get_dataset(
                width,
                &mut available_points,
                AVAILABLE_COLOR,
                &self.stats.memory_available,
            ),
            self.get_dataset(width, &mut free_points, FREE_COLOR, &self.stats.memory_free),
        ])
        .x_axis(x_axis)
        .y_axis(y_axis)
        .render(layout[3], buf);

        let title = Span::raw("Total: ").fg(Color::Green);
        let size_formatted = Span::raw((self.system_info.size_formatter)(
            self.stats.memory_total.value,
        ));

        Paragraph::new(title + size_formatted).render(layout[4], buf);
    }
}
