use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    text::Span,
    widgets::{Row, Table, Widget},
};

use crate::tui::widgets;

impl crate::app::App {
    pub fn render_network(&self, area: Rect, buf: &mut Buffer) {
        let area = widgets::get_bordered_block(" Network ", false, false, area, buf);

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
                    Span::raw("RX").fg(Color::Magenta),
                    Span::raw(download_speed_formatted),
                    Span::raw((self.size_formatter)(self.io.network.total_download)),
                ]),
                Row::new([
                    Span::raw("TX").fg(Color::Yellow),
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
        .render(area, buf);
    }
}
