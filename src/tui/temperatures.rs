use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    widgets::{Row, Table, Widget},
};

use crate::tui::widgets::get_bordered_block;

fn format_temperature(temperature: Option<f32>) -> String {
    match temperature {
        Some(temp) => format!("{temp:.2}°C"),
        None => "Unknown".to_string(),
    }
}

impl crate::app::App {
    pub fn temperatures_panel_width(&self) -> u16 {
        let component_width = self
            .temperatures
            .iter()
            .map(|t| t.sensor.cell_width())
            .max()
            .unwrap_or(0)
            .max("Component".cell_width());

        let temperature_width = self
            .temperatures
            .iter()
            .map(|t| format_temperature(t.temperature).cell_width())
            .max()
            .unwrap_or(0)
            .max("Temperature".cell_width());

        // Two border columns and one two-column gap between the table columns.
        component_width + temperature_width + 4
    }

    pub fn render_temperatures(&mut self, area: Rect, buf: &mut Buffer) {
        let innter_area = get_bordered_block(" Temperatures ", false, false, area, buf);

        let mut maximum_width = "Component".cell_width();

        self.temperatures.sort_by_key(|p| p.sensor.clone());

        let rows: Vec<Row> = self
            .temperatures
            .iter()
            .map(|t| {
                maximum_width = maximum_width.max(t.sensor.cell_width());

                let temperature = format_temperature(t.temperature);

                Row::new([t.sensor.clone(), temperature])
            })
            .collect();

        Table::new(
            rows,
            [Constraint::Length(maximum_width), Constraint::Fill(1)],
        )
        .column_spacing(2)
        .header(Row::new(["Component", "Temperature"]).fg(Color::Green))
        .render(innter_area, buf);
    }
}
