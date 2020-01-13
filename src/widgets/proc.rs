use num_rational::Ratio;
use psutil::process;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::Modifier;
use tui::widgets::{Row, Table, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

#[derive(Clone)]
struct Proc {
	pid: u32,
	name: String,
	commandline: String,
	cpu: f32,
	mem: f32,
}

pub struct ProcWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	procs: Vec<Proc>,
}

impl ProcWidget<'_> {
	pub fn new(colorscheme: &Colorscheme) -> ProcWidget {
		ProcWidget {
			title: " Processes ".to_string(),
			update_interval: Ratio::from_integer(1),
			colorscheme,

			procs: Vec::new(),
		}
	}
}

impl UpdatableWidget for ProcWidget<'_> {
	fn update(&mut self) {
		self.procs = process::processes()
			.unwrap()
			.into_iter()
			.map(|process| {
				let process = process.unwrap();
				let name = process.name().unwrap();
				Proc {
					pid: process.pid(),
					name: name.to_string(),
					commandline: process
						.cmdline()
						.unwrap()
						.unwrap_or_else(|| format!("[{}]", name)),
					cpu: 0.0,
					mem: 0.0,
				}
			})
			.collect();
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for ProcWidget<'_> {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		let mut procs = self.procs.clone();
		procs.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap());

		Table::new(
			["PID", "Command", "CPU%", "Mem%"].iter(),
			procs.iter().map(|proc| {
				Row::StyledData(
					vec![
						proc.pid.to_string(),
						proc.commandline.to_string(),
						format!("{:2.1}", proc.cpu),
						format!("{:2.1}", proc.mem),
					]
					.into_iter(),
					self.colorscheme.text,
				)
			}),
		)
		.block(block::new(self.colorscheme, &self.title))
		.header_style(self.colorscheme.text.modifier(Modifier::BOLD))
		.widths(&[
			Constraint::Length(5),
			Constraint::Min(5),
			Constraint::Length(5),
			Constraint::Length(5),
		])
		.column_spacing(1)
		.header_gap(0)
		.draw(area, buf);
	}
}
