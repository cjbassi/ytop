# ytop

![Minimum rustc version](https://img.shields.io/badge/rustc-1.39+-green.svg)
[![Matrix](https://img.shields.io/matrix/ytop:matrix.org)](https://matrix.to/#/#ytop:matrix.org)

<div align="center">

*Another* TUI based system monitor, this time in Rust!

<img src="./assets/demos/demo.gif" />
<img src="./assets/screenshots/minimal.png" width="96%" />

</div>

## Missing features

- macOS is missing temperatures, disks, and process commandline
- Process filtering isn't implemented
- Mouse usage isn't implemented
- `rate` and `interfaces` cli args aren't implemented

## Installation

Currently working on Linux and macOS with support planned for all major platforms.

### Prebuilt binaries

Run the following to run [this](https://github.com/japaric/trust/blob/gh-pages/install.sh) script to download the correct binary for your system from the releases tab into `~/.cargo/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/ytop
```

Specify `--to` to change the download location.

### Arch Linux

Install `ytop`, `ytop-bin`, or `ytop-git` from the AUR.

### Homebrew

```bash
brew tap cjbassi/ytop
brew install ytop
```

### From source

```bash
cargo install -f --git https://github.com/cjbassi/ytop
```

## Usage

### Keybinds

- Quit: `q` or `<C-c>`
- Pause: `<Space>`
- Process navigation:
	- `k` and `<Up>`: up
	- `j` and `<Down>`: down
	- `<C-u>`: half page up
	- `<C-d>`: half page down
	- `<C-b>`: full page up
	- `<C-f>`: full page down
	- `gg` and `<Home>`: jump to top
	- `G` and `<End>`: jump to bottom
- Process actions:
	- `<Tab>`: toggle process grouping
	- `dd`: kill selected process or process group
- Process sorting:
	- p: PID/Count
	- n: Command
	- c: CPU
	- m: Mem
- Process filtering:
	- `/`: start editing filter
	- (while editing):
		- `<Enter>`: accept filter
		- `<C-c>` and `<Escape>`: clear filter
- CPU and Mem graph scaling:
	- `h`: scale in
	- `l`: scale out
- `?`: toggles keybind help menu

### Mouse

- click to select process
- mouse wheel to scroll through processes

### Colorschemes

ytop ships with a few colorschemes which can be set with the `-c` flag followed by the name of one. You can find all the colorschemes in the [colorschemes folder](./colorschemes).

To make a custom colorscheme, copy one of the default ones to `~/.config/ytop/<new-name>.json` and load it with `ytop -c <new-name>`. Colorscheme PRs are welcome!

### CLI Options

```
USAGE:
    ytop [FLAGS] [OPTIONS]

FLAGS:
    -a, --average-cpu    Show average CPU in the CPU widget.
    -b, --battery        Show Battery widget (overridden by 'minimal' flag).
    -f, --fahrenheit     Show temperatures in fahrenheit.
    -h, --help           Prints help information
    -m, --minimal        Only show the CPU, Mem, and Process widgets.
    -p, --per-cpu        Show each CPU in the CPU widget.
    -s, --statusbar      Show a statusbar with the time.
    -V, --version        Prints version information

OPTIONS:
    -c, --colorscheme <colorscheme>    Set a colorscheme. [default: default]
    -i, --interfaces <interfaces>      Comma separated list of network interfaces to show. Prepend an interface with '!'
                                       to hide it. 'all' shows all interfaces. [default: !tun0]
    -r, --rate <rate>                  Number of times per second to update the CPU and Mem widgets. [default: 1]
```
