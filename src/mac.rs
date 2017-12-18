use std::env;

use BaseDirectories;

#[cfg(target_os = "macos")]
pub fn base_directories() -> BaseDirectories {
    let home_dir         = env::home_dir().unwrap();
    let cache_dir        = home_dir.join("Library/Caches/");
    let config_dir       = home_dir.join("Library/Preferences/");
    let data_dir         = home_dir.join("Library/Application Support/");
    let data_roaming_dir = data_dir.clone();
    let desktop_dir      = home_dir.join("Desktop");
    let documents_dir    = home_dir.join("Documents");
    let download_dir     = home_dir.join("Downloads");
    let music_dir        = home_dir.join("Music");
    let pictures_dir     = home_dir.join("Pictures");
    let public_dir       = home_dir.join("Public");
    let videos_dir       = home_dir.join("Movies");
    let executables_dir  = home_dir.join("Applications/");
    let fonts_dir        = home_dir.join("Library/Fonts/");

    BaseDirectories {
        home_dir:         home_dir,
        cache_dir:        cache_dir,
        config_dir:       config_dir,
        data_dir:         data_dir,
        data_roaming_dir: data_roaming_dir,
        runtime_dir:      None,
        desktop_dir:      desktop_dir,
        documents_dir:    documents_dir,
        download_dir:     download_dir,
        music_dir:        music_dir,
        pictures_dir:     pictures_dir,
        public_dir:       public_dir,
        templates_dir:    None,
        videos_dir:       videos_dir,
        executables_dir:  Some(executables_dir),
        fonts_dir:        Some(fonts_dir)
    }
}
