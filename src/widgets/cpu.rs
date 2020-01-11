use num_rational::Ratio;
use psutil::cpu;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Axis, Chart, Dataset, GraphType, Marker, Widget};

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct CpuWidget {
	title: String,
	update_interval: Ratio<u64>,
	update_count: u64,
	horizontal_scale: i64,

	cpu_count: usize,

	show_average: bool,
	show_percpu: bool,

	average_data: Vec<(f64, f64)>,
	percpu_data: Vec<Vec<(f64, f64)>>,

	collector: cpu::CpuPercentCollector,
}

impl CpuWidget {
	pub fn new(update_interval: Ratio<u64>, show_average: bool, show_percpu: bool) -> CpuWidget {
		let update_count = 0;

		let mut cpu_widget = CpuWidget {
			title: " CPU Usage ".to_string(),
			update_interval,
			update_count,
			horizontal_scale: 100,

			cpu_count: cpu::cpu_count() as usize,

			show_average,
			show_percpu,

			average_data: vec![(update_count as f64, 0.0)],
			percpu_data: Vec::new(),

			collector: cpu::CpuPercentCollector::new().unwrap(),
		};

		if !(show_average || show_percpu) {
			if cpu_widget.cpu_count <= 8 {
				cpu_widget.show_percpu = true
			} else {
				cpu_widget.show_average = true
			}
		}

		if cpu_widget.show_percpu {
			for _i in 0..cpu_widget.cpu_count {
				cpu_widget
					.percpu_data
					.push(vec![(update_count as f64, 0.0)]);
			}
		}

		cpu_widget
	}
}

impl UpdatableWidget for CpuWidget {
	fn update(&mut self) {
		self.update_count += 1;
		if self.show_average {
			let average_percent = self.collector.cpu_percent().unwrap();
			self.average_data
				.push((self.update_count as f64, average_percent.into()));
		}
		if self.show_percpu {
			let percpu_percents = self.collector.cpu_percent_percpu().unwrap();
			for i in 0..self.cpu_count {
				self.percpu_data[i].push((self.update_count as f64, percpu_percents[i].into()));
			}
		}
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for CpuWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		let mut datasets = Vec::new();
		if self.show_average {
			datasets.push(
				Dataset::default()
					.marker(Marker::Braille)
					.style(Style::default().fg(Color::Yellow))
					.graph_type(GraphType::Line)
					.data(&self.average_data),
			)
		}
		if self.show_percpu {
			for i in 0..self.cpu_count {
				datasets.push(
					Dataset::default()
						.marker(Marker::Braille)
						.graph_type(GraphType::Line)
						.style(Style::default().fg(Color::Yellow))
						.data(&self.percpu_data[i]),
				)
			}
		}

		Chart::<String, String>::default()
			.block(block::new().title(&self.title))
			.x_axis(Axis::default().bounds([
				self.update_count as f64 - 100.0,
				self.update_count as f64 + 1.0,
			]))
			.y_axis(Axis::default().bounds([0.0, 100.0]))
			.datasets(&datasets)
			.draw(area, buf);

		if self.show_average {
			buf.set_string(
				area.x + 3,
				area.y + 2,
				format!(
					"AVRG {:3.0}%",
					self.average_data[self.update_count as usize - 1].1
				),
				Style::default().fg(Color::Yellow),
			);
		}

		if self.show_percpu {
			let offset = if self.show_average { 1 } else { 0 };
			for i in 0..self.cpu_count {
				buf.set_string(
					area.x + 3,
					area.y + 2 + offset + i as u16,
					format!(
						"CPU{} {:3.0}%",
						i,
						self.percpu_data[i][self.update_count as usize - 1].1
					),
					Style::default().fg(Color::Yellow),
				);
			}
		}
	}
}
