use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct Statusbar {}

impl Statusbar {
	pub fn new() -> Statusbar {
		Statusbar {}
	}
}

impl Widget for Statusbar {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().draw(area, buf);
	}
}
