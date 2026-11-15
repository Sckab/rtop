use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Cell, Clear, Paragraph, Row, Table, Widget},
};

use crate::{app::keymap, tui::widgets::get_bordered_block};

fn get_maximum_widths(keys: &[keymap::Key]) -> (u16, u16, u16) {
    let mut primary_width = "Primary".cell_width();
    let mut secondary_width = "Secondary".cell_width();
    let mut description_width = "Description".cell_width();

    for key in keys {
        primary_width = primary_width.max(key.primary_key.cell_width());
        secondary_width = secondary_width.max(key.secondary_key.cell_width());
        description_width = description_width.max(key.description.cell_width());
    }

    (primary_width, secondary_width, description_width)
}

fn general_table(keys: Vec<keymap::Key>, spacing: u16, area: Rect, buf: &mut Buffer) {
    let (primary_width, secondary_width, _) = get_maximum_widths(&keys);

    let rows = keys.iter().map(|key| {
        let primary = Span::raw(key.primary_key.clone());
        let secondary = Span::raw(key.secondary_key.clone());
        let description = Span::raw(key.description.clone());

        Row::new([
            Cell::from(primary),
            Cell::from(secondary),
            Cell::from(description),
        ])
    });

    let header = Row::new([
        Span::raw("Primary").style(Style::default().fg(Color::Green)),
        Span::raw("Secondary").style(Style::default().fg(Color::Green)),
        Span::raw("Description").style(Style::default().fg(Color::Green)),
    ]);

    Table::new(
        rows,
        [
            Constraint::Length(primary_width),
            Constraint::Length(secondary_width),
            Constraint::Fill(1),
        ],
    )
    .header(header)
    .column_spacing(spacing)
    .render(area, buf);
}

fn sorting_table(keys: Vec<keymap::Key>, spacing: u16, area: Rect, buf: &mut Buffer) {
    let (primary_width, secondary_width, _) = get_maximum_widths(&keys);

    let rows = keys.iter().map(|key| {
        let primary = Span::raw(key.primary_key.clone());
        let description = Span::raw(key.description.clone());

        Row::new([Cell::from(primary), Cell::from(description)])
    });

    Table::new(
        rows,
        [
            Constraint::Length(primary_width + secondary_width + spacing),
            Constraint::Fill(1),
        ],
    )
    .column_spacing(spacing)
    .render(area, buf);
}

pub fn render_help_panel(keys: keymap::Keys, area: Rect, buf: &mut Buffer) {
    const COLUMN_SPACING: u16 = 2;

    let general_widths = get_maximum_widths(&keys.general);
    let sorting_processes_widths = get_maximum_widths(&keys.sorting_processes);
    let sorting_temperatures_widths = get_maximum_widths(&keys.sorting_temperatures);

    let general_width =
        general_widths.0 + general_widths.1 + general_widths.2 + (2 * COLUMN_SPACING);

    let sorting_title = "Sorting (ascending, Shift for Descending)";
    let sorting_width = sorting_title.cell_width();

    let sorting_width = sorting_width
        .max(sorting_processes_widths.0 + sorting_processes_widths.2 + COLUMN_SPACING)
        .max(sorting_temperatures_widths.0 + sorting_temperatures_widths.2 + COLUMN_SPACING);

    let width = general_width.max(sorting_width) + 2;

    let general_height = keys.general.len() as u16 + 1; // include the table header
    let inner_height = general_height
        + 1 // sorting title
        + 1
        + keys.sorting_processes.len() as u16
        + 1
        + keys.sorting_temperatures.len() as u16;
    let height = inner_height + 2; // borders

    let area_centered = area.centered(Constraint::Length(width), Constraint::Length(height));

    Clear.render(area_centered, buf);

    let popup_area = get_bordered_block(" Help ", false, true, area_centered, buf);

    let layout = Layout::vertical([
        Constraint::Length(general_height),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(keys.sorting_processes.len() as u16),
        Constraint::Length(1),
        Constraint::Length(keys.sorting_temperatures.len() as u16),
    ])
    .split(popup_area);

    general_table(keys.general, COLUMN_SPACING, layout[0], buf);

    Paragraph::new(sorting_title)
        .style(Style::default().fg(Color::Green))
        .render(layout[1], buf);

    Paragraph::new("Processes:")
        .style(Style::default().fg(Color::Green))
        .render(layout[2], buf);
    sorting_table(keys.sorting_processes, COLUMN_SPACING, layout[3], buf);

    Paragraph::new("Temperatures:")
        .style(Style::default().fg(Color::Green))
        .render(layout[4], buf);
    sorting_table(keys.sorting_temperatures, COLUMN_SPACING, layout[5], buf);
}
