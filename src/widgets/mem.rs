use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct MemWidget {
    title: String,
    update_interval: Duration,
}

impl MemWidget {
    pub fn new(update_interval: Duration) -> MemWidget {
        MemWidget {
            title: " Memory Usage ".to_string(),
            update_interval,
        }
    }
    pub async fn update(&mut self) {}
}

impl Widget for MemWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
