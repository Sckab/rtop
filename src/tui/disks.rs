use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::Rect,
    style::Stylize,
    widgets::{Row, StatefulWidget, Table},
};

use crate::{
    app::state::SelectedPanel,
    system::{io::disks::Disk, sorting::DisksSortType},
    tui::widgets::get_selected_style,
};

impl crate::app::App {
    fn get_disks_usage_header(&self) -> Vec<String> {
        vec![
            format!("Name{}", self.sortings.disks.get_arrow(DisksSortType::Name)),
            format!(
                "Mount Point{}",
                self.sortings.disks.get_arrow(DisksSortType::MountPoint)
            ),
            format!("Used{}", self.sortings.disks.get_arrow(DisksSortType::Used)),
            format!(
                "Available{}",
                self.sortings.disks.get_arrow(DisksSortType::Available)
            ),
            "Available %".to_string(),
            format!(
                "Total{}",
                self.sortings.disks.get_arrow(DisksSortType::Total)
            ),
            format!(
                "R{}/W{}",
                self.sortings.disks.get_arrow(DisksSortType::Read),
                self.sortings.disks.get_arrow(DisksSortType::Write)
            ),
        ]
    }

    fn disks_usage_row(&self, disk: &Disk) -> [String; 7] {
        [
            disk.name.to_string_lossy().to_string(),
            disk.mount.to_string_lossy().to_string(),
            (self.size_formatter)(disk.total - disk.available),
            (self.size_formatter)(disk.available),
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

    fn disks_usage_widths(&self) -> Vec<u16> {
        let mut widths: Vec<u16> = self
            .get_disks_usage_header()
            .iter()
            .map(|header| header.cell_width())
            .collect();

        for disk in &self.io.disks.disks {
            for (width, value) in widths.iter_mut().zip(self.disks_usage_row(disk)) {
                *width = (*width).max(value.cell_width());
            }
        }

        widths
    }

    pub fn render_disks_usage(&mut self, area: Rect, buf: &mut Buffer) {
        let selected_index = self.tables.disks_table.selected();

        let selected = match selected_index {
            Some(index) if !self.io.disks.disks.is_empty() => {
                index.min(self.io.disks.disks.len() - 1) + 1
            }
            _ => 0,
        };

        let area = super::widgets::get_bordered_block(
            &self.theme,
            format!(" Disks ({selected}/{}) ", self.io.disks.disks.len()),
            false,
            false,
            area,
            buf,
        );

        let rows: Vec<Row> = self
            .io
            .disks
            .disks
            .iter()
            .map(|disk| Row::new(self.disks_usage_row(disk)))
            .collect();

        Table::new(rows, self.disks_usage_widths())
            .column_spacing(2)
            .header(Row::new(self.get_disks_usage_header()).fg(self.theme.table_headers))
            .row_highlight_style(get_selected_style(
                &self.theme,
                matches!(&self.state.selected_panel, SelectedPanel::DiskUsage),
            ))
            .render(area, buf, &mut self.tables.disks_table);
    }
}
