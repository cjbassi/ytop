use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct NetWidget {
	title: String,
	update_interval: Duration,
}

impl NetWidget {
	pub fn new() -> NetWidget {
		NetWidget {
			title: " Network Usage ".to_string(),
			update_interval: Duration::from_secs(1),
		}
	}

	pub async fn update(&mut self) {}
}

impl Widget for NetWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
