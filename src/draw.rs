use std::io;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::Widget as _;
use tui::Terminal;

use crate::Widgets;

pub fn draw_widgets<B: Backend>(
	terminal: &mut Terminal<B>,
	widgets: &mut Widgets,
) -> io::Result<()> {
	terminal.draw(|mut frame| {
		let vertical_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[
					Constraint::Ratio(1, 3),
					Constraint::Ratio(1, 3),
					Constraint::Ratio(1, 3),
				]
				.as_ref(),
			)
			.split(frame.size());
		widgets.cpu_widget.render(&mut frame, vertical_chunks[0]);
		let middle_horizontal_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
			.split(vertical_chunks[1]);
		widgets
			.mem_widget
			.render(&mut frame, middle_horizontal_chunks[1]);
		let middle_left_vertical_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
			.split(middle_horizontal_chunks[0]);
		widgets
			.disk_widget
			.as_mut()
			.unwrap()
			.render(&mut frame, middle_left_vertical_chunks[0]);
		widgets
			.temp_widget
			.as_mut()
			.unwrap()
			.render(&mut frame, middle_left_vertical_chunks[1]);
		let bottom_horizontal_chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
			.split(vertical_chunks[2]);
		widgets
			.net_widget
			.as_mut()
			.unwrap()
			.render(&mut frame, bottom_horizontal_chunks[0]);
		widgets
			.proc_widget
			.render(&mut frame, bottom_horizontal_chunks[1]);
	})
}

pub fn draw_help_menu<B: Backend>(
	terminal: &mut Terminal<B>,
	widgets: &mut Widgets,
) -> io::Result<()> {
	terminal.draw(|mut frame| {})
}
