use std::collections::HashMap;
use std::path::PathBuf;

use num_rational::Ratio;
use psutil::disk;
use size::Size;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::Modifier;
use tui::widgets::{Row, Table, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

#[derive(Clone)]
struct Partition {
	name: String,
	mountpoint: PathBuf,
	bytes_read: u64,
	bytes_written: u64,
	bytes_read_recently: u64,
	bytes_written_recently: u64,
	used_percent: f32,
	bytes_free: u64,
}

pub struct DiskWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	partitions: HashMap<String, Partition>,

	collector: disk::DiskIoCountersCollector,
}

impl DiskWidget<'_> {
	pub fn new(colorscheme: &Colorscheme) -> DiskWidget {
		DiskWidget {
			title: " Disk Usage ".to_string(),
			update_interval: Ratio::from_integer(1),
			colorscheme,

			partitions: HashMap::new(),

			collector: disk::DiskIoCountersCollector::default(),
		}
	}
}

impl UpdatableWidget for DiskWidget<'_> {
	#[cfg(target_os = "linux")]
	fn update(&mut self) {
		let mut io_counters_perdisk = self.collector.disk_io_counters_per_partition().unwrap();
		self.partitions = disk::partitions_physical()
			.unwrap()
			.into_iter()
			.rev() // fixes the mountpoint when the partition is mounted multiple times (#25)
			.map(|partition| {
				let name = PathBuf::from(partition.device())
					.file_name()
					.unwrap()
					.to_string_lossy()
					.to_string();
				let mountpoint = partition.mountpoint().to_path_buf();

				let disk_usage = disk::disk_usage(&mountpoint).unwrap();
				let io_counters = io_counters_perdisk
					.remove(&partition.device().replace("/dev/", ""))
					.unwrap_or_default();

				let bytes_read = io_counters.read_count();
				let bytes_written = io_counters.read_count();
				let (bytes_read_recently, bytes_written_recently) = self
					.partitions
					.get(&name)
					.map(|other| {
						(
							bytes_read - other.bytes_read,
							bytes_written - other.bytes_written,
						)
					})
					.unwrap_or_default();
				let used_percent = disk_usage.percent();
				let bytes_free = disk_usage.free();

				(
					name.clone(),
					Partition {
						name,
						mountpoint,
						bytes_read,
						bytes_written,
						bytes_read_recently,
						bytes_written_recently,
						used_percent,
						bytes_free,
					},
				)
			})
			.collect();
	}

	#[cfg(target_os = "macos")]
	fn update(&mut self) {}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for DiskWidget<'_> {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		let mut partitions: Vec<Partition> = self.partitions.values().cloned().collect();
		partitions.sort_by(|a, b| a.name.cmp(&b.name));

		Table::new(
			["Partition", "Mount", "Used", "Free", "R/s", "W/s"].iter(),
			partitions.into_iter().map(|partition| {
				Row::StyledData(
					vec![
						partition.name,
						format!("{}", partition.mountpoint.display()),
						format!("{:3.0}%", partition.used_percent),
						format!("{}", Size::Bytes(partition.bytes_free)),
						format!("{}", Size::Bytes(partition.bytes_read_recently)),
						format!("{}", Size::Bytes(partition.bytes_written_recently)),
					]
					.into_iter(),
					self.colorscheme.text,
				)
			}),
		)
		.block(block::new(self.colorscheme, &self.title))
		.header_style(self.colorscheme.text.modifier(Modifier::BOLD))
		.widths(&if area.width > 55 {
			vec![
				// Constraint::Min(5),
				// Constraint::Min(5),
				Constraint::Length(u16::max((area.width as i16 - 2 - 50) as u16, 5)),
				Constraint::Length(u16::max((area.width as i16 - 2 - 50) as u16, 5)),
				Constraint::Length(5),
				Constraint::Length(8),
				Constraint::Length(8),
				Constraint::Length(8),
			]
		} else {
			vec![Constraint::Length(5), Constraint::Length(5)]
		})
		.column_spacing(1)
		.header_gap(0)
		.draw(area, buf);
	}
}
