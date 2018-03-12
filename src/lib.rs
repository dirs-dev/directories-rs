//! The _directories_ crate is
//!
//! - a tiny library with a minimal API (2 structs, 3 factory functions, getters)
//! - that provides the platform-specific, user-accessible locations
//! - for storing configuration, cache and other data
//! - on Linux, Windows (≥ Vista) and macOS.
//!
//! The library provides the location of these directories by leveraging the mechanisms defined by
//!
//! - the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) and the [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specifications on Linux,
//! - the [Known Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx) system on Windows, and
//! - the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6) on macOS.
//! 

use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "linux")]   mod lin;
#[cfg(target_os = "windows")] mod win;
#[cfg(target_os = "macos")]   mod mac;

/// `BaseDirs` provides paths to standard directories, following the conventions of the operating system the library is running on.
///
/// To compute the location of cache, config or data directories for individual projects or applications, use `ProjectDirs` instead.
///
/// # Examples
///
/// All examples on this page are computed with a user named _Alice_.
///
/// ```
/// use directories::BaseDirs;
///
/// let base_dirs = BaseDirs::new();
///
/// base_dirs.config_dir();
/// // Linux:   /home/alice/.config/
/// // Windows: C:\Users\Alice\AppData\Roaming\
/// // macOS:   /Users/Alice/Library/Preferences/
///
/// base_dirs.audio_dir();
/// // Linux:   /home/alice/Music/
/// // Windows: /Users/Alice/Music/
/// // macOS:   C:\Users\Alice\Music\
/// ```
#[derive(Debug, Clone)]
pub struct BaseDirs {
    // home directory
    home_dir:       PathBuf,

    // base directories
    cache_dir:      PathBuf,
    config_dir:     PathBuf,
    data_dir:       PathBuf,
    data_local_dir: PathBuf,
    executable_dir: Option<PathBuf>,
    runtime_dir:    Option<PathBuf>,

    // user directories
    audio_dir:      Option<PathBuf>,
    desktop_dir:    Option<PathBuf>,
    document_dir:   Option<PathBuf>,
    download_dir:   Option<PathBuf>,
    font_dir:       Option<PathBuf>,
    picture_dir:    Option<PathBuf>,
    public_dir:     Option<PathBuf>,
    template_dir:   Option<PathBuf>,
    video_dir:      Option<PathBuf>
}

/// `ProjectDirs` computes the location of cache, config or data directories for a specific application,
/// which are derived from the standard directories and the name of the project/organization.
///
/// # Examples
///
/// All examples on this page are computed with a user named _Alice_,
/// and a `ProjectDirs` struct created with `ProjectDirs::from("com", "Foo Corp", "Bar App")`.
///
/// ```
/// use directories::ProjectDirs;
///
/// let proj_dirs = ProjectDirs::from("com", "Foo Corp",  "Bar App");
/// proj_dirs.config_dir();
/// // Linux:   /home/alice/.config/barapp/
/// // Windows: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App
/// // macOS:   /Users/Alice/Library/Preferences/com.Foo-Corp.Bar-App
/// ```
#[derive(Debug, Clone)]
pub struct ProjectDirs {
    project_path:   PathBuf,
    
    // base directories
    cache_dir:      PathBuf,
    config_dir:     PathBuf,
    data_dir:       PathBuf,
    data_local_dir: PathBuf,
    runtime_dir:    Option<PathBuf>
}

#[deny(missing_docs)]
impl BaseDirs {
    /// Returns the path to the user's home directory.
    ///
    /// |Platform | Value                | Example         |
    /// | ------- | -------------------- | --------------- |
    /// | Linux   | `$HOME`              | /home/alice/    |
    /// | macOS   | `$HOME`              | /Users/Alice/   |
    /// | Windows | `{FOLDERID_Profile}` | C:\Users\Alice\ |
    pub fn home_dir(&self) -> &Path {
        self.home_dir.as_path()
    }
    /// Returns the path to the user's cache directory.
    ///
    /// |Platform | Value                             | Example                       |
    /// | ------- | --------------------------------- | ----------------------------- |
    /// | Linux   | `$XDG_CACHE_HOME` or `~/.cache/`  | /home/alice/.cache/           |
    /// | macOS   | `$HOME/Library/Caches/`           | /Users/Alice/Library/Caches/  |
    /// | Windows | `{FOLDERID_LocalAppData}\cache\`  | C:\Users\Alice\AppData\Local\ |
    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }
    /// Returns the path to the user's config directory.
    ///
    /// |Platform | Value                              | Example                           |
    /// | ------- | ---------------------------------- | --------------------------------- |
    /// | Linux   | `$XDG_CONFIG_HOME` or `~/.config/` | /home/alice/.config               |
    /// | macOS   | `$HOME/Library/Preferences/`       | /Users/Alice/Library/Preferences/ |
    /// | Windows | `{FOLDERID_RoamingAppData}`        | C:\Users\Alice\AppData\Roaming\   |
    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }
    /// Returns the path to the user's data directory.
    ///
    /// |Platform | Value                                 | Example                                   |
    /// | ------- | ------------------------------------- | ----------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME` or `~/.local/share/` | /home/alice/.local/share/                 |
    /// | macOS   | `$HOME/Library/Application Support/`  | /Users/Alice/Library/Application Support/ |
    /// | Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming\           |
    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
    /// Returns the path to the user's local data directory.
    ///
    /// |Platform | Value                                 | Example                                   |
    /// | ------- | ------------------------------------- | ----------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME` or `~/.local/share/` | /home/alice/.local/share/                 |
    /// | macOS   | `$HOME/Library/Application Support/`  | /Users/Alice/Library/Application Support/ |
    /// | Windows | `{FOLDERID_LocalAppData}`             | C:\Users\Alice\AppData\Local\             |
    pub fn data_local_dir(&self) -> &Path {
        self.data_local_dir.as_path()
    }
    /// Returns the path to the user's executable directory.
    ///
    /// |Platform | Value                                                          | Example                  |
    /// | ------- | -------------------------------------------------------------- | ------------------------ |
    /// | Linux   | `$XDG_BIN_HOME/` or `$XDG_DATA_HOME/../bin/` or `~/.local/bin` | /home/alice/.local/bin/  |
    /// | macOS   | –                                                              | –                        |
    /// | Windows | –                                                              | –                        |
    pub fn executable_dir(&self) -> Option<&Path> {
        self.executable_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's runtime directory.
    ///
    /// |Platform | Value              | Example         |
    /// | ------- | ------------------ | --------------- |
    /// | Linux   | `$XDG_RUNTIME_DIR` | /run/user/1001/ |
    /// | macOS   | –                  | –               |
    /// | Windows | –                  | –               |
    pub fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_ref().map(|p| p.as_path())
    }

    /// Returns the path to the user's audio directory.
    ///
    /// |Platform | Value              | Example               |
    /// | ------- | ------------------ | --------------------- |
    /// | Linux   | `XDG_MUSIC_DIR`    | /home/alice/Music/    |
    /// | macOS   | `$HOME/Music/`     | /Users/Alice/Music/   |
    /// | Windows | `{FOLDERID_Music}` | C:\Users\Alice\Music\ |
    pub fn audio_dir(&self) -> Option<&Path> {
        self.audio_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's desktop directory.
    ///
    /// |Platform | Value                | Example                 |
    /// | ------- | -------------------- | ----------------------- |
    /// | Linux   | `XDG_DESKTOP_DIR`    | /home/alice/Desktop/    |
    /// | macOS   | `$HOME/Desktop/`     | /Users/Alice/Desktop/   |
    /// | Windows | `{FOLDERID_Desktop}` | C:\Users\Alice\Desktop\ |
    pub fn desktop_dir(&self) -> Option<&Path> {
        self.desktop_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's document directory.
    ///
    /// |Platform | Value                  | Example                   |
    /// | ------- | ---------------------- | ------------------------- |
    /// | Linux   | `XDG_DOCUMENTS_DIR`    | /home/alice/Documents/    |
    /// | macOS   | `$HOME/Documents/`     | /Users/Alice/Documents/   |
    /// | Windows | `{FOLDERID_Documents}` | C:\Users\Alice\Documents\ |
    pub fn document_dir(&self) -> Option<&Path> {
        self.document_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's download directory.
    ///
    /// |Platform | Value                  | Example                   |
    /// | ------- | ---------------------- | ------------------------- |
    /// | Linux   | `XDG_DOWNLOAD_DIR`     | /home/alice/Downloads/    |
    /// | macOS   | `$HOME/Downloads/`     | /Users/Alice/Downloads/   |
    /// | Windows | `{FOLDERID_Downloads}` | C:\Users\Alice\Downloads\ |
    pub fn download_dir(&self) -> Option<&Path> {
        self.download_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's font directory.
    ///
    /// |Platform | Value                                                  | Example                         |
    /// | ------- | ------------------------------------------------------ | ------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME/fonts/` or `$HOME/.local/share/fonts/` | /home/alice/.local/share/fonts/ |
    /// | macOS   | `$HOME/Library/Fonts/`                                 | /Users/Alice/Library/Fonts/     |
    /// | Windows | –                                                      | –                               |
    pub fn font_dir(&self) -> Option<&Path> {
        self.font_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's picture directory.
    ///
    /// |Platform | Value                 | Example                  |
    /// | ------- | --------------------- | ------------------------ |
    /// | Linux   | `XDG_PICTURES_DIR`    | /home/alice/Pictures/    |
    /// | macOS   | `$HOME/Pictures/`     | /Users/Alice/Pictures/   |
    /// | Windows | `{FOLDERID_Pictures}` | C:\Users\Alice\Pictures\ |
    pub fn picture_dir(&self) -> Option<&Path> {
        self.picture_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's public directory.
    ///
    /// |Platform | Value                 | Example              |
    /// | ------- | --------------------- | -------------------- |
    /// | Linux   | `XDG_PUBLICSHARE_DIR` | /home/alice/Public/  |
    /// | macOS   | `$HOME/Public/`       | /Users/Alice/Public/ |
    /// | Windows | `{FOLDERID_Public}`   | C:\Users\Public\     |
    pub fn public_dir(&self) -> Option<&Path> {
        self.public_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's template directory.
    ///
    /// |Platform | Value                  | Example                                                     |
    /// | ------- | ---------------------- | ----------------------------------------------------------- |
    /// | Linux   | `XDG_TEMPLATES_DIR`    | /home/alice/Templates/                                      |
    /// | macOS   | –                      | –                                                           |
    /// | Windows | `{FOLDERID_Templates}` | C:\Users\Alice\AppData\Roaming\Microsoft\Windows\Templates\ |
    pub fn template_dir(&self) -> Option<&Path> {
        self.template_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's video directory.
    ///
    /// |Platform | Value               | Example                |
    /// | ------- | ------------------- | ---------------------- |
    /// | Linux   | `XDG_VIDEOS_DIR`    | /home/alice/Videos/    |
    /// | macOS   | `$HOME/Movies/`     | /Users/Alice/Movies/   |
    /// | Windows | `{FOLDERID_Videos}` | C:\Users\Alice\Videos\ |
    pub fn video_dir(&self) -> Option<&Path> {
        self.video_dir.as_ref().map(|p| p.as_path())
    }
}

#[deny(missing_docs)]
impl ProjectDirs {
    /// Returns the project path fragment used to compute the project's cache/config/data directories.
    /// The value is derived from the `ProjectDirs::from` call and is platform-dependent.
    pub fn project_path(&self) -> &Path {
        self.project_path.as_path()
    }
    /// Returns the path to the project's cache directory.
    /// 
    /// |Platform | Value                                                          | Example                                                 |
    /// | ------- | -------------------------------------------------------------- | ------------------------------------------------------- |
    /// | Linux   | `$XDG_CACHE_HOME_project_path_/` or `~/.cache/_project_path_/` | /home/alice/.cache/barapp/                              |
    /// | macOS   | `$HOME/Library/Caches/_project_path_/`                         | /Users/Alice/Library/Caches/com.Foo-Corp.Bar-App/cache/ |
    /// | Windows | `{FOLDERID_LocalAppData}\_project_path_\cache\`                | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\cache\    |
    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }
    /// Returns the path to the project's config directory.
    /// 
    /// |Platform | Value                                                             | Example                                                      |
    /// | ------- | ----------------------------------------------------------------- | ------------------------------------------------------------ |
    /// | Linux   | `$XDG_CONFIG_HOME/_project_path_/` or `~/.config/_project_path_/` | /home/alice/.config/barapp/                                  |
    /// | macOS   | `$HOME/Library/Preferences/_project_path_/`                       | /Users/Alice/Library/Preferences/com.Foo-Corp.Bar-App/cache/ |
    /// | Windows | `{FOLDERID_RoamingAppData}\_project_path_\config\`                | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config\      |
    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }
    /// Returns the path to the project's data directory.
    /// 
    /// |Platform | Value                                                                | Example                                                             |
    /// | ------- | -------------------------------------------------------------------- | ------------------------------------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME/_project_path_/` or `~/.local/share/_project_path_/` | /home/alice/.local/share/barapp/                                    |
    /// | macOS   | `$HOME/Library/Application Support/_project_path_/`                  | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App/data/ |
    /// | Windows | `{FOLDERID_RoamingAppData}\_project_path_\data\`                     | C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\data\               |
    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
    /// Returns the path to the project's local data directory.
    /// 
    /// |Platform | Value                                                                | Example                                                             |
    /// | ------- | -------------------------------------------------------------------- | ------------------------------------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME/_project_path_/` or `~/.local/share/_project_path_/` | /home/alice/.local/share/barapp/                                    |
    /// | macOS   | `$HOME/Library/Application Support/_project_path_/`                  | /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App/data/ |
    /// | Windows | `{FOLDERID_LocalAppData}\_project_path_\data\`                       | C:\Users\Alice\AppData\Local\Foo Corp\Bar App\data\                 |
    pub fn data_local_dir(&self) -> &Path {
        self.data_local_dir.as_path()
    }
    /// Returns the path to the project's runtime directory.
    /// 
    /// |Platform | Value              | Example                |
    /// | ------- | ------------------ | ---------------------- |
    /// | Linux   | `$XDG_RUNTIME_DIR` | /run/user/1001/barapp/ |
    /// | macOS   | –                  | –                      |
    /// | Windows | –                  | –                      |
    pub fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_ref().map(|p| p.as_path())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base_dirs() {
        println!("{:?}", ::BaseDirs::new());
    }

    #[test]
    fn test_project_dirs() {
        let proj_dirs = ::ProjectDirs::from("qux", "FooCorp", "BarApp");
        println!(r#"ProjectDirs::from("qux", "FooCorp", "BarApp")"#);
        println!("{:?}", proj_dirs);
        let proj_dirs = ::ProjectDirs::from("qux.zoo", "Foo Corp", "Bar-App");
        println!(r#"ProjectDirs::from("qux.zoo", "Foo Corp", "Bar-App")"#);
        println!("{:?}", proj_dirs);
        let proj_dirs = ::ProjectDirs::from("com", "Foo Corp.", "Bar App");
        println!(r#"ProjectDirs::from("com", "Foo Corp.", "Bar App")"#);
        println!("{:?}", proj_dirs);
    }
}
