use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
};

impl super::App {
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

        if self.show_help {
            self.render_help_panel(area, buf);
        }

        if self.show_kill_failed {
            self.render_kill_failed(area, buf);
        }

        if self.show_kill_confirmation {
            self.render_kill_confirmation(area, buf);
        }
    }
}
