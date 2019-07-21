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

    show_average_cpu_load: bool,
    show_per_cpu_load: bool,

    average_cpu_data: Vec<i32>,
    per_cpu_data: Vec<Vec<i32>>,
}

impl CpuWidget {
    pub fn new(
        update_interval: Duration,
        show_average_cpu_load: bool,
        show_per_cpu_load: bool,
    ) -> CpuWidget {
        let mut cpu_widget = CpuWidget {
            title: " CPU Usage ".to_string(),
            update_interval,

            cpu_count: num_cpus::get(),

            show_average_cpu_load,
            show_per_cpu_load,

            average_cpu_data: Vec::new(),
            per_cpu_data: Vec::new(),
        };

        if !(show_average_cpu_load || show_per_cpu_load) {
            if cpu_widget.cpu_count <= 8 {
                cpu_widget.show_per_cpu_load = true
            } else {
                cpu_widget.show_average_cpu_load = true
            }
        }

        cpu_widget
    }

    pub async fn update(&mut self) {
        if self.show_average_cpu_load {}
        if self.show_per_cpu_load {}
        // let procs = sys.get_processor_list();
        // self.average_cpu_data.push((
        //     self.len_data as f64,
        //     procs[0].get_cpu_usage() as f64 * 100.0,
        // ));
        // for (i, proc) in procs.iter().skip(1).enumerate() {
        //     self.per_cpu_data[i].push((self.len_data as f64, proc.get_cpu_usage() as f64 * 100.0));
        // }
        // self.len_data += 1;
    }
}

impl Widget for CpuWidget {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        // let x_bounds = [self.len_data as f64 - 25.0, self.len_data as f64 - 1.0];
        // let mut datasets = vec![];
        // if self.average_cpu {
        //     datasets.push(
        //         Dataset::default()
        //             .name("AVRG")
        //             .marker(Marker::Braille)
        //             .style(Style::default().fg(Color::Yellow))
        //             .data(&self.average_cpu_data[..]),
        //     )
        // }
        // if self.per_cpu {
        //     for (i, cpu) in self.per_cpu_data.iter().enumerate() {
        //         datasets.push(
        //             Dataset::default()
        //                 .name(&self.cpu_names[i])
        //                 .marker(Marker::Braille)
        //                 .style(Style::default().fg(Color::Yellow))
        //                 .data(cpu),
        //         )
        //     }
        // }

        // let mut chart: Chart<String, String> = Chart::default();
        block::new()
            .title(&self.title)
            //     .x_axis(Axis::default().bounds(x_bounds))
            //     .y_axis(Axis::default().bounds([0.0, 100.0]))
            //     .datasets(&datasets)
            .draw(area, buf);
    }
}
