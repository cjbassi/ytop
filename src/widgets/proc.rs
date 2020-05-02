use std::collections::HashMap;
use std::ops::Not;
use std::process::Command;

use num_rational::Ratio;
use psutil::cpu;
use psutil::memory;
use psutil::process;
#[cfg(target_os = "linux")]
use psutil::process::os::linux::Oneshot;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Rect};
use tui::style::Modifier;
use tui::widgets::{Row, Table, Widget};

use crate::colorscheme::Colorscheme;
use crate::update::UpdatableWidget;
use crate::widgets::block;

const UP_ARROW: &str = "▲";
const DOWN_ARROW: &str = "▼";

#[derive(PartialEq)]
enum SortMethod {
	Cpu,
	Mem,
	Num,
	Command,
}

impl Default for SortMethod {
	fn default() -> Self {
		SortMethod::Cpu
	}
}

#[derive(PartialEq, Clone, Copy)]
enum SortDirection {
	Up,
	Down,
}

impl Default for SortDirection {
	fn default() -> Self {
		SortDirection::Down
	}
}

impl Not for SortDirection {
	type Output = SortDirection;

	fn not(self) -> Self::Output {
		match self {
			SortDirection::Up => SortDirection::Down,
			SortDirection::Down => SortDirection::Up,
		}
	}
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
	sort_method: SortMethod,
	sort_direction: SortDirection,
	view_offset: usize,
	scrolled: bool,
	view_height: usize,

	cpu_count: u64,

	procs: Vec<Proc>,
	grouped_procs: HashMap<String, Proc>,

	process_collector: process::ProcessCollector,
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
			sort_method: SortMethod::default(),
			sort_direction: SortDirection::default(),
			view_offset: 0,
			scrolled: false,
			view_height: 0,

			cpu_count: cpu::cpu_count(),

			procs: Vec::new(),
			grouped_procs: HashMap::new(),

			process_collector: process::ProcessCollector::new().unwrap(),
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

	pub fn scroll_half_page_down(&mut self) {
		self.scroll_count(self.view_height as isize / 2);
	}

	pub fn scroll_half_page_up(&mut self) {
		self.scroll_count(-(self.view_height as isize / 2));
	}

	pub fn scroll_full_page_down(&mut self) {
		self.scroll_count(self.view_height as isize);
	}

	pub fn scroll_full_page_up(&mut self) {
		self.scroll_count(-(self.view_height as isize));
	}

	pub fn toggle_grouping(&mut self) {
		self.grouping = !self.grouping;
		self.selected_proc = None;
	}

	pub fn kill_process(&self) {
		let (command, arg) = match self.selected_proc.as_ref().unwrap() {
			SelectedProc::Pid(pid) => ("kill", pid.to_string()),
			SelectedProc::Name(name) => ("pkill", name.clone()),
		};
		Command::new(command).arg(arg).spawn().unwrap();
	}

	fn sort(&mut self, sort_method: SortMethod) {
		if self.sort_method == sort_method {
			self.sort_direction = !self.sort_direction;
		} else {
			self.sort_method = sort_method;
			self.sort_direction = SortDirection::default();
		}
	}

	pub fn sort_by_num(&mut self) {
		self.sort(SortMethod::Num);
	}

	pub fn sort_by_command(&mut self) {
		self.sort(SortMethod::Command);
	}

	pub fn sort_by_cpu(&mut self) {
		self.sort(SortMethod::Cpu);
	}

	pub fn sort_by_mem(&mut self) {
		self.sort(SortMethod::Mem);
	}
}

impl UpdatableWidget for ProcWidget<'_> {
	fn update(&mut self) {
		self.process_collector.update().unwrap();

		let cpu_count = self.cpu_count as f32;
		let virtual_memory = memory::virtual_memory().unwrap();

		self.procs = self
			.process_collector
			.processes
			.values_mut()
			.map(|process| {
				let num = process.pid();

				#[cfg(target_os = "linux")]
				let name = process.name_oneshot();
				#[cfg(target_os = "macos")]
				let name = process.name()?;

				#[cfg(target_os = "linux")]
				let commandline = process.cmdline()?.unwrap_or_else(|| format!("[{}]", name));
				#[cfg(target_os = "macos")]
				let commandline = String::default();

				#[cfg(target_os = "linux")]
				let cpu = process.cpu_percent_oneshot() / cpu_count;
				#[cfg(target_os = "macos")]
				let cpu = process.cpu_percent()? / cpu_count;

				let mem = process.memory_percent_oneshot(&virtual_memory)?;

				Ok(Proc {
					num,
					name,
					commandline,
					cpu,
					mem,
				})
			})
			.filter_map(|process: process::ProcessResult<Proc>| process.ok())
			.collect();

		self.grouped_procs.clear();
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
	}

	fn get_update_interval(&self) -> Ratio<u64> {
		self.update_interval
	}
}

impl Widget for ProcWidget<'_> {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		if area.height < 3 {
			return;
		}

		let inner = Rect {
			x: area.x + 1,
			y: area.y + 1,
			width: area.width - 2,
			height: area.height - 2,
		};

		self.view_height = inner.height as usize - 1;

		let mut procs = if self.grouping {
			self.grouped_procs.values().cloned().collect()
		} else {
			self.procs.clone()
		};
		if self.sort_direction == SortDirection::Up {
			match self.sort_method {
				SortMethod::Cpu => procs.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap()),
				SortMethod::Mem => procs.sort_by(|a, b| a.mem.partial_cmp(&b.mem).unwrap()),
				SortMethod::Num => procs.sort_by(|a, b| a.num.cmp(&b.num)),
				SortMethod::Command => procs.sort_by(|a, b| a.commandline.cmp(&b.commandline)),
			}
		} else {
			match self.sort_method {
				SortMethod::Cpu => procs.sort_by(|b, a| a.cpu.partial_cmp(&b.cpu).unwrap()),
				SortMethod::Mem => procs.sort_by(|b, a| a.mem.partial_cmp(&b.mem).unwrap()),
				SortMethod::Num => procs.sort_by(|b, a| a.num.cmp(&b.num)),
				SortMethod::Command => procs.sort_by(|b, a| a.commandline.cmp(&b.commandline)),
			}
		}

		let mut header = [
			if self.grouping { " Count" } else { " PID" },
			"Command",
			"CPU%",
			"Mem%",
		];
		let header_index = match &self.sort_method {
			SortMethod::Cpu => 2,
			SortMethod::Mem => 3,
			SortMethod::Num => 0,
			SortMethod::Command => 1,
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
			if self.selected_row > inner.height as usize + self.view_offset - 2 {
				self.view_offset = self.selected_row + 2 - inner.height as usize;
			} else if self.selected_row < self.view_offset {
				self.view_offset = self.selected_row;
			}
		}

		Table::new(
			header.iter(),
			procs.into_iter().skip(self.view_offset).map(|proc| {
				Row::StyledData(
					vec![
						format!(" {}", proc.num),
						if self.grouping {
							proc.name
						} else {
							proc.commandline
						},
						format!("{:>5.1}", proc.cpu),
						format!("{:>4.1}", proc.mem),
					]
					.into_iter(),
					self.colorscheme.text,
				)
			}),
		)
		.block(block::new(self.colorscheme, &self.title))
		.header_style(self.colorscheme.text.modifier(Modifier::BOLD))
		// TODO: this is only a temporary workaround until we fix the table column resizing
		// https://github.com/cjbassi/ytop/issues/23
		.widths(&[
			Constraint::Length(6),
			// Constraint::Min(5),
			Constraint::Length(u16::max((area.width as i16 - 2 - 16 - 3) as u16, 5)),
			Constraint::Length(5),
			Constraint::Length(5),
		])
		.column_spacing(1)
		.header_gap(0)
		.draw(area, buf);

		// Draw cursor.
		let cursor_y = inner.y + 1 + self.selected_row as u16 - self.view_offset as u16;
		if cursor_y < inner.bottom() {
			for i in inner.x..inner.right() {
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
