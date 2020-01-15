use std::collections::HashMap;
use std::process::Command;

use num_rational::Ratio;
use psutil::cpu;
use psutil::process;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::Modifier;
use tui::widgets::{Row, Table, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

const UP_ARROW: &str = "▲";
const DOWN_ARROW: &str = "▼";

enum ProcSorting {
	Cpu,
	Mem,
	Num,
	Command,
}

enum SortDirection {
	Up,
	Down,
}

enum SelectedProc {
	Pid(u32),
	Name(String),
}

#[derive(Clone)]
struct Proc {
	num: u32,
	name: String,
	commandline: String,
	cpu: f32,
	mem: f32,
}

pub struct ProcWidget<'a> {
	title: String,
	update_interval: Ratio<u64>,
	colorscheme: &'a Colorscheme,

	grouping: bool,
	selected_row: usize,
	selected_proc: Option<SelectedProc>,
	sorting: ProcSorting,
	sort_direction: SortDirection,
	view_offset: usize,
	scrolled: bool,

	cpu_count: u64,

	procs: Vec<Proc>,
	grouped_procs: HashMap<String, Proc>,
	processes: HashMap<u32, process::Process>,
}

impl ProcWidget<'_> {
	pub fn new(colorscheme: &Colorscheme) -> ProcWidget {
		ProcWidget {
			title: " Processes ".to_string(),
			update_interval: Ratio::from_integer(1),
			colorscheme,

			grouping: true,
			selected_row: 0,
			selected_proc: None,
			sorting: ProcSorting::Cpu,
			sort_direction: SortDirection::Down,
			view_offset: 0,
			scrolled: false,

			cpu_count: cpu::cpu_count(),

			procs: Vec::new(),
			grouped_procs: HashMap::new(),
			processes: HashMap::new(),
		}
	}

	fn scroll_count(&mut self, count: isize) {
		self.selected_row = isize::max(0, self.selected_row as isize + count) as usize;
		self.selected_proc = None;
		self.scrolled = true;
	}

	fn scroll_to(&mut self, count: usize) {
		self.selected_row = usize::min(
			count,
			if self.grouping {
				self.grouped_procs.len()
			} else {
				self.procs.len()
			} - 1,
		);
		self.selected_proc = None;
		self.scrolled = true;
	}

	pub fn scroll_up(&mut self) {
		self.scroll_count(-1);
	}

	pub fn scroll_down(&mut self) {
		self.scroll_count(1);
	}

	pub fn scroll_top(&mut self) {
		self.scroll_to(0);
	}

	pub fn scroll_bottom(&mut self) {
		self.scroll_to(if self.grouping {
			self.grouped_procs.len()
		} else {
			self.procs.len()
		});
	}

	pub fn toggle_grouping(&mut self) {
		self.grouping = !self.grouping;
		self.selected_proc = None;
	}

	fn kill_process(&self) {
		let (command, arg) = match self.selected_proc.as_ref().unwrap() {
			SelectedProc::Pid(pid) => ("kill", pid.to_string()),
			SelectedProc::Name(name) => ("pkill", name.clone()),
		};
		Command::new(command).arg(arg).spawn().unwrap();
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
		let cpu_count = self.cpu_count as f32;

		self.procs = self
			.processes
			.values_mut()
			.map(|process| {
				let result = {
					let name = process.name()?;
					Ok(Proc {
						num: process.pid(),
						name: name.to_string(),
						commandline: process.cmdline()?.unwrap_or_else(|| format!("[{}]", name)),
						cpu: process.cpu_percent()? / cpu_count,
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

		self.grouped_procs = HashMap::new();
		for proc in self.procs.iter() {
			self.grouped_procs
				.entry(proc.name.clone())
				.and_modify(|e| {
					e.num += 1;
					e.cpu += proc.cpu;
					e.mem += proc.mem;
				})
				.or_insert_with(|| Proc {
					num: 1,
					..proc.clone()
				});
		}

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
		let mut procs = if self.grouping {
			self.grouped_procs.values().cloned().collect()
		} else {
			self.procs.clone()
		};
		procs.sort_by(|a, b| match &self.sorting {
			ProcSorting::Cpu => a.cpu.partial_cmp(&b.cpu).unwrap(),
			ProcSorting::Mem => a.mem.partial_cmp(&b.mem).unwrap(),
			ProcSorting::Num => a.num.cmp(&b.num),
			ProcSorting::Command => a.commandline.cmp(&b.commandline),
		});
		if let SortDirection::Down = self.sort_direction {
			procs.reverse();
		}

		let mut header = [
			if self.grouping { "Count" } else { "PID" },
			"Command",
			"CPU%",
			"Mem%",
		];
		let header_index = match &self.sorting {
			ProcSorting::Cpu => 2,
			ProcSorting::Mem => 3,
			ProcSorting::Num => 0,
			ProcSorting::Command => 1,
		};
		let arrow = match &self.sort_direction {
			SortDirection::Up => UP_ARROW,
			SortDirection::Down => DOWN_ARROW,
		};
		let updated_header = format!("{}{}", header[header_index], arrow);
		header[header_index] = &updated_header;

		self.selected_row = match &self.selected_proc {
			Some(selected_proc) => {
				match selected_proc {
					SelectedProc::Pid(pid) => procs.iter().position(|proc| proc.num == *pid),
					SelectedProc::Name(name) => procs.iter().position(|proc| proc.name == *name),
				}
			}
			.unwrap_or(self.selected_row),
			None => self.selected_row,
		};
		self.scroll_to(self.selected_row);
		self.selected_proc = if self.grouping {
			Some(SelectedProc::Name(
				procs[self.selected_row].name.to_string(),
			))
		} else {
			Some(SelectedProc::Pid(procs[self.selected_row].num))
		};

		if self.scrolled {
			self.scrolled = false;
			if self.selected_row > area.height as usize + self.view_offset - 4 {
				self.view_offset = self.selected_row + 4 - area.height as usize;
			} else if self.selected_row < self.view_offset {
				self.view_offset = self.selected_row;
			}
		}

		Table::new(
			header.iter(),
			procs.into_iter().skip(self.view_offset).map(|proc| {
				Row::StyledData(
					vec![
						proc.num.to_string(),
						if self.grouping {
							proc.name
						} else {
							proc.commandline
						},
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
			Constraint::Length(6),
			// Constraint::Min(5),
			Constraint::Length(u16::max((area.width as i16 - 2 - 18) as u16, 5)),
			Constraint::Length(5),
			Constraint::Length(5),
		])
		.column_spacing(1)
		.header_gap(0)
		.draw(area, buf);

		let cursor_y = area.y + 2 + self.selected_row as u16 - self.view_offset as u16;
		if cursor_y < area.y + area.height - 1 {
			for i in (area.x + 1)..(area.x + area.width - 1) {
				let cell = buf.get_mut(i, cursor_y);
				if cell.symbol != " " {
					cell.set_modifier(Modifier::REVERSED);
					cell.set_fg(self.colorscheme.proc_cursor);
				} else {
					cell.set_bg(self.colorscheme.proc_cursor);
				}
			}
		}
	}
}
