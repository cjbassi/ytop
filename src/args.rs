use std::str::FromStr;

use structopt::StructOpt;

pub enum Colorscheme {
	Default,
	DefaultDark,
	Monokai,
	SolarizedDark,
	Vice,
	Custom(String),
}

impl FromStr for Colorscheme {
	type Err = std::io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"default" => Colorscheme::Default,
			"default-dark" => Colorscheme::DefaultDark,
			"monokai" => Colorscheme::Monokai,
			"solarized-dark" => Colorscheme::SolarizedDark,
			"vice" => Colorscheme::Vice,
			_ => Colorscheme::Custom(s.to_string()),
		})
	}
}

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
	pub colorscheme: Colorscheme,

	/// Show temperatures in fahrenheit.
	#[structopt(short = "f", long = "fahrenheit")]
	pub fahrenheit: bool,

	/// Comma separated list of network interfaces to show. Prepend an interface with '!' to hide it. 'all' shows all interfaces.
	#[structopt(short = "i", long = "interfaces", default_value = "!tun0")]
	pub interfaces: String,

	/// Only show the CPU, Mem, and Process widgets.
	#[structopt(short = "m", long = "minimal")]
	pub minimal: bool,

	/// Show each CPU in the CPU widget.
	#[structopt(short = "p", long = "per-cpu")]
	pub per_cpu: bool,

	/// Number of times per second to update the CPU and Mem widgets.
	#[structopt(short = "r", long = "rate", default_value = "1")]
	pub rate: u64,

	/// Show a statusbar with the time.
	#[structopt(short = "s", long = "statusbar")]
	pub statusbar: bool,
}
