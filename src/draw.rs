use std::io;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::Widget as _;
use tui::{Frame, Terminal};

use crate::{App, Widgets};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
	terminal.draw(|mut frame| {
		if let Some(statusbar) = app.statusbar.as_mut() {
			let chunks = Layout::default()
				.constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
				.split(frame.size());
			draw_widgets(&mut frame, &mut app.widgets, chunks[0]);
			statusbar.render(&mut frame, chunks[1]);
		} else {
			let chunks = Layout::default()
				.constraints(vec![Constraint::Percentage(100)])
				.split(frame.size());
			draw_widgets(&mut frame, &mut app.widgets, chunks[0]);
		}
	})
}

pub fn draw_widgets<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {
	if widgets.temp.is_some() {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[
					Constraint::Ratio(1, 3),
					Constraint::Ratio(1, 3),
					Constraint::Ratio(1, 3),
				]
				.as_ref(),
			)
			.split(area);
		widgets.cpu.render(frame, chunks[0]);
		draw_middle_row(frame, widgets, chunks[1]);
		draw_bottom_row(frame, widgets, chunks[2]);
	} else {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
			.split(area);
		widgets.cpu.render(frame, chunks[0]);
		draw_bottom_row(frame, widgets, chunks[1]);
	}
}

pub fn draw_middle_row<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {
	let horizontal_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
		.split(area);
	widgets.mem.render(frame, horizontal_chunks[1]);
	let vertical_chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
		.split(horizontal_chunks[0]);
	widgets
		.disk
		.as_mut()
		.unwrap()
		.render(frame, vertical_chunks[0]);
	widgets
		.temp
		.as_mut()
		.unwrap()
		.render(frame, vertical_chunks[1]);
}

pub fn draw_bottom_row<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {
	let horizontal_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
		.split(area);
	if let Some(net) = widgets.net.as_mut() {
		net.render(frame, horizontal_chunks[0]);
	} else {
		widgets.mem.render(frame, horizontal_chunks[0]);
	}
	widgets.proc.render(frame, horizontal_chunks[1]);
}

pub fn draw_help_menu<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
	terminal.draw(|mut frame| {
		let rect = app.help_menu.get_rect(&frame.size());
		app.help_menu.render(&mut frame, rect);
	})
}
