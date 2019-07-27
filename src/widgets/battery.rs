use std::collections::HashMap;

use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct BatteryWidget {
	title: String,
	pub update_interval: Ratio<u64>,
	update_count: f64,

	battery_data: HashMap<String, Vec<(f64, f64)>>,
}

impl BatteryWidget {
	pub fn new() -> BatteryWidget {
		BatteryWidget {
			title: " Batteries ".to_string(),
			update_interval: Ratio::from_integer(60),
			update_count: 0.0,

			battery_data: HashMap::new(),
		}
	}

	pub async fn update(&mut self) {
		self.update_count += 1.0;
	}
}

impl Widget for BatteryWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
