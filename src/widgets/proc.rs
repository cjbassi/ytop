use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::{block, WidgetUpdate};

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
}

impl WidgetUpdate for ProcWidget {
	fn update(&mut self) {}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for ProcWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
