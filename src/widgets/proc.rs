use std::collections::HashMap;

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

	selected_row: usize,

	procs: Vec<Proc>,
	processes: HashMap<u32, process::Process>,
}

impl ProcWidget<'_> {
	pub fn new(colorscheme: &Colorscheme) -> ProcWidget {
		ProcWidget {
			title: " Processes ".to_string(),
			update_interval: Ratio::from_integer(1),
			colorscheme,

			selected_row: 0,

			procs: Vec::new(),
			processes: HashMap::new(),
		}
	}
}

impl UpdatableWidget for ProcWidget<'_> {
	fn update(&mut self) {
		process::processes()
			.unwrap()
			.into_iter()
			.filter_map(|process| process.ok())
			.for_each(|process| {
				if !self.processes.contains_key(&process.pid())
					|| self.processes[&process.pid()] != process
				{
					self.processes.insert(process.pid(), process);
				}
			});

		let mut to_remove = Vec::new();

		self.procs = self
			.processes
			.values_mut()
			.map(|process| {
				let result = {
					let name = process.name()?;
					Ok(Proc {
						pid: process.pid(),
						name: name.to_string(),
						commandline: process.cmdline()?.unwrap_or_else(|| format!("[{}]", name)),
						cpu: process.cpu_percent()?,
						mem: process.memory_percent()?,
					})
				};
				if result.is_err() {
					to_remove.push(process.pid());
				}
				result
			})
			.filter_map(|process: process::ProcessResult<Proc>| process.ok())
			.collect();

		for id in to_remove {
			self.processes.remove(&id);
		}
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
			procs.into_iter().map(|proc| {
				Row::StyledData(
					vec![
						proc.pid.to_string(),
						proc.commandline,
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
			// Constraint::Min(5),
			Constraint::Length(u16::max((area.width as i16 - 2 - 18) as u16, 5)),
			Constraint::Length(5),
			Constraint::Length(5),
		])
		.column_spacing(1)
		.header_gap(0)
		.draw(area, buf);
	}
}
