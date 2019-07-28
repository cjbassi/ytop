use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::block;

pub struct ProcWidget {
	title: String,
	pub update_interval: Ratio<u64>,
}

impl ProcWidget {
	pub fn new() -> ProcWidget {
		ProcWidget {
			title: " Processes ".to_string(),
			update_interval: Ratio::from_integer(1),
		}
	}

	pub async fn update(&mut self) {}
}

impl Widget for ProcWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
