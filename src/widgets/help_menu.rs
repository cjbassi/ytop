use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

const TITLE: &str = " Help Menu ";

pub struct HelpMenu {}

impl HelpMenu {
    pub fn new() -> HelpMenu {
        HelpMenu {}
    }
}

impl Widget for HelpMenu {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(TITLE).draw(area, buf);
    }
}
