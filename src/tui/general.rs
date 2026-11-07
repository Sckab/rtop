use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Widget},
};

pub fn get_borderd_block<S>(title: S, area: Rect, buf: &mut Buffer) -> Rect
where
    S: AsRef<str>,
{
    let title = title.as_ref();

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .merge_borders(MergeStrategy::Fuzzy)
        .title(title)
        .border_style(Style::new().fg(Color::Green));

    let inner_area = block.inner(area);
    block.render(area, buf);

    inner_area
}
