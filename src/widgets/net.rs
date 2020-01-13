use num_rational::Ratio;
use psutil::network;
use size::Size;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Modifier;
use tui::widgets::{RenderDirection, Sparkline, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

pub struct NetWidget<'a, 'b> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	interfaces: &'b str,

	bytes_recv: Vec<u64>,
	bytes_sent: Vec<u64>,

	total_bytes_recv: u64,
	total_bytes_sent: u64,

	collector: network::NetIoCountersCollector,
}

impl NetWidget<'_, '_> {
	pub fn new<'a, 'b>(colorscheme: &'a Colorscheme, interfaces: &'b str) -> NetWidget<'a, 'b> {
		NetWidget {
			title: " Network Usage ".to_string(),
			update_interval: Ratio::from_integer(1),
			colorscheme,

			interfaces,

			bytes_recv: Vec::new(),
			bytes_sent: Vec::new(),

			total_bytes_recv: 0,
			total_bytes_sent: 0,

			collector: network::NetIoCountersCollector::default(),
		}
	}
}

impl UpdatableWidget for NetWidget<'_, '_> {
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

impl Widget for NetWidget<'_, '_> {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		block::new(self.colorscheme, &self.title).draw(area, buf);

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
			height: i16::max((inner.height as i16 / 2) - 3, 0) as u16,
		};

		let bottom_half = Rect {
			x: inner.x,
			y: inner.y + (inner.height / 2),
			width: inner.width,
			height: (inner.height / 2),
		};

		let bottom_sparkline = Rect {
			x: inner.x,
			y: inner.y + (inner.height / 2) + 3,
			width: inner.width,
			height: i16::max((inner.height as i16 / 2) - 3, 0) as u16,
		};

		buf.set_string(
			top_half.x + 1,
			top_half.y + 1,
			format!("Total Rx: {}", Size::Bytes(self.total_bytes_recv)),
			self.colorscheme.text.modifier(Modifier::BOLD),
		);

		buf.set_string(
			top_half.x + 1,
			top_half.y + 2,
			format!(
				"Rx/s:     {}/s",
				Size::Bytes(self.bytes_recv.last().unwrap().to_owned())
			),
			self.colorscheme.text.modifier(Modifier::BOLD),
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
			.style(self.colorscheme.net_bars)
			.draw(top_sparkline, buf);

		buf.set_string(
			bottom_half.x + 1,
			bottom_half.y + 1,
			format!("Total Tx: {}", Size::Bytes(self.total_bytes_sent)),
			self.colorscheme.text.modifier(Modifier::BOLD),
		);

		buf.set_string(
			bottom_half.x + 1,
			bottom_half.y + 2,
			format!(
				"Tx/s:     {}/s",
				Size::Bytes(self.bytes_sent.last().unwrap().to_owned())
			),
			self.colorscheme.text.modifier(Modifier::BOLD),
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
			.style(self.colorscheme.net_bars)
			.draw(bottom_sparkline, buf);
	}
}
