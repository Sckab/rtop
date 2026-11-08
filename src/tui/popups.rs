use std::ffi::OsStr;

use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Span,
    widgets::{Clear, Paragraph, Widget},
};

pub fn render_kill_confirmation(name: &OsStr, pid: u32, area: Rect, buf: &mut Buffer) {
    let main_text = format!("Are you sure to kill \"{}\"?", name.display());

    let area = area.centered(
        Constraint::Length(main_text.cell_width() + 2),
        Constraint::Length(5),
    );

    Clear.render(area, buf);

    let popup_area = super::widgets::get_bordered_block(" Confirm ", false, true, area, buf);

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(popup_area);

    Paragraph::new(main_text).render(layout[0], buf);
    Paragraph::new(format!("PID: {pid}")).render(layout[1], buf);
    Paragraph::new(
        Span::raw("[Y] Yes").fg(Color::Red)
            + Span::raw(" ".repeat(4))
            + Span::raw("[N] No").fg(Color::Green),
    )
    .alignment(Alignment::Center)
    .render(layout[2], buf);
}

pub fn render_toast(message: &str, area: Rect, buf: &mut Buffer) {
    if area.width < 5 || area.height < 4 {
        return;
    }

    let width = (message.cell_width() + 4)
        .max(" Error ".len() as u16 + 2)
        .min(area.width.saturating_sub(2));

    let toast_area = Rect {
        x: area.x + area.width.saturating_sub(width + 2),
        y: area.y + 1,
        width,
        height: 3,
    };

    Clear.render(toast_area, buf);

    let inner_area = super::widgets::get_bordered_block(" Error ", true, true, toast_area, buf);

    Paragraph::new(message).render(inner_area, buf);
}
