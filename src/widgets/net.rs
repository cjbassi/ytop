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

const VPN_INTERFACE: &str = "tun0";

pub struct NetWidget<'a, 'b> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	interface: &'b str,
	show_bits: bool,

	bytes_recv: Vec<u64>,
	bytes_sent: Vec<u64>,

	total_bytes_recv: u64,
	total_bytes_sent: u64,

	collector: network::NetIoCountersCollector,
}

impl NetWidget<'_, '_> {
	pub fn new<'a, 'b>(
		colorscheme: &'a Colorscheme,
		interface: &'b str,
		show_bits: bool,
	) -> NetWidget<'a, 'b> {
		NetWidget {
			title: if interface == "all" {
				" Network Usage ".to_string()
			} else {
				format!(" Network Usage: {} ", interface)
			},
			update_interval: Ratio::from_integer(1),
			colorscheme,

			interface,

			bytes_recv: Vec::new(),
			bytes_sent: Vec::new(),

			total_bytes_recv: 0,
			total_bytes_sent: 0,

			collector: network::NetIoCountersCollector::default(),
			show_bits,
		}
	}
}

impl UpdatableWidget for NetWidget<'_, '_> {
	fn update(&mut self) {
		let io_counters: network::NetIoCounters = self
			.collector
			.net_io_counters_pernic()
			.unwrap()
			.into_iter()
			.filter(|(name, _counters)| {
				// Filter out the VPN interface unless specified directly since it gets double
				// counted along with the hardware interfaces it is operating on.
				(self.interface == "all" && name != VPN_INTERFACE) || name == self.interface
			})
			.map(|(_name, counters)| counters)
			.sum();

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

		let bottom_half = Rect {
			x: inner.x,
			y: inner.y + (inner.height / 2),
			width: inner.width,
			height: (inner.height / 2),
		};

		let top_sparkline = Rect {
			x: top_half.x,
			y: top_half.y + 3,
			width: top_half.width,
			height: i16::max(top_half.height as i16 - 3, 0) as u16,
		};

		let bottom_sparkline = Rect {
			x: bottom_half.x,
			y: bottom_half.y + 3,
			width: bottom_half.width,
			height: i16::max(bottom_half.height as i16 - 3, 0) as u16,
		};

		if inner.height < 3 {
			return;
		}

		buf.set_string(
			top_half.x + 1,
			top_half.y + 1,
			format!("Total Rx: {}", Size::Bytes(self.total_bytes_recv)),
			self.colorscheme.text.modifier(Modifier::BOLD),
		);

		buf.set_string(
			top_half.x + 1,
			top_half.y + 2,
			if self.show_bits {
				format!(
					"Rx/s:     {} bits/s",
					self.bytes_recv.last().unwrap().to_owned() * 8
				)
			} else {
				format!(
					"Rx/s:     {}/s",
					Size::Bytes(self.bytes_recv.last().unwrap().to_owned())
				)
			},
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
			.show_baseline(true)
			.max(*self.bytes_recv.iter().max().unwrap())
			.style(self.colorscheme.net_bars)
			.draw(top_sparkline, buf);

		if inner.height < 5 {
			return;
		}

		buf.set_string(
			bottom_half.x + 1,
			bottom_half.y + 1,
			format!("Total Tx: {}", Size::Bytes(self.total_bytes_sent)),
			self.colorscheme.text.modifier(Modifier::BOLD),
		);

		buf.set_string(
			bottom_half.x + 1,
			bottom_half.y + 2,
			if self.show_bits {
				format!(
					"Tx/s:     {} bits/s",
					self.bytes_sent.last().unwrap().to_owned() * 8
				)
			} else {
				format!(
					"Rx/s:     {}/s",
					Size::Bytes(self.bytes_sent.last().unwrap().to_owned())
				)
			},
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
			.show_baseline(true)
			.max(*self.bytes_sent.iter().max().unwrap())
			.style(self.colorscheme.net_bars)
			.draw(bottom_sparkline, buf);
	}
}
