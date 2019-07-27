use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

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
	update_interval: Duration,

	partitions: HashMap<String, Partition>,
}

impl DiskWidget {
	pub fn new() -> DiskWidget {
		DiskWidget {
			title: " Disk Usage ".to_string(),
			update_interval: Duration::from_secs(1),

			partitions: HashMap::new(),
		}
	}

	pub async fn update(&mut self) {}
}

impl Widget for DiskWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);
	}
}
