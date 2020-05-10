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
	fn update(&mut self) {
		let mut io_counters_perdisk = if cfg!(target_os = "linux") {
			self.collector.disk_io_counters_per_partition().unwrap()
		} else {
			Default::default() // not implemented yet on macOS
		};
		// `.rev()` selects the correct mountpoint when the partition is mounted multiple times
		// https://github.com/cjbassi/ytop/issues/25
		self.partitions = disk::partitions_physical()
			.unwrap()
			.into_iter()
			.rev()
			.map(|partition| {
				let name = PathBuf::from(partition.device())
					.file_name()
					.unwrap()
					.to_string_lossy()
					.to_string();
				let mountpoint = partition.mountpoint().to_path_buf();

				// We use `unwrap_or_default` since the function may return an error if there is
				// insufficient permissions to read the disk usage of the partition.
				// https://github.com/cjbassi/ytop/issues/48
				let disk_usage = disk::disk_usage(&mountpoint).unwrap_or_default();
				// TODO: we use an `unwrap_or_default` since rust-psutil doesn't provide a way to
				// match up virtual partitions returned from `partitions_physical` with their
				// corresponding io counters in `disk_io_counters_per_partition()`since the disk
				// is named differently in each function
				let io_counters = io_counters_perdisk
					.remove(&partition.device().replace("/dev/", ""))
					.unwrap_or_default();

				let bytes_read = io_counters.read_count();
				let bytes_written = io_counters.read_count();
				// Here we use an `unwrap_or_default` for when we are adding a new disk partition
				// that is loaded when on ytop is already running.
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

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

// TODO: this is only a temporary workaround until we fix the table column resizing
// https://github.com/cjbassi/ytop/issues/23
#[allow(clippy::all)]
fn custom_column_sizing(width: u16) -> Vec<Constraint> {
	let width = width - 2;
	if width >= 39 + 5 {
		vec![
			Constraint::Length((width - 34) / 2),
			Constraint::Length((width - 34) / 2),
			// Constraint::Min(5),
			// Constraint::Min(5),
			Constraint::Length(5),
			Constraint::Length(8),
			Constraint::Length(8),
			Constraint::Length(8),
		]
	} else if width >= 31 + 4 {
		vec![
			Constraint::Length((width - 25) / 2),
			Constraint::Length((width - 25) / 2),
			Constraint::Length(5),
			Constraint::Length(8),
			Constraint::Length(8),
		]
	} else if width >= 23 + 3 {
		vec![
			Constraint::Length((width - 16) / 2),
			Constraint::Length((width - 16) / 2),
			Constraint::Length(5),
			Constraint::Length(8),
		]
	} else if width >= 15 + 2 {
		vec![
			Constraint::Length((width - 7) / 2),
			Constraint::Length((width - 7) / 2),
			Constraint::Length(5),
		]
	} else if width >= 10 + 1 {
		vec![Constraint::Min(5), Constraint::Min(5)]
	} else {
		vec![]
	}
}

impl Widget for &DiskWidget<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
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
		.widths(&custom_column_sizing(area.width))
		.column_spacing(1)
		.header_gap(0)
		.render(area, buf);
	}
}
