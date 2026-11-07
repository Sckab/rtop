use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

impl super::App {
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

        let hours = Span::raw(format!("{:02}", now.hour()));
        let minutes = Span::raw(format!("{:02}", now.minute()));
        let seconds = Span::raw(format!("{:02}", now.second()));

        let clock_separator = Span::raw(":").fg(Color::Green);

        Paragraph::new(Line::from(vec![
            hours,
            clock_separator.clone(),
            minutes,
            clock_separator,
            seconds,
        ]))
        .render(layout[1], buf);
    }
}
