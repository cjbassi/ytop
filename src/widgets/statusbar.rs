use chrono::prelude::*;
use psutil::host;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::colorscheme::Colorscheme;

pub struct Statusbar<'a> {
	hostname: String,
	program_name: String,
	program_name_len: u16,

	colorscheme: &'a Colorscheme,
}

impl Statusbar<'_> {
	pub fn new<'a>(colorscheme: &'a Colorscheme, program_name: &str) -> Statusbar<'a> {
		Statusbar {
			hostname: host::info().hostname().to_owned(),
			program_name: program_name.to_string(),
			program_name_len: program_name.len() as u16,

			colorscheme,
		}
	}
}

impl<'a> Widget for &mut Statusbar<'a> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let time = Local::now().format("%H:%M:%S").to_string();
		buf.set_string(area.x + 1, area.y, &self.hostname, self.colorscheme.text);
		buf.set_string(
			(area.x + area.width - time.len() as u16) / 2,
			area.y,
			time,
			self.colorscheme.text,
		);
		buf.set_string(
			area.x + area.width - self.program_name_len - 1,
			area.y,
			&self.program_name,
			self.colorscheme.text,
		);
	}
}
