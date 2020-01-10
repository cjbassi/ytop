use std::collections::HashMap;

use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct BatteryWidget {
	title: String,
	update_interval: Ratio<u64>,
	update_count: u64,

	battery_data: HashMap<String, Vec<(f64, f64)>>,
}

impl BatteryWidget {
	pub fn new() -> BatteryWidget {
		BatteryWidget {
			title: " Batteries ".to_string(),
			update_interval: Ratio::from_integer(60),
			update_count: 0,

			battery_data: HashMap::new(),
		}
	}
}

impl UpdatableWidget for BatteryWidget {
	fn update(&mut self) {
		self.update_count += 1;
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for BatteryWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
