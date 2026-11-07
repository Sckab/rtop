use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::{Marker, merge::MergeStrategy},
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Widget},
};

#[derive(Debug, Default, Clone)]
pub struct Memory {
    pub value: u64,
    pub history: super::history::History<u64>,
}

impl super::App {
    fn render_memory(&self, title: &str, memory: &Memory, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

        let title = Span::raw(format!("{title}: ")).fg(Color::Green);
        let size_formatted = Span::raw((self.system_info.size_formatter)(memory.value));

        Paragraph::new(title + size_formatted).render(layout[0], buf);

        let points: Vec<(f64, f64)> = memory
            .history
            .values
            .iter()
            .enumerate()
            .map(|(i, usage)| {
                let normalized = (*usage as f64 / self.stats.memory_total.value as f64) * 100.0;

                (i as f64, normalized)
            })
            .collect();

        let data = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .data(&points);

        let x_axis = Axis::default().bounds([0.0, 100.0]);
        let y_axis = Axis::default().bounds([0.0, 100.0]);

        Chart::new(vec![data])
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(layout[1], buf);

        Block::default()
            .borders(Borders::BOTTOM)
            .merge_borders(MergeStrategy::Fuzzy)
            .fg(Color::Green)
            .render(layout[2], buf);
    }

    pub fn render_memory_usage(&self, area: Rect, buf: &mut Buffer) {
        let area = super::general::get_borderd_block(" Memory Usage ", false, false, area, buf);

        let layout = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .split(area);

        self.render_memory("Used", &self.stats.memory_used, layout[0], buf);
        self.render_memory("Available", &self.stats.memory_available, layout[1], buf);
        self.render_memory("Free", &self.stats.memory_free, layout[2], buf);

        let title = Span::raw("Total: ").fg(Color::Green);
        let size_formatted = Span::raw((self.system_info.size_formatter)(
            self.stats.memory_total.value,
        ));

        Paragraph::new(title + size_formatted).render(layout[3], buf);
    }
}
