use std::env;

use BaseDirectories;
use ProjectDirectories;

#[cfg(target_os = "macos")]
pub fn base_directories() -> BaseDirectories {
    let home_dir         = env::home_dir().unwrap();
    let cache_dir        = home_dir.join("Library/Caches");
    let config_dir       = home_dir.join("Library/Preferences");
    let data_dir         = home_dir.join("Library/Application Support");
    let data_roaming_dir = data_dir.clone();
    let audio_dir        = home_dir.join("Music");
    let desktop_dir      = home_dir.join("Desktop");
    let document_dir     = home_dir.join("Documents");
    let download_dir     = home_dir.join("Downloads");
    let picture_dir      = home_dir.join("Pictures");
    let public_dir       = home_dir.join("Public");
    let video_dir        = home_dir.join("Movies");
    let executable_dir   = home_dir.join("Applications");
    let font_dir         = home_dir.join("Library/Fonts");

    BaseDirectories {
        home_dir:         home_dir,
        cache_dir:        cache_dir,
        config_dir:       config_dir,
        data_dir:         data_dir,
        data_roaming_dir: data_roaming_dir,
        executable_dir:   Some(executable_dir),
        runtime_dir:      None,

        audio_dir:        audio_dir,
        desktop_dir:      desktop_dir,
        document_dir:     document_dir,
        download_dir:     download_dir,
        font_dir:         Some(font_dir),
        picture_dir:      picture_dir,
        public_dir:       public_dir,
        template_dir:     None,
        video_dir:        video_dir
    }
}

impl ProjectDirectories {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirectories {
        let project_name             = String::from(value);
        let home_dir                 = env::home_dir().unwrap();
        let project_cache_dir        = home_dir.join("Library/Caches").join(&value);
        let project_config_dir       = home_dir.join("Library/Preferences").join(&value);
        let project_data_dir         = home_dir.join("Library/Application Support").join(&value);
        let project_roaming_data_dir = project_data_dir.clone();

        ProjectDirectories {
            project_name:             project_name,
            project_cache_dir:        project_cache_dir,
            project_config_dir:       project_config_dir,
            project_data_dir:         project_data_dir,
            project_data_roaming_dir: project_roaming_data_dir,
            project_runtime_dir:      None,
        }
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirectories {
        ProjectDirectories::from_unprocessed_string(project_name)
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirectories {
        ProjectDirectories::from_unprocessed_string(qualified_project_name)
    }
}
