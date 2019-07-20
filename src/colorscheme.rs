use serde::Deserialize;

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
