use std::path::{PathBuf, Path};
use anyhow::Result;

/// A trait that implements helper functions for the BaseDirs struct.
pub trait BaseDirsExt {
    /// Given a relative path `path`, returns an absolute path to a configuration
    /// directory in `XDG_CONFIG_HOME`. The directory and all directories
    /// leading to it are created if they did not exist;
    /// if that is not possible, an error is returned.
    fn create_config_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;
}

/// Creates a directory.
pub fn create_directory<P: AsRef<Path>>(home: &PathBuf, path: P) -> Result<PathBuf> {
    let full_path = home.join(path.as_ref());
    std::fs::create_dir_all(&full_path)?;
    Ok(full_path)
}

/// A trait that implements helper functions for the ProjectDirs struct.
pub trait ProjectDirsExt {
    /// Given a relative path `path`, returns an absolute path to an existing
    /// configuration file, or `None`. Searches `XDG_CONFIG_HOME` and then
    /// `XDG_CONFIG_DIRS`.
    fn find_config_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Given a relative path `path`, returns an absolute path in
    /// `XDG_CONFIG_HOME` where a configuration file may be stored.
    /// Leading directories in the returned path are pre-created;
    /// if that is not possible, an error is returned.
    fn place_config_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Given a relative path `path`, returns an absolute path to an existing
    /// data file, or `None`. Searches `XDG_DATA_HOME` and then
    /// `XDG_DATA_DIRS`.
    fn find_data_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;
}

/// Reads a file from a directory.
pub fn read_file(
    home: &PathBuf, 
    dirs: &Vec<PathBuf>, 
    path: &Path
) -> Option<PathBuf> {
    let full_path = home.join(path);
    if path_exists(&full_path) {
        return Some(full_path)
    }
    for dir in dirs.iter() {
        let full_path = dir.join(path);
        if path_exists(&full_path) {
            return Some(full_path)
        }
    }
    None
}

fn path_exists<P: ?Sized + AsRef<Path>>(path: &P) -> bool {
    fn inner(path: &Path) -> bool {
        std::fs::metadata(path).is_ok()
    }
    inner(path.as_ref())
}

/// Writes a file to a location.
pub fn write_file<P: AsRef<Path>>(home: &PathBuf, path: P) -> Result<PathBuf> {
    std::fs::create_dir_all(home.join(&path))?;
    Ok(PathBuf::from(home.join(path.as_ref())))
}