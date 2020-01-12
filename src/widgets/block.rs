use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};

use crate::colorscheme::Colorscheme;

pub fn new<'a>(colorscheme: &Colorscheme, title: &'a str) -> Block<'a> {
	Block::default()
		.borders(Borders::ALL)
		.border_style(Style::default().fg(Color::Cyan))
		.title(title)
}
