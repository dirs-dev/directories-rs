# Directories ![crates.io version](https://img.shields.io/crates/v/directories.svg) ![actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

## Introduction

- A tiny library with a minimal API (2 structs, 4 functions)
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
directories = "0.1.8"
```

into the `[dependencies]` section of your Cargo.toml file.

### Example

Library run by a user with user name "my_user_name" on Linux:

```rust
extern crate directories;
use directories::ProjectDirectories;

let my_proj_dirs = ProjectDirectories.from_project_name("My Project");
my_proj_dirs.project_config_dir; // "/home/my_user_name/.config/my-project/"
```

## Features

### `BaseDirectories`

The intended use-case for `BaseDirectories` is to query the paths of standard directories
that have been defined according to the conventions of operating system the library is running on.

If you want to compute the location of cache, config or data directories for your own application or project, use `ProjectDirectories` instead.

| Field name         | Value on Linux                                                     | Value on Windows                 | Value on macOS                       |
| ------------------ | ------------------------------------------------------------------ | -------------------------------- | ------------------------------------ |
| `home_dir`         | `$HOME`                                                            | `{FOLDERID_Profile}`             | `$HOME`                              |
| `cache_dir`        | `$XDG_CACHE_DIR`  or `~/.cache/`                                   | `{FOLDERID_LocalAppData}/cache/` | `$HOME/Library/Caches/`              |
| `config_dir`       | `$XDG_CONFIG_DIR` or `~/.config/`                                  | `{FOLDERID_RoamingAppData}`      | `$HOME/Library/Preferences/`         |
| `data_dir`         | `$XDG_DATA_DIR`   or `~/.local/share/`                             | `{FOLDERID_LocalAppData}`        | `$HOME/Library/Application Support/` |
| `data_roaming_dir` | `$XDG_DATA_DIR`   or `~/.local/share/`                             | `{FOLDERID_RoamingAppData}`      | `$HOME/Library/Application Support/` |
| `runtime_dir`      | `Some($XDG_RUNTIME_DIR)`                                           | `None`                           | `None`                               |
| `desktop_dir`      | `XDG_DESKTOP_DIR`                                                  | `{FOLDERID_Desktop}`             | `$HOME/Desktop/`                     |
| `documents_dir`    | `XDG_DOCUMENTS_DIR`                                                | `{FOLDERID_Documents}`           | `$HOME/Documents/`                   |
| `download_dir`     | `XDG_DOWNLOAD_DIR`                                                 | `{FOLDERID_Downloads}`           | `$HOME/Downloads/`                   |
| `music_dir`        | `XDG_MUSIC_DIR`                                                    | `{FOLDERID_Music}`               | `$HOME/Music/`                       |
| `pictures_dir`     | `XDG_PICTURES_DIR`                                                 | `{FOLDERID_Pictures}`            | `$HOME/Pictures/`                    |
| `public_dir`       | `XDG_PUBLICSHARE_DIR`                                              | `{FOLDERID_Public}`              | `$HOME/Public/`                      |
| `templates_dir`    | `XDG_TEMPLATES_DIR`                                                | `{FOLDERID_Templates}`           | `None`                               |
| `videos_dir`       | `XDG_VIDEOS_DIR`                                                   | `{FOLDERID_Videos}`              | `$HOME/Movies/`                      |
| `executables_dir`  | `Some($XDG_DATA_HOME/../bin/)` or `Some($HOME/.local/bin/)`        | `None`                           | `Some($HOME/Applications/)`          |
| `fonts_dir`        | `Some($XDG_DATA_HOME/fonts/)` or `Some($HOME/.local/share/fonts/)` | `None`                           | `Some($HOME/Library/Fonts/)`         |

### `ProjectDirectories`

The intended use-case for `ProjectDirectories` is to compute the location of cache, config or data directories for your own application or project,
which are derived from the standard directories.

| Field name                 | Value on Linux                                                                              | Value on Windows                                   | Value on macOS                                         |
| -------------------------- | ------------------------------------------------------------------------------------------- | -------------------------------------------------- | ------------------------------------------------------ |
| `project_cache_dir`        | `$XDG_CACHE_DIR/_yourprojectname_` or `$HOME/.cache/_yourprojectname_/`                     | `{FOLDERID_LocalAppData}/cache/_yourprojectname_/` | `$HOME/Library/Caches/_yourprojectname_/`              |
| `project_config_dir`       | `$XDG_CONFIG_DIR/_yourprojectname_`  or `$HOME/.config/_yourprojectname_/`                  | `{FOLDERID_RoamingAppData}/_yourprojectname_/`     | `$HOME/Library/Preferences/_yourprojectname_/`         |
| `project_data_dir`         | `$XDG_DATA_DIR/_yourprojectname_` or `$HOME/.local/share/_yourprojectname_/`                | `{FOLDERID_LocalAppData}/_yourprojectname_/`       | `$HOME/Library/Application Support/_yourprojectname_/` |
| `project_data_roaming_dir` | `$XDG_DATA_DIR/_yourprojectname_` or `$HOME/.local/share/_yourprojectname_/`                | `{FOLDERID_RoamingAppData}/_yourprojectname_/`     | `$HOME/Library/Application Support/_yourprojectname_/` |
| `project_runtime_dir`      | `Some($XDG_RUNTIME_DIR/_yourprojectname_)` or `Some($HOME/.local/share/_yourprojectname_/)` | `None`                                             | `None`                                                 |

The specific value of `_yourprojectname_` depends on the function used to create the `ProjectDirectories` struct:

| Function name                 | Example project name           | Value on Linux | Value on Windows | Value on macOS                 |
| ----------------------------- | ------------------------------ | -------------- | ---------------- | ------------------------------ |
| `from_unprocessed_string`     | `"FooBar-App"`                 | `"FooBar-App"` | `"FooBar-App"`   | `"FooBar-App"`                 |
| `from_project_name`           | `"FooBar App"`                 | `"foobar-app"` | `"FooBar App"`   | `"FooBar App"`                 |
| `from_qualified_project_name` | `"org.foobar-corp.FooBar-App"` | `"foobar-app"` | `"FooBar-App"`   | `"org.foobar-corp.FooBar-App"` |

## Versioning

After 1.0, the version number of this library consists of a whole number, which is incremented with each release.
(Think semantic versioning without _minor_ and _patch_ versions.)
