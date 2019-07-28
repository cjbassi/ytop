use lazy_static::lazy_static;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Paragraph, Text, Widget};

use super::block;

const TEXT: &str = r"Quit: q or <C-c>
Process navigation:
  - k and <Up>: up
  - j and <Down>: down
  - <C-u>: half page up
  - <C-d>: half page down
  - <C-b>: full page up
  - <C-f>: full page down
  - gg and <Home>: jump to top
  - G and <End>: jump to bottom
Process actions:
  - <Tab>: toggle process grouping
  - dd: kill selected process or process group
Process sorting:
  - c: CPU
  - m: Mem
  - p: PID
Process filtering:
  - /: start editing filter
  - (while editing):
    - <Enter>: accept filter
    - <C-c> and <Escape>: clear filter
CPU and Mem graph scaling:
  - h: scale in
  - l: scale out";

const TEXT_WIDTH: u16 = 48;
const TEXT_HEIGHT: u16 = 27;

lazy_static! {
	static ref TEXT_VEC: Vec<Text<'static>> = TEXT
		.lines()
		.map(|line| Text::raw(format!("{}\n", line)))
		.collect();
}

pub struct HelpMenu {
	title: String,
}

impl HelpMenu {
	pub fn new() -> HelpMenu {
		HelpMenu {
			title: " Help Menu ".to_string(),
		}
	}

	pub fn get_rect(&self, area: &Rect) -> Rect {
		Rect {
			x: (area.width - TEXT_WIDTH) / 2,
			y: (area.height - TEXT_HEIGHT) / 2,
			width: TEXT_WIDTH,
			height: TEXT_HEIGHT,
		}
	}
}

impl Widget for HelpMenu {
	fn draw(&mut self, area: Rect, buf: &mut Buffer) {
		Paragraph::new(TEXT_VEC.iter())
			.block(block::new().title(&self.title))
			.draw(area, buf);
	}
}
