use std::env;
use std::path::PathBuf;
use std::process::Command;

use BaseDirectories;
use ProjectDirectories;

pub fn base_directories() -> BaseDirectories {
    let home_dir         = env::home_dir().unwrap();
    let cache_dir        = env::var("XDG_CACHE_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".cache"));
    let config_dir       = env::var("XDG_CONFIG_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".config"));
    let data_dir         = env::var("XDG_DATA_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".local/share"));
    let data_roaming_dir = data_dir.clone();
    let runtime_dir      = env::var("XDG_RUNTIME_DIR").ok().and_then(is_absolute_path);
    let executables_dir  = { let mut new_dir = data_dir.clone(); new_dir.pop(); new_dir.push("bin"); new_dir };
    let fonts_dir        = data_dir.join("fonts");

    BaseDirectories {
        home_dir:         home_dir,
        cache_dir:        cache_dir,
        config_dir:       config_dir,
        data_dir:         data_dir,
        data_roaming_dir: data_roaming_dir,
        runtime_dir:      runtime_dir,
        desktop_dir:      run_xdg_user_dir_command("DESKTOP"),
        documents_dir:    run_xdg_user_dir_command("DOCUMENTS"),
        download_dir:     run_xdg_user_dir_command("DOWNLOAD"),
        music_dir:        run_xdg_user_dir_command("MUSIC"),
        pictures_dir:     run_xdg_user_dir_command("PICTURES"),
        public_dir:       run_xdg_user_dir_command("PUBLICSHARE"),
        templates_dir:    Some(run_xdg_user_dir_command("TEMPLATES")),
        videos_dir:       run_xdg_user_dir_command("VIDEOS"),
        executables_dir:  Some(executables_dir),
        fonts_dir:        Some(fonts_dir)
    }
}

impl ProjectDirectories {
    pub fn from_unprocessed_string(value: String) -> ProjectDirectories {
        let home_dir                 = env::home_dir().unwrap();
        let project_cache_dir        = env::var("XDG_CACHE_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".cache")).join(&value);
        let project_config_dir       = env::var("XDG_CONFIG_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".config")).join(&value);
        let project_data_dir         = env::var("XDG_DATA_HOME").ok().and_then(is_absolute_path).unwrap_or(home_dir.join(".local/share")).join(&value);
        let project_roaming_data_dir = project_data_dir.clone();

        ProjectDirectories {
            project_name:             value,
            project_cache_dir:        project_cache_dir,
            project_config_dir:       project_config_dir,
            project_data_dir:         project_data_dir,
            project_data_roaming_dir: project_roaming_data_dir,
            project_runtime_dir:      None,
        }
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
    let out = Command::new("xdg-user-dir").arg(arg).output().expect("failed to execute process");
    PathBuf::from(String::from_utf8(out.stdout).unwrap())
}
