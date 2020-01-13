use std::collections::HashMap;

use battery::Manager;
use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Axis, Chart, Dataset, GraphType, Marker, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct BatteryWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	update_count: u64,
	battery_data: HashMap<String, Vec<(f64, f64)>>,
	manager: Manager,
}

impl BatteryWidget<'_> {
	pub fn new(colorscheme: &Colorscheme) -> BatteryWidget {
		BatteryWidget {
			title: " Batteries ".to_string(),
			update_interval: Ratio::from_integer(60),
			colorscheme,

			update_count: 0,
			battery_data: HashMap::new(),
			manager: Manager::new().unwrap(),
		}
	}
}

impl UpdatableWidget for BatteryWidget<'_> {
	fn update(&mut self) {
		self.update_count += 1;
		let mut current_batteries = Vec::new();

		for battery in self.manager.batteries().unwrap() {
			let battery = battery.unwrap();
			let model = battery.model().unwrap();
			self.battery_data
				.entry(model.to_string())
				.or_default()
				.push((
					self.update_count as f64,
					battery.state_of_charge().value as f64,
				));
			current_batteries.push(model.to_string());
		}

		let models: Vec<String> = self.battery_data.keys().cloned().collect();
		for model in models {
			if !current_batteries.contains(&model) {
				self.battery_data.remove(&model);
			}
		}
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for BatteryWidget<'_> {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		let datasets: Vec<Dataset> = self
			.battery_data
			.values()
			.enumerate()
			.map(|(i, data)| {
				Dataset::default()
					.marker(Marker::Braille)
					.style(self.colorscheme.battery_lines[i % self.colorscheme.battery_lines.len()])
					.graph_type(GraphType::Line)
					.data(&data)
			})
			.collect();

		Chart::<String, String>::default()
			.block(block::new(self.colorscheme, &self.title))
			.x_axis(Axis::default().bounds([
				self.update_count as f64 - 100.0,
				self.update_count as f64 + 1.0,
			]))
			.y_axis(Axis::default().bounds([0.0, 100.0]))
			.datasets(&datasets)
			.draw(area, buf);

		for (i, data) in self.battery_data.iter().enumerate() {
			buf.set_string(
				area.x + 3,
				area.y + 2 + i as u16,
				format!("{} {:3.0}%", data.0, data.1.last().unwrap().1),
				self.colorscheme.battery_lines[i % self.colorscheme.battery_lines.len()],
			);
		}
	}
}
