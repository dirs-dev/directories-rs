use std::path::{Path, PathBuf};
use anyhow::Result;

/// A trait that implements helper functions for the BaseDirs struct.
pub trait BaseDirsExt {
    /// Given a relative path `path`, returns an absolute path to a configuration
    /// directory in `XDG_CONFIG_HOME`. The directory and all directories
    /// leading to it are created if they did not exist;
    /// if that is not possible, an error is returned.
    fn create_config_directory<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf>;
}