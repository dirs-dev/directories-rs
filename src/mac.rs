use std::env;

use BaseDirs;
use ProjectDirs;

#[cfg(target_os = "macos")]
pub fn base_dirs() -> BaseDirs {
    let home_dir       = env::home_dir().unwrap();
    let cache_dir      = home_dir.join("Library/Caches");
    let config_dir     = home_dir.join("Library/Preferences");
    let data_dir       = home_dir.join("Library/Application Support");
    let data_local_dir = data_dir.clone();
    let audio_dir      = home_dir.join("Music");
    let desktop_dir    = home_dir.join("Desktop");
    let document_dir   = home_dir.join("Documents");
    let download_dir   = home_dir.join("Downloads");
    let picture_dir    = home_dir.join("Pictures");
    let public_dir     = home_dir.join("Public");
    let video_dir      = home_dir.join("Movies");
    let font_dir       = home_dir.join("Library/Fonts");

    BaseDirs {
        home_dir:         home_dir,
        cache_dir:        cache_dir,
        config_dir:       config_dir,
        data_dir:         data_dir,
        data_local_dir:   data_local_dir,
        executable_dir:   None,
        runtime_dir:      None,

        audio_dir:        Some(audio_dir),
        desktop_dir:      Some(desktop_dir),
        document_dir:     Some(document_dir),
        download_dir:     Some(download_dir),
        font_dir:         Some(font_dir),
        picture_dir:      Some(picture_dir),
        public_dir:       Some(public_dir),
        template_dir:     None,
        video_dir:        Some(video_dir)
    }
}

impl ProjectDirs {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirs {
        let project_name   = String::from(value);
        let home_dir       = env::home_dir().unwrap();
        let cache_dir      = home_dir.join("Library/Caches").join(&value);
        let config_dir     = home_dir.join("Library/Preferences").join(&value);
        let data_dir       = home_dir.join("Library/Application Support").join(&value);
        let data_local_dir = data_dir.clone();

        ProjectDirs {
            project_name:   project_name,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            runtime_dir:    None,
        }
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirs {
        ProjectDirs::from_unprocessed_string(project_name)
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirs {
        ProjectDirs::from_unprocessed_string(qualified_project_name)
    }
}
