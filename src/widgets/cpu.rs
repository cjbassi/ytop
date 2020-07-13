use num_rational::Ratio;
use psutil::cpu;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::symbols::Marker;
use tui::widgets::{Axis, Chart, Dataset, GraphType, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

const HORIZONTAL_SCALE_DELTA: u64 = 25;

pub struct CpuWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	horizontal_scale: u64,
	max_scale: u64,

	update_count: u64,

	cpu_count: usize,

	show_average: bool,
	show_percpu: bool,

	average_data: Vec<(f64, f64)>,
	percpu_data: Vec<Vec<(f64, f64)>>,

	collector: cpu::CpuPercentCollector,
}

impl CpuWidget<'_> {
	pub fn new(
		colorscheme: &Colorscheme,
		update_interval: Ratio<u64>,
		show_average: bool,
		show_percpu: bool,
	) -> CpuWidget {
		let update_count = 0;
		let default_scale = 100;

		let mut cpu_widget = CpuWidget {
			title: " CPU Usage ".to_string(),
			update_interval,
			colorscheme,

			horizontal_scale: default_scale,
			max_scale: default_scale,

			update_count,

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

	pub fn scale_in(&mut self) {
		if self.horizontal_scale > HORIZONTAL_SCALE_DELTA {
			self.horizontal_scale -= HORIZONTAL_SCALE_DELTA;
		}
	}

	pub fn scale_out(&mut self) {
		self.horizontal_scale += HORIZONTAL_SCALE_DELTA;

		self.max_scale = std::cmp::max(self.max_scale, self.horizontal_scale);
	}
}

impl UpdatableWidget for CpuWidget<'_> {
	fn update(&mut self) {
		self.update_count += 1;
		if self.show_average {
			let average_percent = self.collector.cpu_percent().unwrap();
			self.average_data
				.push((self.update_count as f64, average_percent.into()));
		}

		// Get rid of old samples
		while self.average_data.len() > self.max_scale as usize {
			self.average_data.remove(0);
		}

		if self.show_percpu {
			let percpu_percents = self.collector.cpu_percent_percpu().unwrap();
			if percpu_percents.len() != self.cpu_count {
				// TODO
			} else {
				for (cpu, cpu_percent) in self.percpu_data.iter_mut().zip(percpu_percents) {
					cpu.push((self.update_count as f64, cpu_percent.into()));

					// Get rid of old samples
					while cpu.len() > self.max_scale as usize {
						cpu.remove(0);
					}
				}
			}
		}
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for &CpuWidget<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let mut datasets = Vec::new();
		if self.show_average {
			datasets.push(
				Dataset::default()
					.marker(Marker::Braille)
					.graph_type(GraphType::Line)
					.style(self.colorscheme.cpu_lines[0])
					.data(&self.average_data),
			)
		}
		if self.show_percpu {
			let offset = if self.show_average { 1 } else { 0 };
			for i in 0..self.cpu_count {
				datasets.push(
					Dataset::default()
						.marker(Marker::Braille)
						.graph_type(GraphType::Line)
						.style(
							self.colorscheme.cpu_lines
								[(i + offset as usize) % self.colorscheme.cpu_lines.len()],
						)
						.data(&self.percpu_data[i]),
				)
			}
		}

		Chart::<String, String>::default()
			.block(block::new(self.colorscheme, &self.title))
			.x_axis(Axis::default().bounds([
				self.update_count as f64 - self.horizontal_scale as f64,
				self.update_count as f64 + 1.0,
			]))
			.y_axis(Axis::default().bounds([0.0, 100.0]))
			.datasets(&datasets)
			.render(area, buf);

		if self.show_average {
			buf.set_string(
				area.x + 3,
				area.y + 2,
				format!("AVRG {:3.0}%", self.average_data.last().unwrap().1),
				self.colorscheme.cpu_lines[0],
			);
		}

		if self.show_percpu {
			let offset = if self.show_average { 1 } else { 0 };
			for i in 0..self.cpu_count {
				let y = area.y + 2 + offset + i as u16;
				if y >= area.bottom() - 1 {
					break;
				}
				buf.set_string(
					area.x + 3,
					y,
					format!("CPU{} {:3.0}%", i, self.percpu_data[i].last().unwrap().1),
					self.colorscheme.cpu_lines
						[(i + offset as usize) % self.colorscheme.cpu_lines.len()],
				);
			}
		}
	}
}
