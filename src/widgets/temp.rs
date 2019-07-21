use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct TempWidget {
    title: String,
    update_interval: Duration,
}

impl TempWidget {
    pub fn new() -> TempWidget {
        TempWidget {
            title: " Temperatures ".to_string(),
            update_interval: Duration::from_secs(5),
        }
    }
    pub async fn update(&mut self) {}
}

impl Widget for TempWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
