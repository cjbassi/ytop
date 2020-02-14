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

use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
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

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

fn setup_terminal() {
	let mut stdout = io::stdout();

	execute!(stdout, terminal::EnterAlternateScreen).unwrap();
	execute!(stdout, cursor::Hide).unwrap();

	// Needed for when ytop is run in a TTY since TTYs don't actually have an alternate screen.
	// Must be executed after attempting to enter the alternate screen so that it only clears the
	// 		primary screen if we are running in a TTY.
	// If not running in a TTY, then we just end up clearing the alternate screen which should have
	// 		no effect.
	execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
	let mut stdout = io::stdout();

	// Needed for when ytop is run in a TTY since TTYs don't actually have an alternate screen.
	// Must be executed before attempting to leave the alternate screen so that it only modifies the
	// 		primary screen if we are running in a TTY.
	// If not running in a TTY, then we just end up modifying the alternate screen which should have
	// 		no effect.
	execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
	execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
	execute!(stdout, cursor::Show).unwrap();

	terminal::disable_raw_mode().unwrap();
}

fn setup_ui_events() -> Receiver<Event> {
	let (sender, receiver) = unbounded();
	thread::spawn(move || loop {
		sender.send(crossterm::event::read().unwrap()).unwrap();
	});

	receiver
}

fn setup_ctrl_c() -> Receiver<()> {
	let (sender, receiver) = unbounded();
	ctrlc::set_handler(move || {
		sender.send(()).unwrap();
	})
	.unwrap();

	receiver
}

// The log file currently isn't being used for anything right now, but it does help when debugging
// and we'll probably use it when we clean up the error handling.
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

// We need to catch panics since we need to close the terminal before logging any error messages to
// the screen.
fn setup_panic_hook() {
	panic::set_hook(Box::new(|panic_info| {
		cleanup_terminal();
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

	let app_dirs = AppDirs::new(Some(PROGRAM_NAME), AppUI::CommandLine).unwrap();
	let logfile_path = app_dirs.state_dir.join("errors.log");

	let colorscheme = read_colorscheme(&app_dirs.config_dir, &args.colorscheme).unwrap();
	let mut app = setup_app(&args, update_ratio, &colorscheme, PROGRAM_NAME);
	setup_logfile(&logfile_path);

	let backend = CrosstermBackend::new(io::stdout());
	let mut terminal = Terminal::new(backend).unwrap();

	setup_panic_hook();
	setup_terminal();

	let mut update_seconds = Ratio::from_integer(0);
	let ticker = setup_ticker(args.rate);
	let ui_events_receiver = setup_ui_events();
	let ctrl_c_events = setup_ctrl_c();

	update_widgets(&mut app.widgets, update_seconds);
	draw(&mut terminal, &mut app);

	let mut show_help_menu = false;
	let mut paused = false;

	// Used to keep track of the previous key for actions that required 2 keypresses.
	let mut previous_key_event: Option<KeyEvent> = None;
	// If `skip_key` is set to true, we set the previous key to None instead of recording it.
	let mut skip_key: bool;

	// Used to keep track of whether we need to redraw the process or CPU/Mem widgets after they
	// have been updated.
	let mut proc_modified: bool;
	let mut graphs_modified: bool;

	loop {
		select! {
			recv(ctrl_c_events) -> _ => {
				break;
			}
			recv(ticker) -> _ => {
				if !paused {
					update_seconds = (update_seconds + update_ratio) % Ratio::from_integer(60);
					update_widgets(&mut app.widgets, update_seconds);
					if !show_help_menu {
						draw(&mut terminal, &mut app);
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
									break
								},
								KeyCode::Char('?') => {
									show_help_menu = !show_help_menu;
									if show_help_menu {
										draw_help_menu(&mut terminal, &mut app);
									} else {
										draw(&mut terminal, &mut app);
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
										draw(&mut terminal, &mut app);
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
							draw_help_menu(&mut terminal, &mut app);
						} else {
							draw(&mut terminal, &mut app);
						}
					}
				}

				if !show_help_menu {
					if proc_modified {
						draw_proc(&mut terminal, &mut app);
					} else if graphs_modified {
						draw_graphs(&mut terminal, &mut app);
					}
				}
			}
		}
	}

	cleanup_terminal();
}
