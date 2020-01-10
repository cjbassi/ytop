use num_rational::Ratio;
use psutil::cpu;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Axis, Chart, Dataset, Marker, Widget};

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct CpuWidget {
	title: String,
	update_interval: Ratio<u64>,
	update_count: f64,
	horizontal_scale: i64,

	cpu_count: usize,

	show_average: bool,
	show_percpu: bool,

	average_data: Vec<(f64, f64)>,
	percpu_data: Vec<Vec<(f64, f64)>>,

	average_label: String,
	percpu_labels: Vec<String>,

	average_collector: cpu::CpuPercentCollector,
	percpu_collector: cpu::CpuPercentCollector,
}

impl CpuWidget {
	pub fn new(update_interval: Ratio<u64>, show_average: bool, show_percpu: bool) -> CpuWidget {
		let mut cpu_widget = CpuWidget {
			title: " CPU Usage ".to_string(),
			update_interval,
			update_count: 0.0,
			horizontal_scale: 100,

			cpu_count: cpu::cpu_count() as usize,

			show_average,
			show_percpu,

			average_data: Vec::new(),
			percpu_data: Vec::new(),

			average_label: "AVRG".to_string(),
			percpu_labels: Vec::new(),

			average_collector: cpu::CpuPercentCollector::new().unwrap(),
			percpu_collector: cpu::CpuPercentCollector::new().unwrap(),
		};

		if !(show_average || show_percpu) {
			if cpu_widget.cpu_count <= 8 {
				cpu_widget.show_percpu = true
			} else {
				cpu_widget.show_average = true
			}
		}

		if cpu_widget.show_percpu {
			for i in 0..cpu_widget.cpu_count {
				cpu_widget.percpu_labels.push(format!("CPU{}", i));
				cpu_widget.percpu_data.push(Vec::new());
			}
		}

		cpu_widget
	}
}

impl UpdatableWidget for CpuWidget {
	fn update(&mut self) {
		self.update_count += 1.0;
		if self.show_average {
			let average_percent = self.average_collector.cpu_percent().unwrap();
			self.average_data
				.push((self.update_count, average_percent.into()));
		}
		if self.show_percpu {
			let percpu_percents = self.percpu_collector.cpu_percent_percpu().unwrap();
			for i in 0..self.cpu_count {
				self.percpu_data[i].push((self.update_count, percpu_percents[i].into()));
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
					.name(&self.average_label)
					.marker(Marker::Braille)
					.style(Style::default().fg(Color::Yellow))
					.data(&self.average_data),
			)
		}
		if self.show_percpu {
			for i in 0..self.cpu_count {
				datasets.push(
					Dataset::default()
						.name(&self.percpu_labels[i])
						.marker(Marker::Braille)
						.style(Style::default().fg(Color::Yellow))
						.data(&self.percpu_data[i]),
				)
			}
		}

		Chart::<String, String>::default()
			.block(block::new().title(&self.title))
			.x_axis(Axis::default().bounds([self.update_count - 100.0, self.update_count + 1.0]))
			.y_axis(Axis::default().bounds([0.0, 100.0]))
			.datasets(&datasets)
			.draw(area, buf);
	}
}
