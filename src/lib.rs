use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "linux")]   mod lin;
#[cfg(target_os = "windows")] mod win;
#[cfg(target_os = "macos")]   mod mac;

#[cfg(target_os = "linux")]   pub use lin::base_dirs as base_dirs;
#[cfg(target_os = "windows")] pub use win::base_dirs as base_dirs;
#[cfg(target_os = "macos")]   pub use mac::base_dirs as base_dirs;

#[derive(Debug, Clone)]
pub struct BaseDirs {
    // home directory
    home:         PathBuf,

    // base directories
    cache:        PathBuf,
    config:       PathBuf,
    data:         PathBuf,
    data_local:   PathBuf,
    executable:   Option<PathBuf>,
    runtime:      Option<PathBuf>,

    // user directories
    audio:        PathBuf,
    desktop:      PathBuf,
    document:     PathBuf,
    download:     PathBuf,
    font:         Option<PathBuf>,
    picture:      PathBuf,
    public:       PathBuf,
    template:     Option<PathBuf>,
    video:        PathBuf
}

#[derive(Debug, Clone)]
pub struct ProjectDirs {
    project_name: String,

    // base directories
    cache:        PathBuf,
    config:       PathBuf,
    data:         PathBuf,
    data_local:   PathBuf,
    runtime:      Option<PathBuf>
}

#[deny(missing_docs)]
impl BaseDirs {
    /// Returns the path to the user's home directory.
    ///
    /// |Platform | Value                | Example       |
    /// | ------- | -------------------- | ------------- |
    /// | Linux   | `$HOME`              | /home/eve/    |
    /// | macOS   | `$HOME`              | /Users/eve/   |
    /// | Windows | `{FOLDERID_Profile}` | C:\Users\Eve\ |
    pub fn home(&self) -> &Path {
        self.home.as_path()
    }
    /// Returns the path to the user's cache directory.
    /// 
    /// |Platform | Value                             | Example                           |
    /// | ------- | --------------------------------- | --------------------------------- |
    /// | Linux   | `$XDG_CACHE_HOME` or `~/.cache/`  | /home/eve/.cache/                 |
    /// | macOS   | `$HOME/Library/Caches/`           | /Users/eve/Library/Caches/        |
    /// | Windows | `{FOLDERID_LocalAppData}\cache\`  | C:\Users\Eve\AppData\Local\cache\ |
    pub fn cache(&self) -> &Path {
        self.cache.as_path()
    }
    /// Returns the path to the user's config directory.
    /// 
    /// |Platform | Value                              | Example                         |
    /// | ------- | ---------------------------------- | ------------------------------- |
    /// | Linux   | `$XDG_CONFIG_HOME` or `~/.config/` | /home/eve/.config               |
    /// | macOS   | `$HOME/Library/Preferences/`       | /Users/eve/Library/Preferences/ |
    /// | Windows | `{FOLDERID_RoamingAppData}`        | C:\Users\Eve\AppData\Roaming\   |
    pub fn config(&self) -> &Path {
        self.config.as_path()
    }
    /// Returns the path to the user's data directory.
    /// 
    /// |Platform | Value                                 | Example                                 |
    /// | ------- | ------------------------------------- | --------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME` or `~/.local/share/` | /home/eve/.local/share/                 |
    /// | macOS   | `$HOME/Library/Application Support/`  | /Users/eve/Library/Application Support/ |
    /// | Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Eve\AppData\Roaming\           |
    pub fn data(&self) -> &Path {
        self.data.as_path()
    }
    /// Returns the path to the user's local data directory.
    /// 
    /// |Platform | Value                                 | Example                                 |
    /// | ------- | ------------------------------------- | --------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME` or `~/.local/share/` | /home/eve/.local/share/                 |
    /// | macOS   | `$HOME/Library/Application Support/`  | /Users/eve/Library/Application Support/ |
    /// | Windows | `{FOLDERID_LocalAppData}`             | C:\Users\Eve\AppData\Local\             |
    pub fn data_local(&self) -> &Path {
        self.data_local.as_path()
    }
    /// Returns the path to the user's executable directory.
    /// 
    /// |Platform | Value                                                          | Example                  |
    /// | ------- | -------------------------------------------------------------- | ------------------------ |
    /// | Linux   | `$XDG_BIN_HOME/` or `$XDG_DATA_HOME/../bin/` or `~/.local/bin` | /home/eve/.local/bin/    |
    /// | macOS   | –                                                              | –                        |
    /// | Windows | –                                                              | –                        |
    pub fn executable(&self) -> Option<&Path> {
        self.executable.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's runtime directory.
    /// 
    /// |Platform | Value              | Example         |
    /// | ------- | ------------------ | --------------- |
    /// | Linux   | `$XDG_RUNTIME_DIR` | /run/user/1001/ |
    /// | macOS   | –                  | –               |
    /// | Windows | –                  | –               |
    pub fn runtime(&self) -> Option<&Path> {
        self.runtime.as_ref().map(|p| p.as_path())
    }

    /// Returns the path to the user's audio directory.
    /// 
    /// |Platform | Value              | Example             |
    /// | ------- | ------------------ | ------------------- |
    /// | Linux   | `XDG_MUSIC_DIR`    | /home/eve/Music/    |
    /// | macOS   | `$HOME/Music/`     | /Users/eve/Music/   |
    /// | Windows | `{FOLDERID_Music}` | C:\Users\Eve\Music\ |
    pub fn audio(&self) -> &Path {
        self.audio.as_path()
    }
    /// Returns the path to the user's desktop directory.
    /// 
    /// |Platform | Value              | Example                 |
    /// | ------- | ------------------ | ----------------------- |
    /// | Linux   | `XDG_DESKTOP_DIR`    | /home/eve/Desktop/    |
    /// | macOS   | `$HOME/Desktop/`     | /Users/eve/Desktop/   |
    /// | Windows | `{FOLDERID_Desktop}` | C:\Users\Eve\Desktop\ |
    pub fn desktop(&self) -> &Path {
        self.desktop.as_path()
    }
    /// Returns the path to the user's document directory.
    /// 
    /// |Platform | Value                  | Example                 |
    /// | ------- | ---------------------- | ----------------------- |
    /// | Linux   | `XDG_DOCUMENTS_DIR`    | /home/eve/Documents/    |
    /// | macOS   | `$HOME/Documents/`     | /Users/eve/Documents/   |
    /// | Windows | `{FOLDERID_Documents}` | C:\Users\Eve\Documents\ |
    pub fn document(&self) -> &Path {
        self.document.as_path()
    }
    /// Returns the path to the user's download directory.
    /// 
    /// |Platform | Value                  | Example                 |
    /// | ------- | ---------------------- | ----------------------- |
    /// | Linux   | `XDG_DOWNLOAD_DIR`     | /home/eve/Downloads/    |
    /// | macOS   | `$HOME/Downloads/`     | /Users/eve/Downloads/   |
    /// | Windows | `{FOLDERID_Downloads}` | C:\Users\Eve\Downloads\ |
    pub fn download(&self) -> &Path {
        self.download.as_path()
    }
    /// Returns the path to the user's font directory.
    /// 
    /// |Platform | Value                                                  | Example                       |
    /// | ------- | ------------------------------------------------------ | ----------------------------- |
    /// | Linux   | `$XDG_DATA_HOME/fonts/` or `$HOME/.local/share/fonts/` | /home/eve/.local/share/fonts/ |
    /// | macOS   | `$HOME/Library/Fonts/`                                 | /Users/eve/Library/Fonts/     |
    /// | Windows | –                                                      | –                             |
    pub fn font(&self) -> Option<&Path> {
        self.font.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's picture directory.
    /// 
    /// |Platform | Value                 | Example                |
    /// | ------- | --------------------- | ---------------------- |
    /// | Linux   | `XDG_PICTURES_DIR`    | /home/eve/Pictures/    |
    /// | macOS   | `$HOME/Pictures/`     | /Users/eve/Pictures/   |
    /// | Windows | `{FOLDERID_Pictures}` | C:\Users\Eve\Pictures\ |
    pub fn picture(&self) -> &Path {
        self.picture.as_path()
    }
    /// Returns the path to the user's public directory.
    /// 
    /// |Platform | Value                 | Example            |
    /// | ------- | --------------------- | ------------------ |
    /// | Linux   | `XDG_PUBLICSHARE_DIR` | /home/eve/Public/  |
    /// | macOS   | `$HOME/Public/`       | /Users/eve/Public/ |
    /// | Windows | `{FOLDERID_Public}`   | C:\Users\Public\   |
    pub fn public(&self) -> &Path {
        self.public.as_path()
    }
    /// Returns the path to the user's template directory.
    /// 
    /// |Platform | Value                  | Example                                                   |
    /// | ------- | ---------------------- | --------------------------------------------------------- |
    /// | Linux   | `XDG_TEMPLATES_DIR`    | /home/eve/Templates/                                      |
    /// | macOS   | –                      | –                                                         |
    /// | Windows | `{FOLDERID_Templates}` | C:\Users\Eve\AppData\Roaming\Microsoft\Windows\Templates\ |
    pub fn template(&self) -> Option<&Path> {
        self.template.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's video directory.
    /// 
    /// |Platform | Value               | Example              |
    /// | ------- | ------------------- | -------------------- |
    /// | Linux   | `XDG_VIDEOS_DIR`    | /home/eve/Videos/    |
    /// | macOS   | `$HOME/Movies/`     | /Users/eve/Movies/   |
    /// | Windows | `{FOLDERID_Videos}` | C:\Users\Eve\Videos\ |
    pub fn video(&self) -> &Path {
        self.video.as_path()
    }
}

impl ProjectDirs {
    pub fn project_name(&self) -> &str {
        self.project_name.as_str()
    }
    pub fn project_cache(&self) -> &Path {
        self.cache.as_path()
    }
    pub fn project_config(&self) -> &Path {
        self.config.as_path()
    }
    pub fn project_data(&self) -> &Path {
        self.data.as_path()
    }
    pub fn project_data_local(&self) -> &Path {
        self.data_local.as_path()
    }
    pub fn project_runtime(&self) -> Option<&Path> {
        self.runtime.as_ref().map(|p| p.as_path())
    }
}

fn strip_qualification(name: &str) -> &str {
    name.rfind('.').map(|start| &name[start+1..]).unwrap_or(name)
}

#[cfg(test)]
mod tests {
    use strip_qualification;

    #[test]
    fn test_strip_qualification() {
        let actual1   = strip_qualification("org.foo.BarApp");
        let expected1 = "BarApp";
        assert_eq!(actual1, expected1);

        let actual2   = strip_qualification("BarApp");
        let expected2 = "BarApp";
        assert_eq!(actual2, expected2);
    }
}