use num_rational::Ratio;
use structopt::StructOpt;

use crate::colorscheme::Colorschemes;

#[derive(StructOpt)]
pub struct Args {
	/// Show average CPU in the CPU widget.
	#[structopt(short = "a", long = "average-cpu")]
	pub average_cpu: bool,

	/// Show Battery widget (overridden by 'minimal' flag).
	#[structopt(short = "b", long = "battery")]
	pub battery: bool,

	/// Set a colorscheme.
	#[structopt(
		short = "c",
		long = "colorscheme",
		default_value = "default",
		long_help = r"Colorschemes:
    - default
    - default-dark (for white backgrounds)
    - solarized-dark
    - monokai
    - vice
"
	)]
	pub colorscheme: Colorschemes,

	/// Show temperatures in fahrenheit.
	#[structopt(short = "f", long = "fahrenheit")]
	pub fahrenheit: bool,

	/// Comma separated list of network interfaces to show. Prepend an interface with '!' to hide it. 'all' shows all interfaces.
	#[structopt(short = "i", long = "interfaces", default_value = "!tun0")]
	pub interfaces: String,

	/// Duration of interval in seconds between updates of the CPU and Mem widgets.
	#[structopt(long = "interval", default_value = "1")]
	pub interval: Ratio<u64>,

	/// Only show the CPU, Mem, and Process widgets.
	#[structopt(short = "m", long = "minimal")]
	pub minimal: bool,

	/// Show each CPU in the CPU widget.
	#[structopt(short = "p", long = "per-cpu")]
	pub per_cpu: bool,

	/// Show a statusbar with the time.
	#[structopt(short = "s", long = "statusbar")]
	pub statusbar: bool,
}
