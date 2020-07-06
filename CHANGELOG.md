# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [3.0.1] - 2020-07-05

### Removed

-   Dependency on `cfg-if`

## [3.0.0] - 2020-06-21

### Changed

- **BREAKING CHANGE** The behavior of the `BaseDirs::config_dir` and `ProjectDirs::config_dir`
    on macOS has been adjusted (thanks to [everyone involved](https://github.com/dirs-dev/directories-rs/issues/62)):
  - The existing `config_dir` functions have been changed to return the `Application Support`
    directory on macOS, as suggested by Apple documentation.
  - The behavior of the `config_dir` functions on non-macOS platforms has not been changed.
  - If you have used the `config_dir` functions to store files, it may be necessary to write code
    that migrates the files to the new location on macOS.<br/>
    (Alternative: change uses of the `config_dir` functions to uses of the `preference_dir` functions
    to retain the old behavior.)
    
### Added
    
- The newly added `BaseDirs::preference_dir` and `ProjectDirs::preference_dir` functions returns
  the `Preferences` directory on macOS now, which – according to Apple documentation – shall only
  be used to store .plist files using Apple-proprietary APIs.
  – `preference_dir` and `config_dir` behave identical on non-macOS platforms.

### Fixed

- Compilation errors on Rust 1.13

## [2.0.2] - 2019-07-28

### Added

- Support for compilation on iOS
- Support for compilation on all wasm32 targets

### Fixed

- Unescapted backslashes in rust-docs

## [2.0.1] - 2019-05-31

### Changed

- Bump `dirs-sys` dependency to version 0.3.3

## [2.0.0] - 2020-05-26

### Changed

The behavior of deactivated, missing or invalid [_XDG User Dirs_](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)
entries on Linux has been improved (contributed by @tmiasko, thank you!):

- Version 1 returned the user's home directory (`Some($HOME)`) for such faulty entries, except for a faulty `XDG_DESKTOP_DIR` entry which returned (`Some($HOME/Desktop)`).
- Version 2 returns `None` for such entries.
