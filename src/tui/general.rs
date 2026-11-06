use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Widget},
};

pub fn get_borderd_block(title: &str, area: Rect, buf: &mut Buffer) -> Rect {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .merge_borders(MergeStrategy::Fuzzy)
        .title(title);

    let inner_area = block.inner(area);
    block.render(area, buf);

    inner_area
}
