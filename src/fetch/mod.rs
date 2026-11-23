use ratatui::{
    Terminal, TerminalOptions, Viewport,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Span,
    widgets::{Paragraph, Table, Widget},
};
use std::io::stdout;

mod get_distro_logo;
mod get_maximum_frequency;
mod info;

use crate::theme::{Theme, color::Color};

pub fn fetch(theme: Theme) -> anyhow::Result<()> {
    let distro_logo = get_distro_logo::get_distro_logo();

    let (color, art) = distro_logo.split_once('\n').unwrap();

    let color = Color::try_from(color.to_string())?;
    let longest_line = art.lines().map(|l| l.len()).max().unwrap_or(0);

    let height = art.lines().count() as u16;

    let formatter = humansize::make_format(humansize::DECIMAL);

    let info = info::Info::new(&formatter)?;

    let rows = info.get_rows(&theme, &formatter);

    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(height.max(rows.len() as u16)),
        },
    )?;

    terminal.draw(|frame| {
        let area = frame.area();
        let buf = frame.buffer_mut();

        let [art_area, info_area] = Layout::horizontal([
            Constraint::Length(longest_line as u16 + 3),
            Constraint::Fill(1),
        ])
        .areas(area);

        Paragraph::new(art).fg(color).render(art_area, buf);

        let [header, divider, table] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(info_area);

        Paragraph::new(
            Span::raw(&info.user_name)
                + Span::raw("@").fg(theme.error)
                + Span::raw(&info.host_name).fg(theme.secondary),
        )
        .render(header, buf);
        Paragraph::new("-".repeat(info.user_name.len() + 1 + info.host_name.len()))
            .render(divider, buf);

        Table::new(
            rows,
            [
                Constraint::Length("Frequency".len() as u16),
                Constraint::Fill(1),
            ],
        )
        .column_spacing(2)
        .render(table, buf);
    })?;

    println!();

    Ok(())
}
