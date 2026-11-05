use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Paragraph, Widget},
};

impl Widget for &super::App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(area);

        let info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        let topbar_layout =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(8)]).split(layout[0]);

        Paragraph::new(format!(
            "{}@{} | {}",
            self.system_info.user_name, self.system_info.host_name, self.system_info.os_name
        ))
        .render(topbar_layout[0], buf);

        let formatter = match time::format_description::parse_owned::<3>("[hour]:[minute]:[second]")
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };

        let now =
            time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());

        let formatted_time = now.format(&formatter).unwrap_or_default();

        Paragraph::new(formatted_time).render(topbar_layout[1], buf);

        self.render_cpu_usage(info_layout[0], buf);
        self.render_memory_usage(info_layout[1], buf);

        if self.show_help {
            self.render_help_panel(area, buf);
        }
    }
}
