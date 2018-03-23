[![crates.io](https://img.shields.io/crates/v/directories.svg)](https://crates.io/crates/directories)
[![API documentation](https://docs.rs/directories/badge.svg)](https://docs.rs/directories/)
![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![TravisCI status](https://img.shields.io/travis/soc/directories-rs/master.svg?label=Linux/macOS%20build)](https://travis-ci.org/soc/directories-rs)
[![AppVeyor status](https://img.shields.io/appveyor/ci/soc/directories-rs/master.svg?label=Windows%20build)](https://ci.appveyor.com/project/soc/directories-rs/branch/master)
[![License: MPL-2.0](https://img.shields.io/github/license/soc/directories-rs.svg)](LICENSE)

# Directories

## Introduction

- a tiny library with a minimal API
- that provides the platform-specific, user-accessible locations
- for retrieving and storing configuration, cache and other data
- on Linux, Windows (≥ Vista) and macOS.

The library provides the location of these directories by leveraging the mechanisms defined by
- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and
  the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux
- the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx) API on Windows
- the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
  guidelines on macOS

## Platforms

This library is written in Rust, and supports Linux, macOS and Windows.

A version of this library running on the JVM is provided by [directories-jvm](https://github.com/soc/directories-jvm).

## Usage

#### Dependency

Add the library as a dependency to your project by inserting

```toml
directories = "0.8.4"
```

into the `[dependencies]` section of your Cargo.toml file.

#### Example

Library run by user Alice:

```rust
extern crate directories;
use directories::{BaseDirs, UserDirs, ProjectDirs};

let proj_dirs = ProjectDirs::from("com", "Foo Corp",  "Bar App");
proj_dirs.config_dir();
// Lin: /home/alice/.config/barapp
// Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
// Mac: /Users/Alice/Library/Preferences/com.Foo-Corp.Bar-App

let base_dirs = BaseDirs::new();
base_dirs.executable_dir();
// Lin: Some(/home/alice/.local/share/bin)
// Win: None
// Mac: None

let user_dirs = UserDirs::new();
user_dirs.audio_dir();
// Lin: /home/alice/Music
// Win: C:\Users\Alice\Music
// Mac: /Users/Alice/Music
```

## Design Goals

- The _directories_ library is designed to provide an accurate snapshot of the system's state at
  the point of invocation of `BaseDirs::new`, `UserDirs::new` or `ProjectDirs::from`. Subsequent
  changes to the state of the system are not reflected in values created prior to such a change.
- This library does not create directories or check for their existence. The library only provides
  information on what the path to a certain directory _should_ be. How this information is used is
  a decision that developers need to make based on the requirements of each individual application.
- This library is intentionally focused on providing information on user-writable directories only.
  There is no discernible benefit in returning a path that points to a user-level, writable
  directory on one operating system, but a system-level, read-only directory on another, that would
  outweigh the confusion and unexpected failures such an approach would cause.
  - `executable_dir` is specified to provide the path to a user-writable directory for binaries.<br/>
    As such a directory only commonly exists on Linux, it returns `None` on macOS and Windows.
  - `font_dir` is specified to provide the path to a user-writable directory for fonts.<br/>
    As such a directory only exists on Linux and macOS, it returns `None` Windows.
  - `runtime_dir` is specified to provide the path to a directory for non-essential runtime data.
    It is required that this directory is created when the user logs in, is only accessible by the
    user itself, is deleted when the user logs out, and supports all filesystem features of the
    operating system.<br/>
    As such a directory only commonly exists on Linux, it returns `None` on macOS and Windows.

## Features

### `BaseDirs`

The intended use-case for `BaseDirs` is to query the paths of user-invisible standard directories
that have been defined according to the conventions of the operating system the library is running on.

If you want to compute the location of cache, config or data directories for your own application or project, use `ProjectDirs` instead.

| Function name    | Value on Linux                                                                             | Value on Windows                 | Value on macOS                      |
| ---------------- | ------------------------------------------------------------------------------------------ | -------------------------------- | ----------------------------------- |
| `home_dir`       | `$HOME`                                                                                    | `{FOLDERID_Profile}`             | `$HOME`                             |
| `cache_dir`      | `$XDG_CACHE_HOME`             or `$HOME/.cache`                                            | `{FOLDERID_LocalAppData}`        | `$HOME/Library/Caches`              |
| `config_dir`     | `$XDG_CONFIG_HOME`            or `$HOME/.config`                                           | `{FOLDERID_RoamingAppData}`      | `$HOME/Library/Preferences`         |
| `data_dir`       | `$XDG_DATA_HOME`              or `$HOME/.local/share`                                      | `{FOLDERID_RoamingAppData}`      | `$HOME/Library/Application Support` |
| `data_local_dir` | `$XDG_DATA_HOME`              or `$HOME/.local/share`                                      | `{FOLDERID_LocalAppData}`        | `$HOME/Library/Application Support` |
| `executable_dir` | `Some($XDG_BIN_HOME/../bin)`  or `Some($XDG_DATA_HOME/../bin)` or `Some($HOME/.local/bin)` | `None`                           | `None`                              |
| `runtime_dir`    | `Some($XDG_RUNTIME_DIR)`      or `None`                                                    | `None`                           | `None`                              |

### `UserDirs`

The intended use-case for `UserDirs` is to query the paths of user-facing standard directories
that have been defined according to the conventions of the operating system the library is running on.

| Function name    | Value on Linux                                                                             | Value on Windows                 | Value on macOS                      |
| ---------------- | ------------------------------------------------------------------------------------------ | -------------------------------- | ----------------------------------- |
| `home_dir`       | `$HOME`                                                                                    | `{FOLDERID_Profile}`             | `$HOME`                             |
| `audio_dir`      | `Some(XDG_MUSIC_DIR)`         or `None`                                                    | `Some({FOLDERID_Music})`         | `Some($HOME/Music/)`                |
| `desktop_dir`    | `Some(XDG_DESKTOP_DIR)`       or `None`                                                    | `Some({FOLDERID_Desktop})`       | `Some($HOME/Desktop/)`              |
| `document_dir`   | `Some(XDG_DOCUMENTS_DIR)`     or `None`                                                    | `Some({FOLDERID_Documents})`     | `Some($HOME/Documents/)`            |
| `download_dir`   | `Some(XDG_DOWNLOAD_DIR)`      or `None`                                                    | `Some({FOLDERID_Downloads})`     | `Some($HOME/Downloads/)`            |
| `font_dir`       | `Some($XDG_DATA_HOME/fonts/)` or `Some($HOME/.local/share/fonts/)`                         | `None`                           | `Some($HOME/Library/Fonts/)`        |
| `picture_dir`    | `Some(XDG_PICTURES_DIR)`      or `None`                                                    | `Some({FOLDERID_Pictures})`      | `Some($HOME/Pictures/)`             |
| `public_dir`     | `Some(XDG_PUBLICSHARE_DIR)`   or `None`                                                    | `Some({FOLDERID_Public})`        | `Some($HOME/Public/)`               |
| `template_dir`   | `Some(XDG_TEMPLATES_DIR)`     or `None`                                                    | `Some({FOLDERID_Templates})`     | `None`                              |
| `video_dir`      | `Some(XDG_VIDEOS_DIR)`        or `None`                                                    | `Some({FOLDERID_Videos})`        | `Some($HOME/Movies/)`               |

<!--| `trash_dir`      | `$XDG_DATA_HOME/Trash`        or `$HOME/.local/share/Trash`                                | `???`                            | `$HOME/.trash`                      |-->

### `ProjectDirs`

The intended use-case for `ProjectDirs` is to compute the location of cache, config or data directories for your own application or project,
which are derived from the standard directories.

| Function name    | Value on Linux                                                                  | Value on Windows                                | Value on macOS                                      |
| ---------------- | ------------------------------------------------------------------------------- | ----------------------------------------------- | --------------------------------------------------- |
| `cache_dir`      | `$XDG_CACHE_HOME/_project_path_`        or `$HOME/.cache/_project_path_`        | `{FOLDERID_LocalAppData}/_project_path_/cache`  | `$HOME/Library/Caches/_project_path_`               |
| `config_dir`     | `$XDG_CONFIG_HOME/_project_path_`       or `$HOME/.config/_project_path_`       | `{FOLDERID_RoamingAppData}/_project_path_`      | `$HOME/Library/Preferences/_project_path_`          |
| `data_dir`       | `$XDG_DATA_HOME/_project_path_`         or `$HOME/.local/share/_project_path_`  | `{FOLDERID_RoamingAppData}/_project_path_`      | `$HOME/Library/Application Support/_project_path_`  |
| `data_local_dir` | `$XDG_DATA_HOME/_project_path_`         or `$HOME/.local/share/_project_path_`  | `{FOLDERID_LocalAppData}/_project_path_`        | `$HOME/Library/Application Support/_project_path_`  |
| `runtime_dir`    | `Some($XDG_RUNTIME_DIR/_project_path_)`                                         | `None`                                          | `None`                                              |

The specific value of `_project_path_` is computed by the

    ProjectDirs::from(qualifier: &str,
                      organization: &str,
                      application: &str)

function and varies across operating systems. As an example, calling

    ProjectDirs::from("org"         /*qualifier*/,
                      "Baz Corp"    /*organization*/,
                      "Foo Bar-App" /*application*/)

results in the following values:

| Value on Linux | Value on Windows         | Value on macOS               |
| -------------- | ------------------------ | ---------------------------- |
| `"foobar-app"` | `"Baz Corp/Foo Bar-App"` | `"org.Baz-Corp.Foo-Bar-App"` |

The `ProjectDirs::from_path` function allows the creation of `ProjectDirs` structs directly from a `PathBuf` value.
This argument is used verbatim and is not adapted to operating system standards.
The use of `ProjectDirs::from_path` is strongly discouraged, as its results will not follow operating system standards on at least two of three platforms.

## Versioning

After 1.0, the version number of this library consists of a whole number, which is incremented with each release.
(Think semantic versioning without _minor_ and _patch_ versions.)
