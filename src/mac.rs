use std::env;

use BaseDirs;
use ProjectDirs;

#[cfg(target_os = "macos")]
pub fn base_dirs() -> BaseDirs {
    let home       = env::home_dir().unwrap();
    let cache      = home.join("Library/Caches");
    let config     = home.join("Library/Preferences");
    let data       = home.join("Library/Application Support");
    let data_local = data.clone();
    let audio      = home.join("Music");
    let desktop    = home.join("Desktop");
    let document   = home.join("Documents");
    let download   = home.join("Downloads");
    let picture    = home.join("Pictures");
    let public     = home.join("Public");
    let video      = home.join("Movies");
    let font       = home.join("Library/Fonts");

    BaseDirs {
        home:         home,
        cache:        cache,
        config:       config,
        data:         data,
        data_local:   data_local,
        executable:   None,
        runtime:      None,

        audio:        audio,
        desktop:      desktop,
        document:     document,
        download:     download,
        font:         Some(font),
        picture:      picture,
        public:       public,
        template:     None,
        video:        video
    }
}

impl ProjectDirs {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirs {
        let project_name = String::from(value);
        let home         = env::home_dir().unwrap();
        let cache        = home.join("Library/Caches").join(&value);
        let config       = home.join("Library/Preferences").join(&value);
        let data         = home.join("Library/Application Support").join(&value);
        let data_local   = data.clone();

        ProjectDirs {
            project_name: project_name,
            cache:        cache,
            config:       config,
            data:         data,
            data_local:   data_local,
            runtime:      None,
        }
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirs {
        ProjectDirs::from_unprocessed_string(project_name)
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirs {
        ProjectDirs::from_unprocessed_string(qualified_project_name)
    }
}
