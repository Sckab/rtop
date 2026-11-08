use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Span,
    widgets::{Clear, Paragraph, Widget},
};

impl crate::app::App {
    pub fn render_kill_confirmation(&self, area: Rect, buf: &mut Buffer) {
        let main_text = format!(
            "Are you sure to kill \"{}\"?",
            self.states.selected_process_name.display()
        );

        let area = area.centered(
            Constraint::Length((main_text.cell_width() + 2) as u16),
            Constraint::Length(5),
        );

        ratatui::widgets::Widget::render(Clear, area, buf);

        let popup_area = super::widgets::get_bordered_block(" Confirm ", false, true, area, buf);

        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(popup_area);

        Paragraph::new(main_text).render(layout[0], buf);
        Paragraph::new(format!("PID: {}", self.states.selected_process_pid)).render(layout[1], buf);
        Paragraph::new(
            Span::raw("[Y] Yes").fg(Color::Red)
                + Span::raw(" ".repeat(4))
                + Span::raw("[N] No").fg(Color::Green),
        )
        .alignment(Alignment::Center)
        .render(layout[2], buf);
    }

    pub fn render_kill_failed(&self, area: Rect, buf: &mut Buffer) {
        let area = area.centered(
            Constraint::Length(self.failed_message.cell_width() + 2),
            Constraint::Length(3),
        );

        ratatui::widgets::Widget::render(Clear, area, buf);

        let popup_area = super::widgets::get_bordered_block(" Error ", true, true, area, buf);

        Paragraph::new(self.failed_message.clone()).render(popup_area, buf);
    }
}
