use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct ProcWidget {
	title: String,
	update_interval: Ratio<u64>,
}

impl ProcWidget {
	pub fn new() -> ProcWidget {
		ProcWidget {
			title: " Processes ".to_string(),
			update_interval: Ratio::from_integer(1),
		}
	}
}

impl UpdatableWidget for ProcWidget {
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
