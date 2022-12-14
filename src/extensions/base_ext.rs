use std::path::{Path, PathBuf};
use anyhow::Result;

/// A trait that implements helper functions for the BaseDirs struct.
pub trait BaseDirsExt {
    /// Given a relative path `path`, returns an absolute path to a configuration
    /// directory in `XDG_CONFIG_HOME`. The directory and all directories
    /// leading to it are created if they did not exist;
    /// if that is not possible, an error is returned.
    fn create_config_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`create_config_directory()`](#method.create_config_directory),
    /// but for a data directory in `XDG_DATA_HOME`.
    fn create_data_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`create_config_directory()`](#method.create_config_directory),
    /// but for a cache directory in `XDG_CACHE_HOME`.
    fn create_cache_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`create_config_directory()`](#method.create_config_directory),
    /// but for an application state directory in `XDG_STATE_HOME`.
    fn create_state_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;

    /// Like [`create_config_directory()`](#method.create_config_directory),
    /// but for a runtime directory in `XDG_RUNTIME_DIR`.
    /// If `XDG_RUNTIME_DIR` is not available, returns an error.
    fn create_runtime_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;
}