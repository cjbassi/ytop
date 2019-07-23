use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::widgets::block;

pub struct TempWidget {
    title: String,
    update_interval: Duration,
    update_count: f64,

    temp_data: Vec<(String, Vec<(f64, f64)>)>,
}

impl TempWidget {
    pub fn new() -> TempWidget {
        TempWidget {
            title: " Temperatures ".to_string(),
            update_interval: Duration::from_secs(5),
            update_count: 0.0,

            temp_data: Vec::new(),
        }
    }

    pub async fn update(&mut self) {
        self.update_count += 1.0;
    }
}

impl Widget for TempWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        block::new().title(&self.title).draw(area, buf);
    }
}
