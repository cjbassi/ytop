use chrono::prelude::*;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Widget;

pub struct Statusbar {
	hostname: String,
	program_name: String,
	program_name_len: u16,
}

impl Statusbar {
	pub fn new(program_name: &str) -> Statusbar {
		Statusbar {
			hostname: hostname::get_hostname().unwrap(),
			program_name: program_name.to_string(),
			program_name_len: program_name.len() as u16,
		}
	}
}

impl Widget for Statusbar {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		let time = Local::now().format("%H:%M:%S").to_string();
		buf.set_string(area.x + 1, area.y, &self.hostname, Style::default());
		buf.set_string(
			(area.x + area.width - time.len() as u16) / 2,
			area.y,
			time,
			Style::default(),
		);
		buf.set_string(
			area.x + area.width - self.program_name_len - 1,
			area.y,
			&self.program_name,
			Style::default(),
		);
	}
}
