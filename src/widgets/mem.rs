use futures::try_join;
use heim::memory;
use num_rational::Ratio;
use size::{Base, Size};
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
		let mut main = MemData::default();
		let mut swap = MemData::default();
		main.percents.push((1.0, 0.0));
		swap.percents.push((1.0, 0.0));

		MemWidget {
			title: " Memory Usage ".to_string(),
			update_interval,
			update_count: 1.0,

			main,
			swap,
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
			(self.main.used * 100 / self.main.total) as f64,
		));

		self.swap.total = swap.total().get();
		self.swap.used = swap.used().get();
		self.swap.percents.push((
			self.update_count,
			(self.swap.used * 100 / self.swap.total) as f64,
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
					.marker(Marker::Braille)
					.style(Style::default().fg(Color::Yellow))
					.data(&self.main.percents),
				Dataset::default()
					.marker(Marker::Braille)
					.style(Style::default().fg(Color::Blue))
					.data(&self.swap.percents),
			])
			.draw(area, buf);

		buf.set_string(
			area.x + 3,
			area.y + 2,
			format!(
				"Main {:3.0}% {}/{}",
				self.main.percents.last().unwrap().1,
				Size::Bytes(self.main.used),
				Size::Bytes(self.main.total),
			),
			Style::default().fg(Color::Yellow),
		);

		buf.set_string(
			area.x + 3,
			area.y + 3,
			format!(
				"Swap {:3.0}% {}/{}",
				self.swap.percents.last().unwrap().1,
				Size::Bytes(self.swap.used),
				Size::Bytes(self.swap.total),
			),
			Style::default().fg(Color::Blue),
		);
	}
}
