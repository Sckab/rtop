use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::app::state::Mode;

impl crate::app::App {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        let stats_layout =
            Layout::vertical([Constraint::Percentage(42), Constraint::Fill(1)]).split(layout[1]);

        let cpu_memory_layout =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(stats_layout[0]);

        let temperature_processes_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Fill(1)])
                .split(stats_layout[1]);

        self.render_header(layout[0], buf);

        self.render_cpu_usage(cpu_memory_layout[0], buf);
        self.render_memory_usage(cpu_memory_layout[1], buf);
        self.render_processes(temperature_processes_layout[1], buf);
        self.render_temperatures(temperature_processes_layout[0], buf);

        match &self.mode {
            Mode::Help => super::help::render_help_panel(self.keys.clone(), area, buf),
            Mode::ConfirmKill { request } => {
                super::popups::render_kill_confirmation(&request.name, &request.pid, area, buf)
            }
            Mode::Search { text_area } => {
                super::filter::render_process_filter_input(text_area, layout[1], buf);
            }
            _ => {}
        }

        if let Some(toast) = &self.toast {
            super::popups::render_toast(&toast.message, area, buf);
        }
    }
}
