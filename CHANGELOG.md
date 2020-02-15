# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Types of changes**:
>
> - **Added**: for new features.
> - **Changed**: for changes in existing functionality.
> - **Deprecated**: for soon-to-be removed features.
> - **Removed**: for now removed features.
> - **Fixed**: for any bug fixes.
> - **Security**: in case of vulnerabilities.

## [Unreleased]

### Changed

- Change `rate` cli arg to `interval`

## [0.4.3] - 2020-02-14

### Fixed

- [disk] fix disk widget constantly resizing

## [0.4.2] - 2020-02-10

### Fixed

- fix several 'overflow when subtracting durations' panics
- fix ytop not clearing the screen when run on a TTY
- [cpu] fix CPUs overflowing widget
- [cpu][linux] fix cpu_percent calculation
- [temp][linux] fix panic due to 'not implemented'

## [0.4.1] - 2020-02-04

### Fixed

- [battery] fix battery percent
- [disk][unix] fix compilation on arm
- [disk] fix crash due to escaped partition mountpoint
- [proc] fix crash due to overflow when subtracting durations

## [0.4.0] - 2020-01-25

### Added

- add better-panic
- ignore mio trace logs in logfile
- [disk] fix panic if io counters not found
- [disk] fix mountpoint when partition mounted multiple times
- [disk][macos] add partition and disk usage support
- [mem] handle swap being disabled
- [temp] add degree symbol

## [0.3.0] - 2020-01-23

### Added

- Add preliminary macOS support

## [0.2.0] - 2020-01-20

### Added

- Color temperatures based on value
- Add sorting arrow to process header
- Show baseline spanning the bottom of the net sparklines
- Add sensor label to temperature identifier
- Process cpu percents are now working
- Draw the proc cursor
- Implement all keybinds except for proc filtering
- Add pausing with `Space`
- Add ability to sort processes by command

### Changed

- Group processes by default

### Fixed

- Linecharts now also draw points
- Fix process memory percent calculation

## 0.1.0 - 2020-01-13

Initial release!

[Unreleased]: https://github.com/cjbassi/ytop/compare/0.4.3...HEAD
[0.4.3]: https://github.com/cjbassi/ytop/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/cjbassi/ytop/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/cjbassi/ytop/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/cjbassi/ytop/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/cjbassi/ytop/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/cjbassi/ytop/compare/0.1.0...0.2.0
