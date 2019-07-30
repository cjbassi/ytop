mod battery;
mod block;
mod cpu;
mod disk;
mod help_menu;
mod mem;
mod net;
mod proc;
mod statusbar;
mod temp;

use crossbeam_utils::thread;
use num_rational::Ratio;

use crate::Args;
use crate::Colorscheme;

use self::battery::BatteryWidget;
use self::cpu::CpuWidget;
use self::disk::DiskWidget;
use self::help_menu::HelpMenu;
use self::mem::MemWidget;
use self::net::NetWidget;
use self::proc::ProcWidget;
use self::statusbar::Statusbar;
use self::temp::TempWidget;

trait WidgetUpdate {
	fn update(&mut self);
	fn get_update_interval(&self) -> Ratio<u64>;
}

pub struct App {
	pub help_menu: HelpMenu,
	pub statusbar: Option<Statusbar>,
	pub widgets: Widgets,
}

pub struct Widgets {
	pub battery: Option<BatteryWidget>,
	pub cpu: CpuWidget,
	pub disk: Option<DiskWidget>,
	pub mem: MemWidget,
	pub net: Option<NetWidget>,
	pub proc: ProcWidget,
	pub temp: Option<TempWidget>,
}

pub fn setup_app(
	args: &Args,
	update_ratio: Ratio<u64>,
	colorscheme: &Colorscheme,
	program_name: &str,
) -> App {
	let cpu = CpuWidget::new(update_ratio, args.average_cpu, args.per_cpu);
	let mem = MemWidget::new(update_ratio);
	let proc = ProcWidget::new();
	let help_menu = HelpMenu::new();

	let (battery, disk, net, temp) = if args.minimal {
		(None, None, None, None)
	} else {
		(
			if args.battery {
				Some(BatteryWidget::new())
			} else {
				None
			},
			Some(DiskWidget::new()),
			Some(NetWidget::new(args.interfaces.clone())),
			Some(TempWidget::new(args.fahrenheit)),
		)
	};

	let statusbar = if args.statusbar {
		Some(Statusbar::new(program_name))
	} else {
		None
	};

	App {
		help_menu,
		statusbar,
		widgets: Widgets {
			battery,
			cpu,
			disk,
			mem,
			net,
			proc,
			temp,
		},
	}
}

pub fn update_widgets(widgets: &mut Widgets, seconds: Ratio<u64>) {
	let zero = Ratio::from_integer(0);

	let mut widgets_update: Vec<&mut (dyn WidgetUpdate + Send)> =
		vec![&mut widgets.cpu, &mut widgets.mem, &mut widgets.proc];

	if let (Some(disk), Some(net), Some(temp)) = (
		widgets.disk.as_mut(),
		widgets.net.as_mut(),
		widgets.temp.as_mut(),
	) {
		widgets_update.push(disk);
		widgets_update.push(net);
		widgets_update.push(temp);
		if let Some(battery) = widgets.battery.as_mut() {
			widgets_update.push(battery);
		}
	}

	thread::scope(|scope| {
		for widget in widgets_update {
			if seconds % widget.get_update_interval() == zero {
				scope.spawn(move |_| {
					widget.update();
				});
			}
		}
	})
	.unwrap();
}
