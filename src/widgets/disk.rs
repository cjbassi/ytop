use std::collections::HashMap;
use std::path::PathBuf;

use num_rational::Ratio;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::{block, WidgetUpdate};

struct Partition {
	name: String,
	mountpoint: PathBuf,
	bytes_read: u64,
	bytes_written: u64,
	bytes_read_recently: u64,
	bytes_written_recently: u64,
	used_percent: f64,
	bytes_free: u64,
}

pub struct DiskWidget {
	title: String,
	pub update_interval: Ratio<u64>,

	partitions: HashMap<String, Partition>,
}

impl DiskWidget {
	pub fn new() -> DiskWidget {
		DiskWidget {
			title: " Disk Usage ".to_string(),
			update_interval: Ratio::from_integer(1),

			partitions: HashMap::new(),
		}
	}
}

impl WidgetUpdate for DiskWidget {
	fn update(&mut self) {}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for DiskWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
