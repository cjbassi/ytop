use num_rational::Ratio;

use crate::args::Args;
use crate::colorscheme::Colorscheme;
use crate::widgets::*;

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
