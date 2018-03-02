use std::env;
use std::path::PathBuf;
use std::process::Command;

use BaseDirs;
use ProjectDirs;
use strip_qualification;

pub fn base_dirs() -> BaseDirs {
    let home       = env::home_dir().unwrap();
    let cache      = env::var("XDG_CACHE_HOME") .ok().and_then(is_absolute_path).unwrap_or(home.join(".cache"));
    let config     = env::var("XDG_CONFIG_HOME").ok().and_then(is_absolute_path).unwrap_or(home.join(".config"));
    let data       = env::var("XDG_DATA_HOME")  .ok().and_then(is_absolute_path).unwrap_or(home.join(".local/share"));
    let data_local = data.clone();
    let runtime    = env::var("XDG_RUNTIME_DIR").ok().and_then(is_absolute_path);
    let executable = 
        env::var("XDG_BIN_HOME").ok().and_then(is_absolute_path).unwrap_or({
            let mut new_dir = data.clone(); new_dir.pop(); new_dir.push("bin"); new_dir });
    let font       = data.join("fonts");

    BaseDirs {
        home:       home,
        cache:      cache,
        config:     config,
        data:       data,
        data_local: data_local,
        executable: Some(executable),
        runtime:    runtime,

        audio:      run_xdg_user_dir_command("MUSIC"),
        desktop:    run_xdg_user_dir_command("DESKTOP"),
        document:   run_xdg_user_dir_command("DOCUMENTS"),
        download:   run_xdg_user_dir_command("DOWNLOAD"),
        font:       Some(font),
        picture:    run_xdg_user_dir_command("PICTURES"),
        public:     run_xdg_user_dir_command("PUBLICSHARE"),
        template:   Some(run_xdg_user_dir_command("TEMPLATES")),
        video:      run_xdg_user_dir_command("VIDEOS")
    }
}

impl ProjectDirs {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirs {
        let project_name = String::from(value);
        let home       = env::home_dir().unwrap();
        let cache      = env::var("XDG_CACHE_HOME") .ok().and_then(is_absolute_path).unwrap_or(home.join(".cache")).join(&value);
        let config     = env::var("XDG_CONFIG_HOME").ok().and_then(is_absolute_path).unwrap_or(home.join(".config")).join(&value);
        let data       = env::var("XDG_DATA_HOME")  .ok().and_then(is_absolute_path).unwrap_or(home.join(".local/share")).join(&value);
        let data_local = data.clone();
        let runtime    = env::var("XDG_RUNTIME_DIR").ok().and_then(is_absolute_path).unwrap().join(&value);

        ProjectDirs {
            project_name: project_name,
            cache:        cache,
            config:       config,
            data:         data,
            data_local:   data_local,
            runtime:      Some(runtime)
        }
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirs {
        let name = trim_and_replace_spaces_with_hyphens_then_lowercase(project_name);
        ProjectDirs::from_unprocessed_string(&name)
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirs {
        let name = strip_qualification(qualified_project_name).to_lowercase();
        ProjectDirs::from_unprocessed_string(name.trim())
    }

}

fn is_absolute_path(path: String) -> Option<PathBuf> {
    let path = PathBuf::from(path);
    if path.is_absolute() {
        Some(path)
    } else {
        None
    }
}

fn run_xdg_user_dir_command(arg: &str) -> PathBuf {
    let mut out  = Command::new("xdg-user-dir").arg(arg).output().expect("failed to execute process").stdout;
    let out_len = out.len();
    out.truncate(out_len - 1);
    PathBuf::from(String::from_utf8(out).unwrap())
}

fn trim_and_replace_spaces_with_hyphens_then_lowercase(name: &str) -> String {
    let mut buf = String::with_capacity(name.len());
    let mut parts = name.split_whitespace();
    let mut current_part = parts.next();
    while current_part.is_some() {
        let value = current_part.unwrap().to_lowercase();
        buf.push_str(&value);
        current_part = parts.next();
        if current_part.is_some() {
            buf.push('-');
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use lin::trim_and_replace_spaces_with_hyphens_then_lowercase;

    #[test]
    fn test_trim_and_replace_spaces_with_hyphens_then_lowercase() {
        let input1    = "Bar App";
        let actual1   = trim_and_replace_spaces_with_hyphens_then_lowercase(input1);
        let expected1 = "bar-app";
        assert_eq!(expected1, actual1);

        let input2    = "BarApp-Foo";
        let actual2   = trim_and_replace_spaces_with_hyphens_then_lowercase(input2);
        let expected2 = "barapp-foo";
        assert_eq!(expected2, actual2);

        let input3    = " Bar App ";
        let actual3   = trim_and_replace_spaces_with_hyphens_then_lowercase(input3);
        let expected3 = "bar-app";
        assert_eq!(expected3, actual3);

        let input4    = "  Bar  App  ";
        let actual4   = trim_and_replace_spaces_with_hyphens_then_lowercase(input4);
        let expected4 = "bar-app";
        assert_eq!(expected4, actual4);
    }
}
