use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Span,
    widgets::{Clear, Paragraph, Widget},
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

        if self.show_kill_confirmation {
            let main_text = format!(
                "Are you sure to kill \"{}\"",
                self.states.selected_process_name.display()
            );

            let area = area.centered(
                Constraint::Length((main_text.cell_width() + 2) as u16),
                Constraint::Length(5),
            );

            ratatui::widgets::Widget::render(Clear, area, buf);

            let popup_area = super::general::get_borderd_block(" Confirm ", area, buf);

            let layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(popup_area);

            Paragraph::new(main_text).render(layout[0], buf);
            Paragraph::new(format!("PID: {}", self.states.selected_process_pid))
                .render(layout[1], buf);
            Paragraph::new(
                Span::raw("[Y] Yes").fg(Color::Red)
                    + Span::raw(" ".repeat(4))
                    + Span::raw("[N] No").fg(Color::Green),
            )
            .alignment(Alignment::Center)
            .render(layout[2], buf);
        }
    }
}
