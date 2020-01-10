use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct TempWidget {
	title: String,
	update_interval: Ratio<u64>,
	update_count: f64,

	fahrenheit: bool,
	temp_data: Vec<(String, Vec<(f64, f64)>)>,
}

impl TempWidget {
	pub fn new(fahrenheit: bool) -> TempWidget {
		TempWidget {
			title: " Temperatures ".to_string(),
			update_interval: Ratio::from_integer(5),
			update_count: 0.0,

			fahrenheit,
			temp_data: Vec::new(),
		}
	}
}

impl UpdatableWidget for TempWidget {
	fn update(&mut self) {
		self.update_count += 1.0;
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for TempWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
