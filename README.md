[![crates.io version](https://img.shields.io/crates/v/directories.svg)](https://crates.io/crates/directories) [![](https://docs.rs/directories/badge.svg)](https://docs.rs/directories/) ![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg) [![TravisCI status](https://travis-ci.org/soc/directories-rs.svg?branch=master)](https://travis-ci.org/soc/directories-rs) [![AppVeyor status](https://ci.appveyor.com/api/projects/status/p5c600gk0lthlhjn?svg=true)](https://ci.appveyor.com/project/soc/directories-rs)

# Directories

## Introduction

- A tiny library with a minimal API (2 structs, 3 factory functions, getters)
- that provides the platform-specific, user-accessible locations
- for storing configuration, cache and other data
- on Linux, Windows (≥ Vista) and macOS.

The library provides the location of these directories by leveraging the mechanisms defined by
- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and
  the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux,
- the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx) system on Windows, and
- the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
  on macOS.

## Usage

### Dependency

Add the library as a dependency to your project by inserting

```toml
directories = "0.6.0"
```

into the `[dependencies]` section of your Cargo.toml file.

### Example

Library run by user Alice:

```rust
extern crate directories;
use directories::ProjectDirs;

let proj_dirs = ProjectDirs::from("com", "Foo Corp",  "Bar App");
proj_dirs.config_dir();
// Linux:   /home/alice/.config/barapp/
// Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config\
// macOS:   /Users/Alice/Library/Preferences/com.Foo-Corp.Bar-App
```

## Features

### `BaseDirs`

The intended use-case for `BaseDirs` is to query the paths of standard directories
that have been defined according to the conventions of the operating system the library is running on.

If you want to compute the location of cache, config or data directories for your own application or project, use `ProjectDirs` instead.

| Function name    | Value on Linux                                                                               | Value on Windows                 | Value on macOS                       |
| ---------------- | -------------------------------------------------------------------------------------------- | -------------------------------- | ------------------------------------ |
| `home_dir`       | `$HOME`                                                                                      | `{FOLDERID_Profile}`             | `$HOME`                              |
| `cache_dir`      | `$XDG_CACHE_HOME`             or `~/.cache/`                                                 | `{FOLDERID_LocalAppData}` | `$HOME/Library/Caches/`              |
| `config_dir`     | `$XDG_CONFIG_HOME`            or `~/.config/`                                                | `{FOLDERID_RoamingAppData}`      | `$HOME/Library/Preferences/`         |
| `data_dir`       | `$XDG_DATA_HOME`              or `~/.local/share/`                                           | `{FOLDERID_RoamingAppData}`      | `$HOME/Library/Application Support/` |
| `data_local_dir` | `$XDG_DATA_HOME`              or `~/.local/share/`                                           | `{FOLDERID_LocalAppData}`        | `$HOME/Library/Application Support/` |
| `executable_dir` | `Some($XDG_BIN_HOME/../bin/)` or `Some($XDG_DATA_HOME/../bin/)` or `Some($HOME/.local/bin/)` | `None`                           | `None`                               |
| `runtime_dir`    | `Some($XDG_RUNTIME_DIR)`      or `None`                                                      | `None`                           | `None`                               |
| `audio_dir`      | `Some(XDG_MUSIC_DIR)`         or `None`                                                      | `Some({FOLDERID_Music})`         | `Some($HOME/Music/)`                 |
| `desktop_dir`    | `Some(XDG_DESKTOP_DIR)`       or `None`                                                      | `Some({FOLDERID_Desktop})`       | `Some($HOME/Desktop/)`               |
| `document_dir`   | `Some(XDG_DOCUMENTS_DIR)`     or `None`                                                      | `Some({FOLDERID_Documents})`     | `Some($HOME/Documents/)`             |
| `download_dir`   | `Some(XDG_DOWNLOAD_DIR)`      or `None`                                                      | `Some({FOLDERID_Downloads})`     | `Some($HOME/Downloads/)`             |
| `font_dir`       | `Some($XDG_DATA_HOME/fonts/)` or `Some($HOME/.local/share/fonts/)`                           | `None`                           | `Some($HOME/Library/Fonts/)`         |
| `picture_dir`    | `Some(XDG_PICTURES_DIR)`      or `None`                                                      | `Some({FOLDERID_Pictures})`      | `Some($HOME/Pictures/)`              |
| `public_dir`     | `Some(XDG_PUBLICSHARE_DIR)`   or `None`                                                      | `Some({FOLDERID_Public})`        | `Some($HOME/Public/)`                |
| `template_dir`   | `Some(XDG_TEMPLATES_DIR)`     or `None`                                                      | `Some({FOLDERID_Templates})`     | `None`                               |
| `video_dir`      | `Some(XDG_VIDEOS_DIR)`        or `None`                                                      | `Some({FOLDERID_Videos})`        | `Some($HOME/Movies/)`                |

### `ProjectDirs`

The intended use-case for `ProjectDirs` is to compute the location of cache, config or data directories for your own application or project,
which are derived from the standard directories.

| Function name    | Value on Linux                                                                  | Value on Windows                                | Value on macOS                                      |
| ---------------- | ------------------------------------------------------------------------------- | ----------------------------------------------- | --------------------------------------------------- |
| `cache_dir`      | `$XDG_CACHE_HOME/_project_path_`        or `$HOME/.cache/_project_path_/`       | `{FOLDERID_LocalAppData}/_project_path_/cache/` | `$HOME/Library/Caches/_project_path_/`              |
| `config_dir`     | `$XDG_CONFIG_HOME/_project_path_`       or `$HOME/.config/_project_path_/`      | `{FOLDERID_RoamingAppData}/_project_path_/`     | `$HOME/Library/Preferences/_project_path_/`         |
| `data_dir`       | `$XDG_DATA_HOME/_project_path_`         or `$HOME/.local/share/_project_path_/` | `{FOLDERID_RoamingAppData}/_project_path_/`     | `$HOME/Library/Application Support/_project_path_/` |
| `data_local_dir` | `$XDG_DATA_HOME/_project_path_`         or `$HOME/.local/share/_project_path_/` | `{FOLDERID_LocalAppData}/_project_path_/`       | `$HOME/Library/Application Support/_project_path_/` |
| `runtime_dir`    | `Some($XDG_RUNTIME_DIR/_project_path_)`                                         | `None`                                          | `None`                                              |

The specific value of `_project_path_` is computed by the

    ProjectDirs::from(qualifier: &str, organization: &str, project: &str)

function and varies across operating systems. As an example, calling

    ProjectDirs::from("org" /*qualifier*/, "Baz Corp" /*organization*/, "Foo Bar-App" /*application*/)

results in the following values:

| Value on Linux | Value on Windows         | Value on macOS               |
| -------------- | ------------------------ | ---------------------------- |
| `"foobar-app"` | `"Baz Corp/Foo Bar-App"` | `"org.Baz-Corp.Foo-Bar-App"` |

The `ProjectDirs::from_path` function allows the creation of `ProjectDirs` structs directly from a `PathBuf` value.
This argument is used verbatim and is not adapted to operating system standards.
The use of `ProjectDirs::from_path` is heavily discouraged, as its results will not follow operating system standards on at least two of three platforms.

## Versioning

After 1.0, the version number of this library consists of a whole number, which is incremented with each release.
(Think semantic versioning without _minor_ and _patch_ versions.)
