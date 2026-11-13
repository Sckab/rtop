use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    widgets::{Row, StatefulWidget, Table},
};

use crate::{
    app::state::SelectedPanel,
    system::sorting::TemperatureSortType,
    tui::widgets::{get_bordered_block, get_selected_style},
};

fn format_temperature(temperature: Option<f32>) -> String {
    match temperature {
        Some(temp) => format!("{temp:.2}°C"),
        None => "Unknown".to_string(),
    }
}

impl crate::app::App {
    pub fn temperatures_panel_width(&self) -> u16 {
        let component_width = self
            .temperature_state
            .temperatures
            .iter()
            .map(|t| t.sensor.cell_width())
            .max()
            .unwrap_or(0)
            .max(
                format!(
                    "Sensor{}",
                    self.temperature_state
                        .sort
                        .get_arrow(TemperatureSortType::Sensor)
                )
                .cell_width(),
            );

        let temperature_width = self
            .temperature_state
            .temperatures
            .iter()
            .map(|t| format_temperature(t.temperature).cell_width())
            .max()
            .unwrap_or(0)
            .max(
                format!(
                    "Temperature{}",
                    self.temperature_state
                        .sort
                        .get_arrow(TemperatureSortType::Temperature)
                )
                .cell_width(),
            );

        // Two border columns and one two-column gap between the table columns.
        component_width + temperature_width + 4
    }

    pub fn render_temperatures(&mut self, area: Rect, buf: &mut Buffer) {
        let selected_index = self.tables.temperature_table.selected();

        let selected = match selected_index {
            Some(index) if !self.temperature_state.temperatures.is_empty() => {
                index.min(self.temperature_state.temperatures.len() - 1) + 1
            }
            _ => 0,
        };

        let innter_area = get_bordered_block(
            format!(
                " Temperatures ({selected}/{}) ",
                self.temperature_state.temperatures.len()
            ),
            false,
            false,
            area,
            buf,
        );

        let header = [
            format!(
                "Sensor{}",
                self.temperature_state
                    .sort
                    .get_arrow(TemperatureSortType::Sensor)
            ),
            format!(
                "Temperature{}",
                self.temperature_state
                    .sort
                    .get_arrow(TemperatureSortType::Temperature)
            ),
        ];

        let mut maximum_width = header[0].cell_width();

        let rows: Vec<Row> = self
            .temperature_state
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
        .header(Row::new(header).fg(Color::Green))
        .row_highlight_style(get_selected_style(matches!(
            &self.state.selected_panel,
            SelectedPanel::Termperature
        )))
        .render(innter_area, buf, &mut self.tables.temperature_table);
    }
}
