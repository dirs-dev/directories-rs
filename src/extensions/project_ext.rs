use std::path::{Path, PathBuf};
use anyhow::Result;

/// A trait that implements helper functions for the ProjectDirs struct.
pub trait ProjectDirsExt {
    /// Given a relative path `path`, returns an absolute path to an existing
    /// configuration file, or `None`. Searches `XDG_CONFIG_HOME` and then
    /// `XDG_CONFIG_DIRS`.
    fn find_config_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Given a relative path `path`, returns an absolute path to an existing
    /// data file, or `None`. Searches `XDG_DATA_HOME` and then
    /// `XDG_DATA_DIRS`.
    fn find_data_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Given a relative path `path`, returns an absolute path to an existing
    /// cache file, or `None`. Searches `XDG_CACHE_HOME`.
    fn find_cache_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Given a relative path `path`, returns an absolute path to an existing
    /// application state file, or `None`. Searches `XDG_STATE_HOME`.
    fn find_state_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Given a relative path `path`, returns an absolute path to an existing
    /// runtime file, or `None`. Searches `XDG_RUNTIME_DIR`.
    /// If `XDG_RUNTIME_DIR` is not available, returns `None`.
    fn find_runtime_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;


    /// Like [`place_config_file()`](#method.place_config_file), but does
    /// not create any directories.
    fn get_config_file<P: AsRef<Path>>(&self, path: P) -> PathBuf;

    /// Like [`place_data_file()`](#method.place_data_file), but does
    /// not create any directories.
    fn get_data_file<P: AsRef<Path>>(&self, path: P) -> PathBuf;

    /// Like [`place_cache_file()`](#method.place_cache_file), but does
    /// not create any directories.
    fn get_cache_file<P: AsRef<Path>>(&self, path: P) -> PathBuf;

    /// Like [`place_state_file()`](#method.place_state_file), but does
    /// not create any directories.
    fn get_state_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;

    /// Like [`place_runtime_file()`](#method.place_runtime_file), but does
    /// not create any directories.
    /// If `XDG_RUNTIME_DIR` is not available, returns an error.
    fn get_runtime_file<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf>;


    /// Given a relative path `path`, returns an absolute path in
    /// `XDG_CONFIG_HOME` where a configuration file may be stored.
    /// Leading directories in the returned path are pre-created;
    /// if that is not possible, an error is returned.
    fn place_config_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`place_config_file()`](#method.place_config_file), but for
    /// a data file in `XDG_DATA_HOME`.
    fn place_data_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`place_config_file()`](#method.place_config_file), but for
    /// a cache file in `XDG_CACHE_HOME`.
    fn place_cache_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`place_config_file()`](#method.place_config_file), but for
    /// an application state file in `XDG_STATE_HOME`.
    fn place_state_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`place_config_file()`](#method.place_config_file), but for
    /// a runtime file in `XDG_RUNTIME_DIR`.
    /// If `XDG_RUNTIME_DIR` is not available, returns an error.
    fn place_runtime_file<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;
}
