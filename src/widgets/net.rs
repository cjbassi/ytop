use num_rational::Ratio;
use psutil::network;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{RenderDirection, Sparkline, Widget};

use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct NetWidget {
	title: String,
	update_interval: Ratio<u64>,

	interfaces: String,

	bytes_recv: Vec<u64>,
	bytes_sent: Vec<u64>,

	total_bytes_recv: u64,
	total_bytes_sent: u64,

	collector: network::NetIoCountersCollector,
}

impl NetWidget {
	pub fn new(interfaces: String) -> NetWidget {
		NetWidget {
			title: " Network Usage ".to_string(),
			update_interval: Ratio::from_integer(1),

			interfaces,

			bytes_recv: Vec::new(),
			bytes_sent: Vec::new(),

			total_bytes_recv: 0,
			total_bytes_sent: 0,

			collector: network::NetIoCountersCollector::default(),
		}
	}
}

impl UpdatableWidget for NetWidget {
	fn update(&mut self) {
		let io_counters = self.collector.net_io_counters().unwrap();

		if self.total_bytes_recv == 0 {
			self.bytes_recv.push(0);
			self.bytes_sent.push(0);
		} else {
			self.bytes_recv
				.push(io_counters.bytes_recv() - self.total_bytes_recv);
			self.bytes_sent
				.push(io_counters.bytes_sent() - self.total_bytes_sent);
		}

		self.total_bytes_recv = io_counters.bytes_recv();
		self.total_bytes_sent = io_counters.bytes_sent();
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for NetWidget {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new().title(&self.title).draw(area, buf);

		let inner = Rect {
			x: area.x + 1,
			y: area.y + 1,
			width: area.width - 2,
			height: area.height - 2,
		};

		let top_half = Rect {
			x: inner.x,
			y: inner.y,
			width: inner.width,
			height: (inner.height / 2),
		};

		let top_sparkline = Rect {
			x: inner.x,
			y: inner.y + 3,
			width: inner.width,
			height: (inner.height / 2) - 3,
		};

		let bottom_half = Rect {
			x: inner.x,
			y: inner.y + (inner.height / 2),
			width: inner.width,
			height: (inner.height / 2),
		};

		let bottom_sparkline = Rect {
			x: inner.x,
			y: inner.y + (inner.height / 2) + 4,
			width: inner.width,
			height: (inner.height / 2) - 3,
		};

		buf.set_string(
			top_half.x + 1,
			top_half.y + 1,
			format!("Total Rx: {:>3.1} {:>2}", 0.0, ""),
			Style::default().fg(Color::Yellow),
		);

		buf.set_string(
			top_half.x + 1,
			top_half.y + 2,
			format!("Rx/s:     {:>3.1} {:>2}/s", 0.0, ""),
			Style::default().fg(Color::Yellow),
		);

		Sparkline::default()
			.data(
				&self
					.bytes_recv
					.iter()
					.cloned()
					.rev()
					.collect::<Vec<u64>>()
					.as_slice(),
			)
			.direction(RenderDirection::RTL)
			.max(*self.bytes_recv.iter().max().unwrap())
			.style(Style::default().fg(Color::Red))
			.draw(top_sparkline, buf);

		buf.set_string(
			bottom_half.x + 1,
			bottom_half.y + 1,
			format!("Total Tx: {:>3.1} {:>2}", 0.0, ""),
			Style::default().fg(Color::Yellow),
		);

		buf.set_string(
			bottom_half.x + 1,
			bottom_half.y + 2,
			format!("Tx/s:     {:>3.1} {:>2}/s", 0.0, ""),
			Style::default().fg(Color::Yellow),
		);

		Sparkline::default()
			.data(
				&self
					.bytes_sent
					.iter()
					.cloned()
					.rev()
					.collect::<Vec<u64>>()
					.as_slice(),
			)
			.direction(RenderDirection::RTL)
			.max(*self.bytes_sent.iter().max().unwrap())
			.style(Style::default().fg(Color::Red))
			.draw(bottom_sparkline, buf);
	}
}
