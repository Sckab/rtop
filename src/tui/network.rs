use ratatui::{
    buffer::Buffer,
    layout::{Constraint, HorizontalAlignment, Layout, Rect},
    style::{Color, Stylize},
    symbols::Marker,
    text::Span,
    widgets::{Axis, Chart, Dataset, GraphType, Row, Table, Widget},
};

use crate::{system::history::History, tui::widgets};

impl crate::app::App {
    fn get_dataset_network<'a>(
        &self,
        width: u16,
        points: &'a mut Vec<(f64, f64)>,
        color: Color,
        history: &History<u64>,
    ) -> Dataset<'a> {
        points.clear();

        let values = &history.values;
        let start = values.len().saturating_sub(width as usize);

        points.extend(
            values
                .range(start..)
                .enumerate()
                .map(|(i, usage)| (i as f64, *usage as f64)),
        );

        Dataset::default()
            .marker(Marker::Braille)
            .fg(color)
            .graph_type(GraphType::Line)
            .data(points)
    }

    pub fn render_network(&self, area: Rect, buf: &mut Buffer) {
        const RX_COLOR: Color = Color::Magenta;
        const TX_COLOR: Color = Color::Yellow;

        let area = widgets::get_bordered_block(" Network ", false, false, area, buf);

        let [table_area, chart_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]).areas(area);

        let download_speed = self.io.network.download_speed;
        let download_speed_formatted =
            format!("{}/s", (self.size_formatter)(download_speed as u64));
        let download_speed_formatted_size = download_speed_formatted.len() as u16;

        let upload_speed = self.io.network.upload_speed;
        let upload_speed_formatted = format!("{}/s", (self.size_formatter)(upload_speed as u64));
        let upload_speed_formatted_size = upload_speed_formatted.len() as u16;

        Table::new(
            [
                Row::new([
                    Span::raw("RX").fg(RX_COLOR),
                    Span::raw(download_speed_formatted),
                    Span::raw((self.size_formatter)(self.io.network.total_download)),
                ]),
                Row::new([
                    Span::raw("TX").fg(TX_COLOR),
                    Span::raw(upload_speed_formatted),
                    Span::raw((self.size_formatter)(self.io.network.total_upload)),
                ]),
            ],
            [
                Constraint::Length(2),
                Constraint::Length(
                    "Speed".len().max(
                        download_speed_formatted_size.max(upload_speed_formatted_size) as usize,
                    ) as u16,
                ),
                Constraint::Fill(1),
            ],
        )
        .header(Row::new(["", "Speed", "Total"]).fg(Color::Green))
        .column_spacing(2)
        .render(table_area, buf);

        let mut download_points = Vec::new();
        let mut upload_points = Vec::new();

        let max_value_download = self
            .io
            .network
            .history_rx
            .values
            .iter()
            .copied()
            .max()
            .unwrap_or(0);

        let max_value_upload = self
            .io
            .network
            .history_tx
            .values
            .iter()
            .copied()
            .max()
            .unwrap_or(0);

        let width = chart_area.width;

        let data = vec![
            self.get_dataset_network(
                width,
                &mut download_points,
                RX_COLOR,
                &self.io.network.history_rx,
            ),
            self.get_dataset_network(
                width,
                &mut upload_points,
                TX_COLOR,
                &self.io.network.history_tx,
            ),
        ];

        let y_max = max_value_download.max(max_value_upload) as f64;

        let max_value_formatted = (self.size_formatter)(y_max as u64);

        let mut parts = max_value_formatted.split_whitespace();

        let max_value = parts.next().unwrap_or("0");
        let max_value_unit = parts.next().unwrap_or("B");

        let x_axis = Axis::default().bounds([0.0, width as f64 - 1.5]);
        let y_axis = Axis::default()
            .bounds([0.0, y_max])
            .labels([format!("0 {}", max_value_unit), max_value.to_string()])
            .labels_alignment(HorizontalAlignment::Right)
            .fg(Color::Green);

        Chart::new(data)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(chart_area, buf);
    }
}
