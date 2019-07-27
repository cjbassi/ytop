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

use futures::future::{join_all, FutureExt};
use num_rational::Ratio;

use crate::args::Args;
use crate::colorscheme::Colorscheme;

use self::battery::BatteryWidget;
use self::cpu::CpuWidget;
use self::disk::DiskWidget;
use self::help_menu::HelpMenu;
use self::mem::MemWidget;
use self::net::NetWidget;
use self::proc::ProcWidget;
use self::statusbar::Statusbar;
use self::temp::TempWidget;

pub struct Widgets {
	pub battery_widget: Option<BatteryWidget>,
	pub cpu_widget: CpuWidget,
	pub disk_widget: Option<DiskWidget>,
	pub help_menu: HelpMenu,
	pub mem_widget: MemWidget,
	pub net_widget: Option<NetWidget>,
	pub proc_widget: ProcWidget,
	pub statusbar: Option<Statusbar>,
	pub temp_widget: Option<TempWidget>,
}

pub fn setup_widgets(args: &Args, update_ratio: Ratio<u64>, colorscheme: &Colorscheme) -> Widgets {
	let cpu_widget = CpuWidget::new(update_ratio, args.average_cpu, args.per_cpu);
	let mem_widget = MemWidget::new(update_ratio);
	let proc_widget = ProcWidget::new();
	let help_menu = HelpMenu::new();

	let (battery_widget, disk_widget, net_widget, temp_widget) = if args.minimal {
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

pub async fn update_widgets(widgets: &mut Widgets, seconds: Ratio<u64>) {
	let zero = Ratio::from_integer(0);

	let mut futures = vec![
		widgets.cpu_widget.update().boxed(),
		widgets.mem_widget.update().boxed(),
	];

	if seconds % widgets.proc_widget.update_interval == zero {
		futures.push(widgets.proc_widget.update().boxed());
	}

	if let (Some(disk_widget), Some(net_widget), Some(temp_widget)) = (
		widgets.disk_widget.as_mut(),
		widgets.net_widget.as_mut(),
		widgets.temp_widget.as_mut(),
	) {
		if seconds % disk_widget.update_interval == zero {
			futures.push(disk_widget.update().boxed());
		}
		if seconds % net_widget.update_interval == zero {
			futures.push(net_widget.update().boxed());
		}
		if seconds % temp_widget.update_interval == zero {
			futures.push(temp_widget.update().boxed());
		}

		if let Some(battery_widget) = widgets.battery_widget.as_mut() {
			if seconds % battery_widget.update_interval == zero {
				futures.push(battery_widget.update().boxed());
			}
		}
	}

	join_all(futures).await;
}
