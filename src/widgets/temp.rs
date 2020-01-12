use num_rational::Ratio;
use psutil::sensors;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{List, Text, Widget};

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct TempWidget {
	title: String,
	update_interval: Ratio<u64>,

	fahrenheit: bool,
	temp_data: Vec<(String, f64)>,
}

impl TempWidget {
	pub fn new(fahrenheit: bool) -> TempWidget {
		TempWidget {
			title: " Temperatures ".to_string(),
			update_interval: Ratio::from_integer(5),

			fahrenheit,
			temp_data: Vec::new(),
		}
	}
}

impl UpdatableWidget for TempWidget {
	fn update(&mut self) {
		self.temp_data = sensors::temperatures()
			.into_iter()
			.filter_map(|sensor| sensor.ok())
			.map(|sensor| {
				(
					sensor.unit().to_string(),
					if self.fahrenheit {
						sensor.current().fahrenheit()
					} else {
						sensor.current().celcius()
					},
				)
			})
			.filter(|data| data.1 > 0.0)
			.collect()
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for TempWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		List::new(self.temp_data.iter().map(|item| {
			Text::Raw(std::borrow::Cow::from(format!(
				"{:width$} {:2.0}{}",
				item.0.to_string(),
				item.1,
				if self.fahrenheit { "F" } else { "C" },
				width = area.width as usize - 6
			)))
		}))
		.block(block::new().title(&self.title))
		.style(Style::default().fg(Color::White))
		.draw(area, buf);
	}
}
