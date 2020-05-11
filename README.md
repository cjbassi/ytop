# ytop

![Minimum rustc version](https://img.shields.io/badge/rustc-1.39+-green.svg)
[![Matrix](https://img.shields.io/badge/matrix-%23ytop-blue.svg)](https://matrix.to/#/#ytop:matrix.org)

<div align="center">

*Another* TUI based system monitor, this time in Rust!

<img src="./assets/demos/demo.gif" />
<img src="./assets/screenshots/minimal.png" width="96%" />

</div>

## Missing features

- macOS is missing disk io counters and process commandline
- Process filtering isn't implemented
- Mouse usage isn't implemented

## Installation

Currently works on Linux and macOS with support planned for all major platforms.

### Package managers

[![Packaging status](https://repology.org/badge/vertical-allrepos/ytop.svg)](https://repology.org/project/ytop/versions)

### Homebrew

```bash
brew tap cjbassi/ytop
brew install ytop
```

### Prebuilt binaries

Prebuilt binaries are provided in the [releases](https://github.com/cjbassi/ytop/releases) tab.

### Fedora

Available in main repo and more up to date version in [Copr](https://copr.fedorainfracloud.org/coprs/atim/ytop/).

```bash
sudo dnf install ytop
```

### From source

```bash
cargo install ytop
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
	- `p`: PID/Count
	- `n`: Command
	- `c`: CPU
	- `m`: Mem
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
    -a, --average-cpu    Show average CPU in the CPU widget
    -b, --battery        Show Battery widget (overridden by 'minimal' flag)
    -f, --fahrenheit     Show temperatures in fahrenheit
    -h, --help           Prints help information
    -m, --minimal        Only show the CPU, Mem, and Process widgets
    -p, --per-cpu        Show each CPU in the CPU widget
    -s, --statusbar      Show a statusbar with the time
    -V, --version        Prints version information

OPTIONS:
    -c, --colorscheme <colorscheme>    Set a colorscheme [default: default]
    -i, --interface <interface>        The name of the network interface to show in the Net widget. 'all' shows all
                                       interfaces [default: all]
    -I, --interval <interval>          Interval in seconds between updates of the CPU and Mem widgets. Can specify
                                       either a whole number or a fraction with a numerator of 1 [default: 1]
```

## Related projects

- [bashtop](https://github.com/aristocratos/bashtop)
- [bottom](https://github.com/ClementTsang/bottom)
- [glances](https://github.com/nicolargo/glances)
- [gotop](https://github.com/cjbassi/gotop)
- [gtop](https://github.com/aksakalli/gtop)
- [htop](https://github.com/hishamhm/htop)
- [vtop](https://github.com/MrRio/vtop)
- [zenith](https://github.com/bvaisvil/zenith)
