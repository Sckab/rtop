use ratatui::{
    buffer::{Buffer, CellWidth},
    layout::Rect,
    widgets::{Clear, Widget},
};

use crate::app::state::Mode;

impl crate::app::App {
    pub fn current_query(&self) -> Option<&str> {
        match &self.state.mode {
            Mode::Search { text_area } => Some(text_area.lines()[0].as_str()),
            _ => self.processes_state.process_query.as_deref(),
        }
    }
}

pub fn render_process_filter_input(
    text_area: &ratatui_textarea::TextArea,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = " Input Process Filter ";
    let width = title.cell_width() + 4;

    let area = Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.height / 3,
        width,
        height: 3,
    };

    Clear.render(area, buf);

    let inner_area = super::widgets::get_bordered_block(title, false, true, area, buf);

    text_area.render(inner_area, buf);
}
