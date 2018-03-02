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

#[derive(Debug, Clone)]
pub struct ProjectDirs {
    project_name:   String,
    
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
    /// |Platform | Value                | Example       |
    /// | ------- | -------------------- | ------------- |
    /// | Linux   | `$HOME`              | /home/eve/    |
    /// | macOS   | `$HOME`              | /Users/eve/   |
    /// | Windows | `{FOLDERID_Profile}` | C:\Users\Eve\ |
    pub fn home_dir(&self) -> &Path {
        self.home_dir.as_path()
    }
    /// Returns the path to the user's cache directory.
    /// 
    /// |Platform | Value                             | Example                           |
    /// | ------- | --------------------------------- | --------------------------------- |
    /// | Linux   | `$XDG_CACHE_HOME` or `~/.cache/`  | /home/eve/.cache/                 |
    /// | macOS   | `$HOME/Library/Caches/`           | /Users/eve/Library/Caches/        |
    /// | Windows | `{FOLDERID_LocalAppData}\cache\`  | C:\Users\Eve\AppData\Local\cache\ |
    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }
    /// Returns the path to the user's config directory.
    /// 
    /// |Platform | Value                              | Example                         |
    /// | ------- | ---------------------------------- | ------------------------------- |
    /// | Linux   | `$XDG_CONFIG_HOME` or `~/.config/` | /home/eve/.config               |
    /// | macOS   | `$HOME/Library/Preferences/`       | /Users/eve/Library/Preferences/ |
    /// | Windows | `{FOLDERID_RoamingAppData}`        | C:\Users\Eve\AppData\Roaming\   |
    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }
    /// Returns the path to the user's data directory.
    /// 
    /// |Platform | Value                                 | Example                                 |
    /// | ------- | ------------------------------------- | --------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME` or `~/.local/share/` | /home/eve/.local/share/                 |
    /// | macOS   | `$HOME/Library/Application Support/`  | /Users/eve/Library/Application Support/ |
    /// | Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Eve\AppData\Roaming\           |
    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
    /// Returns the path to the user's local data directory.
    /// 
    /// |Platform | Value                                 | Example                                 |
    /// | ------- | ------------------------------------- | --------------------------------------- |
    /// | Linux   | `$XDG_DATA_HOME` or `~/.local/share/` | /home/eve/.local/share/                 |
    /// | macOS   | `$HOME/Library/Application Support/`  | /Users/eve/Library/Application Support/ |
    /// | Windows | `{FOLDERID_LocalAppData}`             | C:\Users\Eve\AppData\Local\             |
    pub fn data_local_dir(&self) -> &Path {
        self.data_local_dir.as_path()
    }
    /// Returns the path to the user's executable directory.
    /// 
    /// |Platform | Value                                                          | Example                  |
    /// | ------- | -------------------------------------------------------------- | ------------------------ |
    /// | Linux   | `$XDG_BIN_HOME/` or `$XDG_DATA_HOME/../bin/` or `~/.local/bin` | /home/eve/.local/bin/    |
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
    /// |Platform | Value              | Example             |
    /// | ------- | ------------------ | ------------------- |
    /// | Linux   | `XDG_MUSIC_DIR`    | /home/eve/Music/    |
    /// | macOS   | `$HOME/Music/`     | /Users/eve/Music/   |
    /// | Windows | `{FOLDERID_Music}` | C:\Users\Eve\Music\ |
    pub fn audio_dir(&self) -> Option<&Path> {
        self.audio_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's desktop directory.
    /// 
    /// |Platform | Value              | Example                 |
    /// | ------- | ------------------ | ----------------------- |
    /// | Linux   | `XDG_DESKTOP_DIR`    | /home/eve/Desktop/    |
    /// | macOS   | `$HOME/Desktop/`     | /Users/eve/Desktop/   |
    /// | Windows | `{FOLDERID_Desktop}` | C:\Users\Eve\Desktop\ |
    pub fn desktop_dir(&self) -> Option<&Path> {
        self.desktop_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's document directory.
    /// 
    /// |Platform | Value                  | Example                 |
    /// | ------- | ---------------------- | ----------------------- |
    /// | Linux   | `XDG_DOCUMENTS_DIR`    | /home/eve/Documents/    |
    /// | macOS   | `$HOME/Documents/`     | /Users/eve/Documents/   |
    /// | Windows | `{FOLDERID_Documents}` | C:\Users\Eve\Documents\ |
    pub fn document_dir(&self) -> Option<&Path> {
        self.document_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's download directory.
    /// 
    /// |Platform | Value                  | Example                 |
    /// | ------- | ---------------------- | ----------------------- |
    /// | Linux   | `XDG_DOWNLOAD_DIR`     | /home/eve/Downloads/    |
    /// | macOS   | `$HOME/Downloads/`     | /Users/eve/Downloads/   |
    /// | Windows | `{FOLDERID_Downloads}` | C:\Users\Eve\Downloads\ |
    pub fn download_dir(&self) -> Option<&Path> {
        self.download_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's font directory.
    /// 
    /// |Platform | Value                                                  | Example                       |
    /// | ------- | ------------------------------------------------------ | ----------------------------- |
    /// | Linux   | `$XDG_DATA_HOME/fonts/` or `$HOME/.local/share/fonts/` | /home/eve/.local/share/fonts/ |
    /// | macOS   | `$HOME/Library/Fonts/`                                 | /Users/eve/Library/Fonts/     |
    /// | Windows | –                                                      | –                             |
    pub fn font_dir(&self) -> Option<&Path> {
        self.font_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's picture directory.
    /// 
    /// |Platform | Value                 | Example                |
    /// | ------- | --------------------- | ---------------------- |
    /// | Linux   | `XDG_PICTURES_DIR`    | /home/eve/Pictures/    |
    /// | macOS   | `$HOME/Pictures/`     | /Users/eve/Pictures/   |
    /// | Windows | `{FOLDERID_Pictures}` | C:\Users\Eve\Pictures\ |
    pub fn picture_dir(&self) -> Option<&Path> {
        self.picture_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's public directory.
    /// 
    /// |Platform | Value                 | Example            |
    /// | ------- | --------------------- | ------------------ |
    /// | Linux   | `XDG_PUBLICSHARE_DIR` | /home/eve/Public/  |
    /// | macOS   | `$HOME/Public/`       | /Users/eve/Public/ |
    /// | Windows | `{FOLDERID_Public}`   | C:\Users\Public\   |
    pub fn public_dir(&self) -> Option<&Path> {
        self.public_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's template directory.
    /// 
    /// |Platform | Value                  | Example                                                   |
    /// | ------- | ---------------------- | --------------------------------------------------------- |
    /// | Linux   | `XDG_TEMPLATES_DIR`    | /home/eve/Templates/                                      |
    /// | macOS   | –                      | –                                                         |
    /// | Windows | `{FOLDERID_Templates}` | C:\Users\Eve\AppData\Roaming\Microsoft\Windows\Templates\ |
    pub fn template_dir(&self) -> Option<&Path> {
        self.template_dir.as_ref().map(|p| p.as_path())
    }
    /// Returns the path to the user's video directory.
    /// 
    /// |Platform | Value               | Example              |
    /// | ------- | ------------------- | -------------------- |
    /// | Linux   | `XDG_VIDEOS_DIR`    | /home/eve/Videos/    |
    /// | macOS   | `$HOME/Movies/`     | /Users/eve/Movies/   |
    /// | Windows | `{FOLDERID_Videos}` | C:\Users\Eve\Videos\ |
    pub fn video_dir(&self) -> Option<&Path> {
        self.video_dir.as_ref().map(|p| p.as_path())
    }
}

impl ProjectDirs {
    pub fn project_name(&self) -> &str {
        self.project_name.as_str()
    }
    pub fn cache_dir(&self) -> &Path {
        self.cache_dir.as_path()
    }
    pub fn config_dir(&self) -> &Path {
        self.config_dir.as_path()
    }
    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }
    pub fn data_local_dir(&self) -> &Path {
        self.data_local_dir.as_path()
    }
    pub fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_ref().map(|p| p.as_path())
    }
}

fn strip_qualification(name: &str) -> &str {
    name.rfind('.').map(|start| &name[start+1..]).unwrap_or(name)
}

#[cfg(test)]
mod tests {
    use strip_qualification;

    #[test]
    fn test_base_dirs() {
        println!("{:?}", ::base_dirs());
    }

    #[test]
    fn test_project_dirs() {
        let proj_dirs = ::ProjectDirs::from_unprocessed_string("foobar");
        println!("{:?}", proj_dirs);
    }

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
