use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::state::Mode;

impl crate::app::App {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(area);

        let info_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(42), Constraint::Fill(1)])
            .split(layout[1]);

        let usage_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(info_layout[0]);

        self.render_header(layout[0], buf);

        self.render_cpu_usage(usage_layout[0], buf);
        self.render_memory_usage(usage_layout[1], buf);
        self.render_processes(info_layout[1], buf);

        match &self.mode {
            Mode::Help => {
                super::help::render_help_panel(&self.keys, &mut self.tables.help_table, area, buf)
            }
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
