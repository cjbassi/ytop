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

### Added

- add better-panic
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

[Unreleased]: https://github.com/cjbassi/ytop/compare/0.3.0...HEAD
[0.3.0]: https://github.com/cjbassi/ytop/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/cjbassi/ytop/compare/0.1.0...0.2.0
