use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

use BaseDirs;
use UserDirs;
use ProjectDirs;

use unix;

pub fn base_dirs() -> Option<BaseDirs> {
    if let Some(home_dir)  = unix::home_dir() {
        let cache_dir      = env::var_os("XDG_CACHE_HOME") .and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".cache"));
        let config_dir     = env::var_os("XDG_CONFIG_HOME").and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".config"));
        let data_dir       = env::var_os("XDG_DATA_HOME")  .and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".local/share"));
        let data_local_dir = data_dir.clone();
        let runtime_dir    = env::var_os("XDG_RUNTIME_DIR").and_then(is_absolute_path);
        let executable_dir = 
            env::var_os("XDG_BIN_HOME").and_then(is_absolute_path).unwrap_or_else(|| {
                let mut new_dir = data_dir.clone(); new_dir.pop(); new_dir.push("bin"); new_dir });

        let base_dirs = BaseDirs {
            home_dir:       home_dir,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            executable_dir: Some(executable_dir),
            runtime_dir:    runtime_dir
        };
        Some(base_dirs)
    } else {
        None
    }
}

pub fn user_dirs() -> Option<UserDirs> {
    if let Some(home_dir) = unix::home_dir() {
        let data_dir  = env::var_os("XDG_DATA_HOME").and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".local/share"));
        let font_dir  = data_dir.join("fonts");

        let user_dirs = UserDirs {
            home_dir:     home_dir,
            audio_dir:    run_xdg_user_dir_command("MUSIC"),
            desktop_dir:  run_xdg_user_dir_command("DESKTOP"),
            document_dir: run_xdg_user_dir_command("DOCUMENTS"),
            download_dir: run_xdg_user_dir_command("DOWNLOAD"),
            font_dir:     Some(font_dir),
            picture_dir:  run_xdg_user_dir_command("PICTURES"),
            public_dir:   run_xdg_user_dir_command("PUBLICSHARE"),
            template_dir: run_xdg_user_dir_command("TEMPLATES"),
            video_dir:    run_xdg_user_dir_command("VIDEOS")
        };
        Some(user_dirs)
    } else {
        None
    }
}

pub fn project_dirs_from_path(project_path: PathBuf) -> Option<ProjectDirs> {
    if let Some(home_dir)  = unix::home_dir() { 
        let cache_dir      = env::var_os("XDG_CACHE_HOME") .and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".cache")).join(&project_path);
        let config_dir     = env::var_os("XDG_CONFIG_HOME").and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".config")).join(&project_path);
        let data_dir       = env::var_os("XDG_DATA_HOME")  .and_then(is_absolute_path).unwrap_or_else(|| home_dir.join(".local/share")).join(&project_path);
        let data_local_dir = data_dir.clone();
        let runtime_dir    = env::var_os("XDG_RUNTIME_DIR").and_then(is_absolute_path).map(|o| o.join(&project_path));

        let project_dirs = ProjectDirs {
            project_path:   project_path,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            runtime_dir:    runtime_dir
        };
        Some(project_dirs)
    } else {
        None
    }
}

pub fn project_dirs_from(_qualifier: &str, _organization: &str, application: &str) -> Option<ProjectDirs> {
    ProjectDirs::from_path(PathBuf::from(&trim_and_lowercase_then_replace_spaces(application, "")))
}

// we don't need to explicitly handle empty strings in the code above,
// because an empty string is not considered to be a absolute path here.
fn is_absolute_path(path: OsString) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

fn run_xdg_user_dir_command(arg: &str) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    let mut out = match Command::new("xdg-user-dir").arg(arg).output() {
        Ok(output) => output.stdout,
        Err(_) => return None,
    };
    let out_len = out.len();
    out.truncate(out_len - 1);
    Some(PathBuf::from(OsString::from_vec(out)))
}

fn trim_and_lowercase_then_replace_spaces(name: &str, replacement: &str) -> String {
    let mut buf = String::with_capacity(name.len());
    let mut parts = name.split_whitespace();
    let mut current_part = parts.next();
    let replace = !replacement.is_empty();
    while current_part.is_some() {
        let value = current_part.unwrap().to_lowercase();
        buf.push_str(&value);
        current_part = parts.next();
        if replace && current_part.is_some() {
            buf.push_str(replacement);
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use lin::trim_and_lowercase_then_replace_spaces;

    #[test]
    fn test_trim_and_lowercase_then_replace_spaces() {
        let input1    = "Bar App";
        let actual1   = trim_and_lowercase_then_replace_spaces(input1, "-");
        let expected1 = "bar-app";
        assert_eq!(expected1, actual1);

        let input2    = "BarApp-Foo";
        let actual2   = trim_and_lowercase_then_replace_spaces(input2, "-");
        let expected2 = "barapp-foo";
        assert_eq!(expected2, actual2);

        let input3    = " Bar App ";
        let actual3   = trim_and_lowercase_then_replace_spaces(input3, "-");
        let expected3 = "bar-app";
        assert_eq!(expected3, actual3);

        let input4    = "  Bar  App  ";
        let actual4   = trim_and_lowercase_then_replace_spaces(input4, "-");
        let expected4 = "bar-app";
        assert_eq!(expected4, actual4);
    }

    #[test]
    fn test_file_user_dirs_exists() {
        let base_dirs      = ::BaseDirs::new();
        let user_dirs_file = base_dirs.unwrap().config_dir().join("user-dirs.dirs");
        println!("{:?} exists: {:?}", user_dirs_file, user_dirs_file.exists());
    }
}
