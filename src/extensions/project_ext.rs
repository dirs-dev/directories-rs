use std::path::{Path, PathBuf};
use anyhow::Result;

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
