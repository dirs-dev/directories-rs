use std::{path::{Path, PathBuf}, fs::read_dir};
use anyhow::Result;

/// Creates a directory.
pub fn create_directory<P: AsRef<Path>>(home: &PathBuf, path: P) -> Result<PathBuf> {
    let full_path = home.join(path.as_ref());
    std::fs::create_dir_all(&full_path)?;
    Ok(full_path)
}

/// Reads a file from a directory.
pub fn read_file(
    home: &PathBuf, 
    path: &Path
) -> Option<PathBuf> {
    let full_path = home.join(path);
    if path_exists(&full_path) {
        return Some(full_path)
    }
    let dirs: Vec<PathBuf> = read_dir(home).ok()?
        .into_iter()
        .map(|entry| entry.unwrap())
        .map(|path| path.path())
        .collect();
    for dir in dirs.iter() {
        let full_path = dir.join(path);
        if path_exists(&full_path) {
            return Some(full_path)
        }
    }
    None
}

/// Checks if a path exists.
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