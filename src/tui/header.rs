use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

impl crate::app::App {
    pub fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(8)]).split(area);

        let user_at_host = Span::raw(self.system_info.user_name.as_str());
        let at = Span::raw("@").fg(Color::Red).bold();
        let host = Span::raw(self.system_info.host_name.as_str()).fg(Color::Blue);
        let separator = Span::raw(" | ").fg(Color::Green);
        let os_name = Span::raw(self.system_info.os_name.as_str());

        Paragraph::new(Line::from(vec![user_at_host, at, host, separator, os_name]))
            .render(layout[0], buf);

        let now =
            time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());

        Paragraph::new(format!(
            "{:02}:{:02}:{:02}",
            now.hour(),
            now.minute(),
            now.second()
        ))
        .render(layout[1], buf);
    }
}
