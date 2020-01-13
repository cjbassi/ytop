use num_rational::Ratio;
use psutil::sensors;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{List, Text, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct TempWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	fahrenheit: bool,
	temp_data: Vec<(String, f64)>,
}

impl TempWidget<'_> {
	pub fn new(colorscheme: &Colorscheme, fahrenheit: bool) -> TempWidget {
		TempWidget {
			title: " Temperatures ".to_string(),
			update_interval: Ratio::from_integer(5),
			colorscheme,

			fahrenheit,
			temp_data: Vec::new(),
		}
	}
}

impl UpdatableWidget for TempWidget<'_> {
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

impl Widget for TempWidget<'_> {
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
		.block(block::new(self.colorscheme, &self.title))
		.style(self.colorscheme.text)
		.draw(area, buf);
	}
}
