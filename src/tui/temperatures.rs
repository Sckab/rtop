use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    widgets::{Row, Table, Widget},
};

use crate::tui::widgets::get_bordered_block;

impl crate::app::App {
    pub fn render_temperatures(&mut self, area: Rect, buf: &mut Buffer) {
        let innter_area = get_bordered_block(" Temperatures ", false, false, area, buf);

        let mut maximum_width = "Component".len();

        self.temperatures.sort_by_key(|p| p.sensor.clone());

        let rows: Vec<Row> = self
            .temperatures
            .iter()
            .map(|t| {
                maximum_width = maximum_width.max(t.sensor.len());

                let temperature = match t.temperature {
                    Some(temp) => format!("{temp:.2}C°"),
                    None => "Unknown".to_string(),
                };

                Row::new([t.sensor.clone(), temperature])
            })
            .collect();

        Table::new(
            rows,
            [
                Constraint::Length(maximum_width as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(2)
        .header(Row::new(["Component", "Temperature"]).fg(Color::Green))
        .render(innter_area, buf);
    }
}
