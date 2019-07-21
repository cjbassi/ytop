mod args;
mod colorscheme;
mod utils;
mod widgets;

use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::{AlternateScreen, InputEvent, KeyEvent, MouseEvent};
use log::info;
use platform_dirs::{AppDirs, AppUI};
use structopt::StructOpt;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::Widget;
use tui::Terminal;

use args::Args;

struct Widgets {
    battery_widget: Option<widgets::BatteryWidget>,
    cpu_widget: widgets::CpuWidget,
    disk_widget: Option<widgets::DiskWidget>,
    help_menu: widgets::HelpMenu,
    mem_widget: widgets::MemWidget,
    net_widget: Option<widgets::NetWidget>,
    proc_widget: widgets::ProcWidget,
    statusbar: Option<widgets::Statusbar>,
    temp_widget: Option<widgets::TempWidget>,
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend>> {
    let screen = AlternateScreen::to_alternate(true)?;
    let backend = CrosstermBackend::with_alternate_screen(screen)?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(terminal)
}

fn setup_ui_events() -> Receiver<InputEvent> {
    let (ui_events_sender, ui_events_receiver) = unbounded();
    thread::spawn(move || {
        let _screen = crossterm::RawScreen::into_raw_mode().unwrap(); // TODO: unwrap
        let input = crossterm::input();
        input.enable_mouse_mode().unwrap(); // TODO: unwrap
        let mut reader = input.read_sync();
        loop {
            ui_events_sender.send(reader.next().unwrap()).unwrap(); // TODO: unwraps
        }
    });
    ui_events_receiver
}

fn setup_logfile(logfile_path: &Path) {
    fs::create_dir_all(logfile_path.parent().unwrap()).unwrap(); // TODO: unwrap
    let logfile = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(logfile_path)
        .unwrap(); // TODO: unwrap
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}]: {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(logfile)
        .apply()
        .unwrap(); // TODO: unwrap
}

fn read_colorscheme(
    config_folder: &Path,
    colorscheme: &args::Colorscheme,
) -> serde_json::Result<colorscheme::Colorscheme> {
    match colorscheme {
        args::Colorscheme::Custom(name) => serde_json::from_str(
            &fs::read_to_string(config_folder.join(name).with_extension("json")).unwrap(), // TODO: unwrap
        ),
        _ => {
            let json_string = match colorscheme {
                args::Colorscheme::Default => include_str!("../colorschemes/default.json"),
                args::Colorscheme::DefaultDark => include_str!("../colorschemes/default-dark.json"),
                args::Colorscheme::SolarizedDark => {
                    include_str!("../colorschemes/solarized-dark.json")
                }
                args::Colorscheme::Monokai => include_str!("../colorschemes/monokai.json"),
                args::Colorscheme::Vice => include_str!("../colorschemes/vice.json"),
                _ => unreachable!(),
            };
            Ok(serde_json::from_str(json_string).unwrap())
        }
    }
}

fn setup_widgets(args: &Args, colorscheme: &colorscheme::Colorscheme) -> Widgets {
    let battery_widget = Some(widgets::BatteryWidget::new());
    let cpu_widget = widgets::CpuWidget::new(Duration::from_secs(1), true, true);
    let disk_widget = Some(widgets::DiskWidget::new());
    let help_menu = widgets::HelpMenu::new();
    let mem_widget = widgets::MemWidget::new(Duration::from_secs(1));
    let net_widget = Some(widgets::NetWidget::new());
    let proc_widget = widgets::ProcWidget::new();
    let statusbar = Some(widgets::Statusbar::new());
    let temp_widget = Some(widgets::TempWidget::new());

    Widgets {
        battery_widget,
        cpu_widget,
        disk_widget,
        help_menu,
        mem_widget,
        net_widget,
        proc_widget,
        statusbar,
        temp_widget,
    }
}

fn update_widgets(widgets: &mut Widgets, ticks: i64) {
    // if ticks % widgets.cpu_widget.update_interval == 0 {
    //     widgets.cpu_widget.update();
    // }
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, widgets: &mut Widgets) -> io::Result<()> {
    terminal.draw(|mut frame| {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ]
                .as_ref(),
            )
            .split(frame.size());
        widgets.cpu_widget.render(&mut frame, vertical_chunks[0]);
        let middle_horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
            .split(vertical_chunks[1]);
        widgets
            .mem_widget
            .render(&mut frame, middle_horizontal_chunks[1]);
        let middle_left_vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(middle_horizontal_chunks[0]);
        widgets
            .disk_widget
            .as_mut()
            .unwrap() // TODO: unwrap
            .render(&mut frame, middle_left_vertical_chunks[0]);
        widgets
            .temp_widget
            .as_mut()
            .unwrap() // TODO: unwrap
            .render(&mut frame, middle_left_vertical_chunks[1]);
        let bottom_horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .split(vertical_chunks[2]);
        widgets
            .net_widget
            .as_mut()
            .unwrap() // TODO: unwrap
            .render(&mut frame, bottom_horizontal_chunks[0]);
        widgets
            .proc_widget
            .render(&mut frame, bottom_horizontal_chunks[1]);
    })
}

fn main() {
    let args = Args::from_args();

    let program_name = env!("CARGO_PKG_NAME");
    let app_dirs = AppDirs::new(Some(program_name), AppUI::CommandLine).unwrap(); // TODO: unwrap
    let logfile_path = app_dirs.state_dir.join("errors.log");

    let colorscheme = read_colorscheme(&app_dirs.config_dir, &args.colorscheme).unwrap(); // TODO: unwrap
    let mut widgets = setup_widgets(&args, &colorscheme);

    setup_logfile(&logfile_path);
    let mut terminal = setup_terminal().unwrap(); // TODO: unwrap

    let ticker = tick(Duration::from_secs(1));
    let ui_events_receiver = setup_ui_events();

    let mut ticks = 0;
    update_widgets(&mut widgets, ticks);
    draw(&mut terminal, &mut widgets).unwrap(); // TODO: unwrap

    loop {
        select! {
            recv(ticker) -> _ => {
                ticks = (ticks + 1) % 60;
                update_widgets(&mut widgets, ticks);
                draw(&mut terminal, &mut widgets).unwrap(); // TODO: unwrap
            }
            recv(ui_events_receiver) -> message => {
                match message.unwrap() { // TODO: unwrap
                    InputEvent::Keyboard(key_event) => {
                        match key_event {
                            KeyEvent::Char(c) => match c {
                                'q' => break,
                                _ => {}
                            },
                            KeyEvent::Ctrl(c) => match c {
                                'c' => break,
                                _ => {},
                            },
                            _ => {}
                        }
                    }
                    InputEvent::Mouse(mouse_event) => match mouse_event {
                        _ => {}
                    }
                    _ => {}
                }
            }
        }
    }
}
