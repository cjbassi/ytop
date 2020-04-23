use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

#[cfg(target_os = "linux")]
use psutil::sensors;

pub struct TempWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	fahrenheit: bool,
	temp_threshold: f64,

	temp_data: Vec<(String, f64)>,
}

impl TempWidget<'_> {
	pub fn new(colorscheme: &Colorscheme, fahrenheit: bool) -> TempWidget {
		TempWidget {
			title: " Temperatures ".to_string(),
			update_interval: Ratio::from_integer(5),
			colorscheme,

			fahrenheit,
			temp_threshold: 80.0,
			temp_data: Vec::new(),
		}
	}
}

impl UpdatableWidget for TempWidget<'_> {
	#[cfg(target_os = "linux")]
	fn update(&mut self) {
		self.temp_data = sensors::temperatures()
			.into_iter()
			.filter_map(|sensor| sensor.ok())
			.map(|sensor| {
				(
					match sensor.label() {
						Some(label) => format!("{}-{}", sensor.unit(), label),
						None => sensor.unit().to_string(),
					},
					if self.fahrenheit {
						sensor.current().fahrenheit()
					} else {
						sensor.current().celsius()
					},
				)
			})
			.filter(|data| data.1 > 0.0)
			.collect()
	}

	#[cfg(target_os = "macos")]
	fn update(&mut self) {}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl<'a> Widget for &TempWidget<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		block::new(self.colorscheme, &self.title).render(area, buf);

		if area.height < 2 {
			return;
		}

		let inner = Rect {
			x: area.x + 1,
			y: area.y + 1,
			width: area.width - 2,
			height: area.height - 2,
		};

		for (i, (label, data)) in self.temp_data.iter().enumerate() {
			if i >= inner.height as usize {
				break;
			}
			let y = inner.y + i as u16;
			buf.set_string(inner.x, y, label, self.colorscheme.text);
			buf.set_string(
				inner.right() - 5,
				y,
				format!("{:3.0}°{}", data, if self.fahrenheit { "F" } else { "C" },),
				if data < &self.temp_threshold {
					self.colorscheme.temp_low
				} else {
					self.colorscheme.temp_high
				},
			);
		}
	}
}
