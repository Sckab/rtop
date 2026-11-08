use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Widget},
};

pub fn get_bordered_block<S>(
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
    let mut color = Color::Green;

    if error {
        title = " Error ";
        color = Color::Red;
    }

    let mut block = Block::bordered()
        .border_type(BorderType::Rounded)
        .merge_borders(MergeStrategy::Fuzzy)
        .title(title)
        .border_style(Style::new().fg(color));

    if title_center || error {
        block = block.title_alignment(Alignment::Center);
    }

    let inner_area = block.inner(area);
    block.render(area, buf);

    inner_area
}
