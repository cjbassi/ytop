mod app;
mod args;
mod colorscheme;
mod draw;
mod update;
mod widgets;

use std::fs;
use std::io::{self, Write};
use std::panic;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
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
	execute!(stdout, cursor::Hide)?;

	// for TTYs
	execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

	terminal::enable_raw_mode()?;

	let backend = CrosstermBackend::new(stdout);
	let terminal = Terminal::new(backend)?;

	Ok(terminal)
}

fn cleanup_terminal() -> Result<()> {
	let mut stdout = io::stdout();

	// for TTYs
	execute!(stdout, cursor::MoveTo(0, 0))?;
	execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

	execute!(stdout, terminal::LeaveAlternateScreen)?;
	execute!(stdout, cursor::Show)?;

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
		.level_for("mio", log::LevelFilter::Debug)
		.apply()
		.unwrap();
}

fn setup_panic_hook() {
	panic::set_hook(Box::new(|panic_info| {
		cleanup_terminal().unwrap();
		better_panic::Settings::auto().create_panic_handler()(panic_info);
	}));
}

fn setup_ticker(rate: u64) -> Receiver<Instant> {
	tick(Duration::from_nanos(
		Duration::from_secs(1).as_nanos() as u64 / rate,
	))
}

fn main() {
	better_panic::install();

	let args = Args::from_args();
	let update_ratio = Ratio::new(1, args.rate);

	let program_name = env!("CARGO_PKG_NAME");
	let app_dirs = AppDirs::new(Some(program_name), AppUI::CommandLine).unwrap();
	let logfile_path = app_dirs.state_dir.join("errors.log");

	let colorscheme = read_colorscheme(&app_dirs.config_dir, &args.colorscheme).unwrap();
	let mut app = setup_app(&args, update_ratio, &colorscheme, program_name);
	setup_logfile(&logfile_path);
	let mut terminal = setup_terminal().unwrap();

	setup_panic_hook();

	let mut update_seconds = Ratio::from_integer(0);
	let ticker = setup_ticker(args.rate);
	let ui_events_receiver = setup_ui_events();
	let ctrl_c_events = setup_ctrl_c().unwrap();

	update_widgets(&mut app.widgets, update_seconds);
	draw(&mut terminal, &mut app).unwrap();

	let mut show_help_menu = false;
	let mut paused = false;
	let mut previous_key_event: Option<KeyEvent> = None;
	let mut proc_modified: bool;
	let mut graphs_modified: bool;
	let mut skip_key: bool;

	loop {
		select! {
			recv(ctrl_c_events) -> _ => {
				cleanup_terminal().unwrap();
				break;
			}
			recv(ticker) -> _ => {
				if !paused {
					update_seconds = (update_seconds + update_ratio) % Ratio::from_integer(60);
					update_widgets(&mut app.widgets, update_seconds);
					if !show_help_menu {
						draw(&mut terminal, &mut app).unwrap();
					}
				}
			}
			recv(ui_events_receiver) -> message => {
				proc_modified = false;
				graphs_modified = false;
				skip_key = false;

				match message.unwrap() {
					Event::Key(key_event) => {
						if key_event.modifiers.is_empty() {
							match key_event.code {
								KeyCode::Char('q') => {
									cleanup_terminal().unwrap();
									break
								},
								KeyCode::Char('?') => {
									show_help_menu = !show_help_menu;
									if show_help_menu {
										draw_help_menu(&mut terminal, &mut app).unwrap();
									} else {
										draw(&mut terminal, &mut app).unwrap();
									}
								},
								KeyCode::Char(' ') => {
									paused = !paused;
								},
								KeyCode::Char('j') | KeyCode::Down => {
									app.widgets.proc.scroll_down();
									proc_modified = true;
								},
								KeyCode::Char('k') | KeyCode::Up => {
									app.widgets.proc.scroll_up();
									proc_modified = true;
								},
								KeyCode::Char('g') => {
									if previous_key_event == Some(KeyEvent::from(KeyCode::Char('g'))) {
										app.widgets.proc.scroll_top();
										proc_modified = true;
										skip_key = true;
									}
								},
								KeyCode::Home => {
									app.widgets.proc.scroll_top();
									proc_modified = true;
								},
								KeyCode::Char('G') | KeyCode::End => {
									app.widgets.proc.scroll_bottom();
									proc_modified = true;
								},
								KeyCode::Char('d') => {
									if previous_key_event == Some(KeyEvent::from(KeyCode::Char('d'))) {
										app.widgets.proc.kill_process();
										skip_key = true;
									}
								},
								KeyCode::Char('h') => {
									app.widgets.cpu.scale_in();
									app.widgets.mem.scale_in();
									graphs_modified = true;
								},
								KeyCode::Char('l') => {
									app.widgets.cpu.scale_out();
									app.widgets.mem.scale_out();
									graphs_modified = true;
								},
								KeyCode::Esc => {
									if show_help_menu {
										show_help_menu = false;
										draw(&mut terminal, &mut app).unwrap();
									}
								}
								KeyCode::Tab => {
									app.widgets.proc.toggle_grouping();
									proc_modified = true;
								},
								KeyCode::Char('p') => {
									app.widgets.proc.sort_by_num();
									proc_modified = true;
								},
								KeyCode::Char('n') => {
									app.widgets.proc.sort_by_command();
									proc_modified = true;
								},
								KeyCode::Char('c') => {
									app.widgets.proc.sort_by_cpu();
									proc_modified = true;
								},
								KeyCode::Char('m') => {
									app.widgets.proc.sort_by_mem();
									proc_modified = true;
								},
								_ => {}
							}
						} else if key_event.modifiers == KeyModifiers::CONTROL {
							match key_event.code {
								KeyCode::Char('c') => {
									cleanup_terminal().unwrap();
									break
								},
								KeyCode::Char('d') => {
									app.widgets.proc.scroll_half_page_down();
									proc_modified = true;
								},
								KeyCode::Char('u') => {
									app.widgets.proc.scroll_half_page_up();
									proc_modified = true;
								},
								KeyCode::Char('f') => {
									app.widgets.proc.scroll_full_page_down();
									proc_modified = true;
								},
								KeyCode::Char('b') => {
									app.widgets.proc.scroll_full_page_up();
									proc_modified = true;
								},
								_ => {}
							}
						}

						previous_key_event = if skip_key {
							None
						} else {
							Some(key_event)
						};
					}
					// TODO: figure out why these aren't working
					Event::Mouse(mouse_event) => match mouse_event {
						MouseEvent::ScrollUp(_, _, _) => {
							app.widgets.proc.scroll_up();
							proc_modified = true;
						},
						MouseEvent::ScrollDown(_, _, _) => {
							app.widgets.proc.scroll_down();
							proc_modified = true;
						},
						_ => {}
					}
					Event::Resize(_width, _height) => {
						if show_help_menu {
							draw_help_menu(&mut terminal, &mut app).unwrap();
						} else {
							draw(&mut terminal, &mut app).unwrap();
						}
					}
				}

				if !show_help_menu {
					if proc_modified {
						draw_proc(&mut terminal, &mut app).unwrap();
					} else if graphs_modified {
						draw_graphs(&mut terminal, &mut app).unwrap();
					}
				}
			}
		}
	}
}
