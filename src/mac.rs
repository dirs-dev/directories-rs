use std::env;
use std::path::PathBuf;

use BaseDirs;
use UserDirs;
use ProjectDirs;

pub fn base_dirs() -> BaseDirs {
    let home_dir       = env::home_dir().unwrap();
    let cache_dir      = home_dir.join("Library/Caches");
    let config_dir     = home_dir.join("Library/Preferences");
    let data_dir       = home_dir.join("Library/Application Support");
    let data_local_dir = data_dir.clone();

    BaseDirs {
        home_dir:       home_dir,
        cache_dir:      cache_dir,
        config_dir:     config_dir,
        data_dir:       data_dir,
        data_local_dir: data_local_dir,
        executable_dir: None,
        runtime_dir:    None
    }
}

pub fn user_dirs() -> UserDirs {
    let home_dir       = env::home_dir().unwrap();
    let audio_dir      = home_dir.join("Music");
    let desktop_dir    = home_dir.join("Desktop");
    let document_dir   = home_dir.join("Documents");
    let download_dir   = home_dir.join("Downloads");
    let picture_dir    = home_dir.join("Pictures");
    let public_dir     = home_dir.join("Public");
    // let trash_dir      = home_dir.join(".trash");
    let video_dir      = home_dir.join("Movies");
    let font_dir       = home_dir.join("Library/Fonts");

    UserDirs {
        home_dir:     home_dir,
        audio_dir:    Some(audio_dir),
        desktop_dir:  Some(desktop_dir),
        document_dir: Some(document_dir),
        download_dir: Some(download_dir),
        font_dir:     Some(font_dir),
        picture_dir:  Some(picture_dir),
        public_dir:   Some(public_dir),
        template_dir: None,
        // trash_dir:    trash_dir,
        video_dir:    Some(video_dir)
    }
}

pub fn project_dirs_from_path(project_path: PathBuf) -> ProjectDirs {
    let home_dir       = env::home_dir().unwrap();
    let cache_dir      = home_dir.join("Library/Caches").join(&project_path);
    let config_dir     = home_dir.join("Library/Preferences").join(&project_path);
    let data_dir       = home_dir.join("Library/Application Support").join(&project_path);
    let data_local_dir = data_dir.clone();

    ProjectDirs {
        project_path:   project_path,
        cache_dir:      cache_dir,
        config_dir:     config_dir,
        data_dir:       data_dir,
        data_local_dir: data_local_dir,
        runtime_dir:    None,
    }
}

pub fn project_dirs_from(qualifier: &str, organization: &str, application: &str) -> ProjectDirs {
    // we should replace more characters, according to RFC1034 identifier rules
    let organization = organization.replace(" ", "-");
    let application  = application.replace(" ", "-");
    let mut parts    = vec![qualifier, &organization, &application]; parts.retain(|e| !e.is_empty());
    let bundle_id    = parts.join(".");
    ProjectDirs::from_path(PathBuf::from(bundle_id))
}
