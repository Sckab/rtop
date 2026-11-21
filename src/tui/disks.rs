use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Row, Table, Widget},
};

use crate::system::io::disks::Disk;

const DISKS_USAGE_HEADERS: [&str; 6] =
    ["Name", "Mount Point", "Used", "Available %", "Total", "R/W"];

impl crate::app::App {
    fn disks_usage_row(&self, disk: &Disk) -> [String; 6] {
        [
            disk.name.to_string_lossy().to_string(),
            disk.mount.to_string_lossy().to_string(),
            (self.size_formatter)(disk.total - disk.available),
            format!(
                "{:.1}%",
                (disk.available as f64 / disk.total as f64) * 100.0
            ),
            (self.size_formatter)(disk.total),
            format!(
                "{}/{}",
                (self.size_formatter)(disk.read),
                (self.size_formatter)(disk.write)
            ),
        ]
    }

    fn disks_usage_widths(&self) -> [u16; 6] {
        let mut widths = DISKS_USAGE_HEADERS.map(|header| header.cell_width());

        for disk in &self.io.disks.disks {
            for (width, value) in widths.iter_mut().zip(self.disks_usage_row(disk)) {
                *width = (*width).max(value.cell_width());
            }
        }

        widths
    }

    pub fn render_disks_usage(&self, area: Rect, buf: &mut Buffer) {
        let area = super::widgets::get_bordered_block(" Disks Usage ", false, false, area, buf);

        let rows: Vec<Row> = self
            .io
            .disks
            .disks
            .iter()
            .map(|disk| Row::new(self.disks_usage_row(disk)))
            .collect();

        Table::new(rows, self.disks_usage_widths())
            .column_spacing(2)
            .header(Row::new(DISKS_USAGE_HEADERS).fg(Color::Green))
            .render(area, buf);
    }
}
