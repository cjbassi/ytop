use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};

pub fn new() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
}
