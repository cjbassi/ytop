use futures::try_join;
use heim::memory;
use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Axis, Chart, Dataset, Marker, Widget};

use super::block;

#[derive(Default)]
struct MemData {
	total: u64,
	used: u64,
	percents: Vec<(f64, f64)>,
}

pub struct MemWidget {
	title: String,
	pub update_interval: Ratio<u64>,
	update_count: f64,

	main: MemData,
	swap: MemData,
}

impl MemWidget {
	pub fn new(update_interval: Ratio<u64>) -> MemWidget {
		MemWidget {
			title: " Memory Usage ".to_string(),
			update_interval,
			update_count: 0.0,

			main: MemData::default(),
			swap: MemData::default(),
		}
	}

	pub async fn update(&mut self) {
		self.update_count += 1.0;

		let main = memory::memory();
		let swap = memory::swap();
		let (main, swap) = try_join!(main, swap).unwrap();

		self.main.total = main.total().get();
		self.main.used = self.main.total - main.available().get();
		self.main.percents.push((
			self.update_count,
			self.main.used as f64 / self.main.total as f64,
		));

		self.swap.total = swap.total().get();
		self.swap.used = swap.used().get();
		self.swap.percents.push((
			self.update_count,
			self.swap.used as f64 / self.swap.total as f64,
		));
	}
}

impl Widget for MemWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		Chart::<String, String>::default()
			.block(block::new().title(&self.title))
			.x_axis(Axis::default().bounds([self.update_count - 100.0, self.update_count + 1.0]))
			.y_axis(Axis::default().bounds([0.0, 100.0]))
			.datasets(&[
				Dataset::default()
					.name("Main")
					.marker(Marker::Braille)
					.style(Style::default().fg(Color::Yellow))
					.data(&self.main.percents),
				Dataset::default()
					.name("Swap")
					.marker(Marker::Braille)
					.style(Style::default().fg(Color::Blue))
					.data(&self.swap.percents),
			])
			.draw(area, buf);
	}
}
