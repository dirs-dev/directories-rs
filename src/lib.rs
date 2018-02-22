use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "linux")]   mod lin;
#[cfg(target_os = "windows")] mod win;
#[cfg(target_os = "macos")]   mod mac;

#[cfg(target_os = "linux")]   pub use lin::base_directories as base_directories;
#[cfg(target_os = "windows")] pub use win::base_directories as base_directories;
#[cfg(target_os = "macos")]   pub use mac::base_directories as base_directories;

#[derive(Debug, Clone)]
pub struct BaseDirectories {
    // home directory
    home_dir:         PathBuf,

    // base directories
    cache_dir:        PathBuf,
    config_dir:       PathBuf,
    data_dir:         PathBuf,
    data_roaming_dir: PathBuf,
    runtime_dir:      Option<PathBuf>,

    // user directories
    desktop_dir:      PathBuf,
    documents_dir:    PathBuf,
    downloads_dir:     PathBuf,
    music_dir:        PathBuf,
    pictures_dir:     PathBuf,
    public_dir:       PathBuf,
    templates_dir:    Option<PathBuf>,
    videos_dir:       PathBuf,

    // derived directories
    executables_dir:  Option<PathBuf>,
    fonts_dir:        Option<PathBuf>
}

#[derive(Debug, Clone)]
pub struct ProjectDirectories {
    project_name:             String,
    
    // base directories
    project_cache_dir:        PathBuf,
    project_config_dir:       PathBuf,
    project_data_dir:         PathBuf,
    project_data_roaming_dir: PathBuf,
    project_runtime_dir:      Option<PathBuf>,
}

impl BaseDirectories {
    pub fn home_dir(&self) -> &Path {
        self.home_dir.as_path()
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
    pub fn data_roaming_dir(&self) -> &Path {
        self.data_roaming_dir.as_path()
    }
    pub fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_ref().map(|p| p.as_path())
    }
    pub fn desktop_dir(&self) -> &Path {
        self.desktop_dir.as_path()
    }
    pub fn documents_dir(&self) -> &Path {
        self.documents_dir.as_path()
    }
    pub fn downloads_dir(&self) -> &Path {
        self.downloads_dir.as_path()
    }
    pub fn music_dir(&self) -> &Path {
        self.music_dir.as_path()
    }
    pub fn pictures_dir(&self) -> &Path {
        self.pictures_dir.as_path()
    }
    pub fn public_dir(&self) -> &Path {
        self.public_dir.as_path()
    }
    pub fn templates_dir(&self) -> Option<&Path> {
        self.templates_dir.as_ref().map(|p| p.as_path())
    }
    pub fn videos_dir(&self) -> &Path {
        self.videos_dir.as_path()
    }
    pub fn executables_dir(&self) -> Option<&Path> {
        self.executables_dir.as_ref().map(|p| p.as_path())
    }
    pub fn fonts_dir(&self) -> Option<&Path> {
        self.fonts_dir.as_ref().map(|p| p.as_path())
    }
}

impl ProjectDirectories {
    pub fn project_name(&self) -> &str {
        self.project_name.as_str()
    }
    pub fn project_cache_dir(&self) -> &Path {
        self.project_cache_dir.as_path()
    }
    pub fn project_config_dir(&self) -> &Path {
        self.project_config_dir.as_path()
    }
    pub fn project_data_dir(&self) -> &Path {
        self.project_data_dir.as_path()
    }
    pub fn project_data_roaming_dir(&self) -> &Path {
        self.project_data_roaming_dir.as_path()
    }
    pub fn project_runtime_dir(&self) -> Option<&Path> {
        self.project_runtime_dir.as_ref().map(|p| p.as_path())
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