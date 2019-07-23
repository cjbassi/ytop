use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct MemWidget {
    title: String,
    update_interval: Duration,
    update_count: f64,
}

impl MemWidget {
    pub fn new(update_interval: Duration) -> MemWidget {
        MemWidget {
            title: " Memory Usage ".to_string(),
            update_interval,
            update_count: 0.0,
        }
    }

    pub async fn update(&mut self) {
        self.update_count += 1.0;
    }
}

impl Widget for MemWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
