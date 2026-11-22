use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::scrollbar::VERTICAL,
    text::Span,
    widgets::{
        Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, Widget,
    },
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

pub fn render_help_panel(keys: keymap::Keys, area: Rect, buf: &mut Buffer, scroll: &mut usize) {
    const COLUMN_SPACING: u16 = 2;

    let general_widths = get_maximum_widths(&keys.general);
    let sorting_processes_widths = get_maximum_widths(&keys.sorting_processes);
    let sorting_temperatures_widths = get_maximum_widths(&keys.sorting_temperatures);
    let sorting_disks_widths = get_maximum_widths(&keys.sorting_disks);

    let general_width =
        general_widths.0 + general_widths.1 + general_widths.2 + (2 * COLUMN_SPACING);

    let sorting_title = "Sorting (ascending, Shift for Descending)";
    let sorting_width = sorting_title
        .cell_width()
        .max(sorting_processes_widths.0 + sorting_processes_widths.2 + COLUMN_SPACING)
        .max(sorting_temperatures_widths.0 + sorting_temperatures_widths.2 + COLUMN_SPACING)
        .max(sorting_disks_widths.0 + sorting_disks_widths.2 + COLUMN_SPACING);

    let required_width = general_width.max(sorting_width);

    let general_height = keys.general.len() as u16 + 1; // include the table header
    let content_height = general_height
        + 1 // sorting title
        + 1
        + keys.sorting_processes.len() as u16
        + 1
        + keys.sorting_temperatures.len() as u16
        + 1
        + keys.sorting_disks.len() as u16;

    let width =
        (required_width + 2 + u16::from(content_height + 2 > area.height.saturating_sub(2)))
            .min(area.width.saturating_sub(2));
    let height = (content_height + 2).min(area.height.saturating_sub(2));

    if width < 3 || height < 3 {
        *scroll = 0;
        return;
    }

    let area_centered = area.centered(Constraint::Length(width), Constraint::Length(height));

    Clear.render(area_centered, buf);

    let has_scrollbar = height < content_height + 2;
    let title = if has_scrollbar {
        " Help (j/k to scroll) "
    } else {
        " Help "
    };

    let popup_area = get_bordered_block(title, false, true, area_centered, buf);

    // Render the full content off-screen, then copy only the visible rows into
    // the popup so the help can scroll without a `Paragraph`/`scroll` wrapper.
    let content_width = if has_scrollbar {
        popup_area.width.saturating_sub(1)
    } else {
        popup_area.width
    };
    let content_area = Rect::new(0, 0, content_width, content_height);
    let mut content_buf = Buffer::empty(content_area);

    let layout = Layout::vertical([
        Constraint::Length(general_height),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(keys.sorting_processes.len() as u16),
        Constraint::Length(1),
        Constraint::Length(keys.sorting_temperatures.len() as u16),
        Constraint::Length(1),
        Constraint::Length(keys.sorting_disks.len() as u16),
    ])
    .split(content_area);

    general_table(keys.general, COLUMN_SPACING, layout[0], &mut content_buf);

    Paragraph::new(sorting_title)
        .style(Style::default().fg(Color::Green))
        .render(layout[1], &mut content_buf);

    Paragraph::new("Processes:")
        .style(Style::default().fg(Color::Green))
        .render(layout[2], &mut content_buf);
    sorting_table(
        keys.sorting_processes,
        COLUMN_SPACING,
        layout[3],
        &mut content_buf,
    );

    Paragraph::new("Temperatures:")
        .style(Style::default().fg(Color::Green))
        .render(layout[4], &mut content_buf);
    sorting_table(
        keys.sorting_temperatures,
        COLUMN_SPACING,
        layout[5],
        &mut content_buf,
    );

    Paragraph::new("Disks:")
        .style(Style::default().fg(Color::Green))
        .render(layout[6], &mut content_buf);
    sorting_table(
        keys.sorting_disks,
        COLUMN_SPACING,
        layout[7],
        &mut content_buf,
    );

    let max_scroll = content_height.saturating_sub(popup_area.height) as usize;
    *scroll = (*scroll).min(max_scroll);
    let scroll = *scroll as u16;

    for y in 0..popup_area.height {
        let source_y = scroll + y;

        if source_y >= content_height {
            break;
        }

        for x in 0..content_width {
            if let Some(cell) = content_buf.cell((x, source_y)) {
                buf[(popup_area.x + x, popup_area.y + y)] = cell.clone();
            }
        }
    }

    if has_scrollbar {
        let scrollbar_area = Rect::new(
            popup_area.x + popup_area.width - 1,
            popup_area.y,
            1,
            popup_area.height,
        );

        // Map the scroll offset (0..=max_scroll) onto the range expected by
        // `ScrollbarState` (0..=content_length - 1) so the thumb reaches the
        // bottom when the last line is visible.
        let position = scroll as usize * (content_height as usize - 1) / max_scroll.max(1);

        let mut scrollbar_state = ScrollbarState::new(content_height as usize)
            .position(position)
            .viewport_content_length(popup_area.height as usize);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .symbols(VERTICAL)
            .begin_style(Style::new().fg(Color::Green))
            .end_style(Style::new().fg(Color::Green))
            .track_style(Style::new().fg(Color::DarkGray))
            .thumb_style(Style::new().fg(Color::Green));

        ratatui::widgets::StatefulWidget::render(
            scrollbar,
            scrollbar_area,
            buf,
            &mut scrollbar_state,
        );
    }
}
