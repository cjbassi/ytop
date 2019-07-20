use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct BatteryWidget {
    title: String,
    update_interval: Duration,
}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        BatteryWidget {
            title: " Batteries ".to_string(),
            update_interval: Duration::from_secs(60),
        }
    }
}

impl Widget for BatteryWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
