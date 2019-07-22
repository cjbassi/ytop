use std::time::Duration;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Axis, Chart, Dataset, Marker, Widget};

use crate::widgets::block;

pub struct CpuWidget {
    title: String,
    update_interval: Duration,

    cpu_count: usize,

    show_average_cpu: bool,
    show_per_cpu: bool,

    update_count: f64,

    average_cpu_data: (String, Vec<(f64, f64)>),
    per_cpu_data: Vec<(String, Vec<(f64, f64)>)>,
}

impl CpuWidget {
    pub fn new(update_interval: Duration, show_average_cpu: bool, show_per_cpu: bool) -> CpuWidget {
        let mut cpu_widget = CpuWidget {
            title: " CPU Usage ".to_string(),
            update_interval,

            cpu_count: num_cpus::get(),

            show_average_cpu,
            show_per_cpu,

            update_count: 0.0,

            average_cpu_data: ("AVRG".to_string(), Vec::new()),
            per_cpu_data: Vec::new(),
        };

        if !(show_average_cpu || show_per_cpu) {
            if cpu_widget.cpu_count <= 8 {
                cpu_widget.show_per_cpu = true
            } else {
                cpu_widget.show_average_cpu = true
            }
        }

        if cpu_widget.show_per_cpu {
            for i in 0..cpu_widget.cpu_count {
                cpu_widget
                    .per_cpu_data
                    .push((format!("CPU{}", i), Vec::new()));
            }
        }

        cpu_widget
    }

    pub async fn update(&mut self) {
        self.update_count += 1.0;
        if self.show_average_cpu {
            self.average_cpu_data.1.push((self.update_count, 5.0));
        }
        if self.show_per_cpu {
            for i in 0..self.cpu_count {
                self.per_cpu_data[i].1.push((self.update_count, 5.0));
            }
        }
    }
}

impl Widget for CpuWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let mut datasets = Vec::new();
        if self.show_average_cpu {
            datasets.push(
                Dataset::default()
                    .name(&self.average_cpu_data.0)
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .data(&self.average_cpu_data.1),
            )
        }
        if self.show_per_cpu {
            for per_cpu_data in self.per_cpu_data.iter() {
                datasets.push(
                    Dataset::default()
                        .name(&per_cpu_data.0)
                        .marker(Marker::Braille)
                        .style(Style::default().fg(Color::Yellow))
                        .data(&per_cpu_data.1),
                )
            }
        }

        Chart::<String, String>::default()
            .block(block::new().title(&self.title))
            .x_axis(Axis::default().bounds([self.update_count - 100.0, self.update_count + 1.0]))
            .y_axis(Axis::default().bounds([0.0, 100.0]))
            .datasets(&datasets)
            .draw(area, buf);
    }
}
