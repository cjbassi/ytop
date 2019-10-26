mod args;
mod colorscheme;
mod draw;
mod utils;
mod widgets;

use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::{AlternateScreen, InputEvent, KeyEvent, MouseEvent};
use num_rational::Ratio;
use platform_dirs::{AppDirs, AppUI};
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use args::*;
use colorscheme::*;
use draw::*;
use widgets::*;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
	let screen = AlternateScreen::to_alternate(true)?;
	let backend = CrosstermBackend::with_alternate_screen(io::stdout(), screen)?;
	let mut terminal = Terminal::new(backend)?;
	terminal.hide_cursor()?;
	terminal.clear()?;
	Ok(terminal)
}

fn setup_ui_events() -> Receiver<InputEvent> {
	let (ui_events_sender, ui_events_receiver) = unbounded();
	thread::spawn(move || {
		let _screen = crossterm::RawScreen::into_raw_mode().unwrap();
		let input = crossterm::input();
		input.enable_mouse_mode().unwrap();
		let mut reader = input.read_sync();
		loop {
			ui_events_sender.send(reader.next().unwrap()).unwrap();
		}
	});
	ui_events_receiver
}

fn setup_ctrl_c() -> Result<Receiver<()>, ctrlc::Error> {
	let (sender, receiver) = unbounded();
	ctrlc::set_handler(move || {
		sender.send(()).unwrap();
	})?;

	Ok(receiver)
}

fn setup_logfile(logfile_path: &Path) {
	fs::create_dir_all(logfile_path.parent().unwrap()).unwrap();
	let logfile = fs::OpenOptions::new()
		.write(true)
		.create(true)
		.truncate(true)
		.open(logfile_path)
		.unwrap();
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
		.unwrap();
}

fn main() {
	let args = Args::from_args();
	let update_ratio = Ratio::new(1, args.rate);
	let mut show_help_menu = false;

	let program_name = env!("CARGO_PKG_NAME");
	let app_dirs = AppDirs::new(Some(program_name), AppUI::CommandLine).unwrap();
	let logfile_path = app_dirs.state_dir.join("errors.log");

	let colorscheme = read_colorscheme(&app_dirs.config_dir, &args.colorscheme).unwrap();
	let mut app = setup_app(&args, update_ratio, &colorscheme, program_name);

	setup_logfile(&logfile_path);
	let mut terminal = setup_terminal().unwrap();

	let mut update_seconds = Ratio::from_integer(0);
	let ticker = tick(Duration::from_nanos(
		Duration::from_secs(1).as_nanos() as u64 / args.rate,
	));
	let ui_events_receiver = setup_ui_events();
	let ctrl_c_events = setup_ctrl_c().unwrap();

	update_widgets(&mut app.widgets, update_seconds);
	draw(&mut terminal, &mut app).unwrap();

	loop {
		select! {
			recv(ctrl_c_events) -> _ => {
				break;
			}
			recv(ticker) -> _ => {
				update_seconds = (update_seconds + update_ratio) % Ratio::from_integer(60);
				update_widgets(&mut app.widgets, update_seconds);
				if !show_help_menu {
					draw(&mut terminal, &mut app).unwrap();
				}
			}
			recv(ui_events_receiver) -> message => {
				match message.unwrap() {
					InputEvent::Keyboard(key_event) => {
						match key_event {
							KeyEvent::Char(c) => match c {
								'q' => break,
								'?' => {
									show_help_menu = !show_help_menu;
									if show_help_menu {
										draw_help_menu(&mut terminal, &mut app).unwrap();
									} else {
										draw(&mut terminal, &mut app).unwrap();
									}
								},
								_ => {}
							},
							KeyEvent::Ctrl(c) => match c {
								'c' => break,
								_ => {},
							},
							KeyEvent::Esc => {
								if show_help_menu {
									show_help_menu = false;
									draw(&mut terminal, &mut app).unwrap();
								}
							}
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
