use std::fs;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

pub enum Colorschemes {
	Default,
	DefaultDark,
	Monokai,
	SolarizedDark,
	Vice,
	Custom(String),
}

impl FromStr for Colorschemes {
	type Err = std::io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"default" => Colorschemes::Default,
			"default-dark" => Colorschemes::DefaultDark,
			"monokai" => Colorschemes::Monokai,
			"solarized-dark" => Colorschemes::SolarizedDark,
			"vice" => Colorschemes::Vice,
			_ => Colorschemes::Custom(s.to_string()),
		})
	}
}

#[derive(Deserialize)]
pub struct Colorscheme {
	fg: i64,
	bg: i64,

	titles: i64,
	borders: i64,

	battery_lines: Vec<i64>,

	// need at least 8 entries
	cpu_lines: Vec<i64>,

	mem_main: i64,
	mem_swap: i64,

	net_bars: i64,

	proc_cursor: i64,

	temp_low: i64,
	temp_high: i64,
}

pub fn read_colorscheme(
	config_folder: &Path,
	colorscheme: &Colorschemes,
) -> serde_json::Result<Colorscheme> {
	match colorscheme {
		Colorschemes::Custom(name) => serde_json::from_str(
			&fs::read_to_string(config_folder.join(name).with_extension("json")).unwrap(),
		),
		_ => {
			let json_string = match colorscheme {
				Colorschemes::Default => include_str!("../colorschemes/default.json"),
				Colorschemes::DefaultDark => include_str!("../colorschemes/default-dark.json"),
				Colorschemes::SolarizedDark => include_str!("../colorschemes/solarized-dark.json"),
				Colorschemes::Monokai => include_str!("../colorschemes/monokai.json"),
				Colorschemes::Vice => include_str!("../colorschemes/vice.json"),
				_ => unreachable!(),
			};
			Ok(serde_json::from_str(json_string)
				.expect("statically defined and verified json colorschemes"))
		}
	}
}
