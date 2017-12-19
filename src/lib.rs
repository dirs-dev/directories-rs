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
    download_dir:     PathBuf,
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