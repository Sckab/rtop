use ratatui::{buffer::Buffer, layout::Rect};

impl super::App {
    pub fn render_processes(&self, area: Rect, buf: &mut Buffer) {
        super::general::get_borderd_block(" Processes ", area, buf);
    }
}
