use std::collections::HashMap;
use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct BatteryWidget {
	title: String,
	update_interval: Duration,
	update_count: f64,

	battery_data: HashMap<String, Vec<(f64, f64)>>,
}

impl BatteryWidget {
	pub fn new() -> BatteryWidget {
		BatteryWidget {
			title: " Batteries ".to_string(),
			update_interval: Duration::from_secs(60),
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
