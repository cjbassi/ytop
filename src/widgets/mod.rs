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

use futures::future::{join_all, FutureExt as _};
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

pub fn setup_app(args: &Args, update_ratio: Ratio<u64>, colorscheme: &Colorscheme) -> App {
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
		Some(Statusbar::new())
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

pub async fn update_widgets(widgets: &mut Widgets, seconds: Ratio<u64>) {
	let zero = Ratio::from_integer(0);

	let mut futures = vec![widgets.cpu.update().boxed(), widgets.mem.update().boxed()];

	if seconds % widgets.proc.update_interval == zero {
		futures.push(widgets.proc.update().boxed());
	}

	if let (Some(disk), Some(net), Some(temp)) = (
		widgets.disk.as_mut(),
		widgets.net.as_mut(),
		widgets.temp.as_mut(),
	) {
		if seconds % disk.update_interval == zero {
			futures.push(disk.update().boxed());
		}
		if seconds % net.update_interval == zero {
			futures.push(net.update().boxed());
		}
		if seconds % temp.update_interval == zero {
			futures.push(temp.update().boxed());
		}

		if let Some(battery) = widgets.battery.as_mut() {
			if seconds % battery.update_interval == zero {
				futures.push(battery.update().boxed());
			}
		}
	}

	join_all(futures).await;
}
