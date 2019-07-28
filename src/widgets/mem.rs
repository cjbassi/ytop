use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::block;

pub struct MemWidget {
	title: String,
	pub update_interval: Ratio<u64>,
	update_count: f64,
}

impl MemWidget {
	pub fn new(update_interval: Ratio<u64>) -> MemWidget {
		MemWidget {
			title: " Memory Usage ".to_string(),
			update_interval,
			update_count: 0.0,
		}
	}

	pub async fn update(&mut self) {
		self.update_count += 1.0;
	}
}

impl Widget for MemWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
