mod app;
mod args;
mod colorscheme;
mod draw;
mod update;
mod utils;
mod widgets;

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal;
use num_rational::Ratio;
use platform_dirs::{AppDirs, AppUI};
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use app::*;
use args::*;
use colorscheme::*;
use draw::*;
use update::*;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
	let mut stdout = io::stdout();
	execute!(stdout, terminal::EnterAlternateScreen)?;
	terminal::enable_raw_mode()?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	terminal.hide_cursor()?;
	// terminal.clear()?;

	Ok(terminal)
}

fn cleanup_terminal() -> Result<()> {
	let mut stdout = io::stdout();
	execute!(stdout, terminal::LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;

	Ok(())
}

fn setup_ui_events() -> Receiver<Event> {
	let (sender, receiver) = unbounded();
	thread::spawn(move || loop {
		sender.send(event::read().unwrap()).unwrap();
	});

	receiver
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
				cleanup_terminal().unwrap();
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
					Event::Key(key_event) => {
						if key_event.modifiers.is_empty() {
							match key_event.code {
								KeyCode::Char(c) => {
									match c {
										'q' => {
											cleanup_terminal().unwrap();
											break
										},
										'?' => {
											show_help_menu = !show_help_menu;
											if show_help_menu {
												draw_help_menu(&mut terminal, &mut app).unwrap();
											} else {
												draw(&mut terminal, &mut app).unwrap();
											}
										},
										_ => {}
									}
								},
								KeyCode::Esc => {
									if show_help_menu {
										show_help_menu = false;
										draw(&mut terminal, &mut app).unwrap();
									}
								}
								_ => {}
							}
						} else if key_event.modifiers == KeyModifiers::CONTROL {
							match key_event.code {
								KeyCode::Char(c) => {
									match c {
										'c' => {
											cleanup_terminal().unwrap();
											break
										},
										_ => {}
									}
								},
								_ => {}
							}
						}
					}
					Event::Mouse(mouse_event) => match mouse_event {
						_ => {
						}
					}
					Event::Resize(width, height) => {
					}
				}
			}
		}
	}
}
