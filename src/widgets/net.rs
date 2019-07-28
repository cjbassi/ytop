use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::block;

pub struct NetWidget {
	title: String,
	pub update_interval: Ratio<u64>,

	interfaces: String,
}

impl NetWidget {
	pub fn new(interfaces: String) -> NetWidget {
		NetWidget {
			title: " Network Usage ".to_string(),
			update_interval: Ratio::from_integer(1),

			interfaces,
		}
	}

	pub async fn update(&mut self) {}
}

impl Widget for NetWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
