use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Widget},
};

use crate::theme::Theme;

pub fn get_bordered_block<S>(
    theme: &Theme,
    title: S,
    error: bool,
    title_center: bool,
    area: Rect,
    buf: &mut Buffer,
) -> Rect
where
    S: AsRef<str>,
{
    let mut title = title.as_ref();
    let mut color = &theme.primary;

    if error {
        title = " Error ";
        color = &theme.error;
    }

    let mut block = Block::bordered()
        .border_type(BorderType::Rounded)
        .merge_borders(MergeStrategy::Fuzzy)
        .title(title)
        .border_style(Style::new().fg(color.into()));

    if title_center || error {
        block = block.title_alignment(Alignment::Center);
    }

    let inner_area = block.inner(area);
    block.render(area, buf);

    inner_area
}

pub fn get_selected_style(theme: &Theme, selected: bool) -> Style {
    if selected {
        Style::new()
            .fg(theme.selected_fg.into())
            .bg(theme.selected_bg.into())
    } else {
        Style::new()
            .fg(theme.not_selected_fg.into())
            .bg(theme.not_selected_bg.into())
    }
}
