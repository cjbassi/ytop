use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct ProcWidget {
    title: String,
    update_interval: Duration,
}

impl ProcWidget {
    pub fn new() -> ProcWidget {
        ProcWidget {
            title: " Processes ".to_string(),
            update_interval: Duration::from_secs(1),
        }
    }

    pub async fn update(&mut self) {}
}

impl Widget for ProcWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
