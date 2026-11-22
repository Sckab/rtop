use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::{app::state::Mode, tui::NETWORK_PANEL_PERCENT};

impl crate::app::App {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, body_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

        // Two border rows and one header row, sized on the longer of the two panels.
        let bottom_height = (self
            .temperature_state
            .temperatures
            .len()
            .max(self.io.disks.disks.len()) as u16
            + 3)
        .clamp(5, 12)
        .min(body_area.height / 3);

        let [stats_area, processes_area, bottom_area] = Layout::vertical([
            Constraint::Percentage(35),
            Constraint::Fill(1),
            Constraint::Length(bottom_height),
        ])
        .areas(body_area);

        let [network_area, processes_area] = Layout::horizontal([
            Constraint::Percentage(NETWORK_PANEL_PERCENT),
            Constraint::Fill(1),
        ])
        .areas(processes_area);

        let [cpu_area, memory_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(stats_area);

        let temperatures_width = self
            .temperatures_panel_width()
            .min((bottom_area.width / 3).max(1));

        let [temperatures_area, disks_area] =
            Layout::horizontal([Constraint::Length(temperatures_width), Constraint::Fill(1)])
                .areas(bottom_area);

        self.render_header(header_area, buf);

        self.render_cpu_usage(cpu_area, buf);
        self.render_memory_usage(memory_area, buf);

        self.render_network(network_area, buf);
        self.render_processes(processes_area, buf);

        self.render_temperatures(temperatures_area, buf);
        self.render_disks_usage(disks_area, buf);

        match &mut self.state.mode {
            Mode::Help { scroll } => {
                super::help::render_help_panel(&self.theme, self.keys.clone(), area, buf, scroll)
            }
            Mode::ConfirmKill { request } => super::popups::render_kill_confirmation(
                &self.theme,
                &request.name,
                &request.pid,
                area,
                buf,
            ),
            Mode::Search { text_area } => {
                super::filter::render_process_filter_input(&self.theme, text_area, body_area, buf);
            }
            _ => {}
        }

        if let Some(toast) = &self.toast {
            super::popups::render_toast(&self.theme, &toast.message, area, buf);
        }
    }
}
