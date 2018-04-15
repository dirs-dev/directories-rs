#![cfg(target_os = "macos")]

use std::env;
use std::path::PathBuf;

use BaseDirs;
use UserDirs;
use ProjectDirs;

impl BaseDirs {
    /// Creates a `BaseDirs` struct which holds the paths to user-invisible directories for cache, config, etc. data on the system.
    /// The returned struct is a snapshot of the state of the system at the time `new()` was invoked.
    ///
    /// # Panics
    ///
    /// Panics if the home directory cannot be determined. See [`home_dir`].
    ///
    /// [`home_dir`]: #method.home_dir
    pub fn new() -> BaseDirs {
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
}

impl UserDirs {
    /// Creates a `UserDirs` struct which holds the paths to user-facing directories for audio, font, video, etc. data on the system.
    /// The returned struct is a snapshot of the state of the system at the time `new()` was invoked.
    ///
    /// # Panics
    ///
    /// Panics if the home directory cannot be determined. See [`home_dir`].
    ///
    /// [`home_dir`]: #method.home_dir
    pub fn new() -> UserDirs {
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
}

impl ProjectDirs {
    /// Creates a `ProjectDirs` struct directly from a `PathBuf` value.
    /// The argument is used verbatim and is not adapted to operating system standards.
    /// 
    /// The use of `ProjectDirs::from_path` is strongly discouraged, as its results will
    /// not follow operating system standards on at least two of three platforms.
    ///
    /// # Panics
    ///
    /// Panics if the home directory cannot be determined. See [`BaseDirs::home_dir`].
    ///
    /// [`BaseDirs::home_dir`]: struct.BaseDirs.html#method.home_dir
    pub fn from_path(project_path: PathBuf) -> ProjectDirs {
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

    /// Creates a `ProjectDirs` struct from values describing the project.
    ///
    /// The use of `ProjectDirs::from` (instead of `ProjectDirs::from_path`) is strongly encouraged,
    /// as its results will follow operating system standards on Linux, macOS and Windows.
    ///
    /// # Parameters
    ///
    /// - `qualifier`    – The reverse domain name notation of the application, excluding the organization or application name itself.<br/>
    ///   An empty string can be passed if no qualifier should be used (only affects macOS).<br/>
    ///   Example values: `"com.example"`, `"org"`, `"uk.co"`, `"io"`, `""`
    /// - `organization` – The name of the organization that develops this application, or for which the application is developed.<br/>
    ///   An empty string can be passed if no organization should be used (only affects macOS and Windows).<br/>
    ///   Example values: `"Foo Corp"`, `"Alice and Bob Inc"`, `""`
    /// - `application`  – The name of the application itself.<br/>
    ///   Example values: `"Bar App"`, `"ExampleProgram"`, `"Unicorn-Programme"`
    ///
    /// # Panics
    ///
    /// Panics if the home directory cannot be determined. See [`BaseDirs::home_dir`].
    ///
    /// [`BaseDirs::home_dir`]: struct.BaseDirs.html#method.home_dir
    #[allow(unused_variables)]
    pub fn from(qualifier: &str, organization: &str, project: &str) -> ProjectDirs {
        // we should replace more characters, according to RFC1034 identifier rules
        let organization = organization.replace(" ", "-");
        let project      = project.replace(" ", "-");
        let mut parts    = vec![qualifier, &organization, &project]; parts.retain(|e| !e.is_empty());
        let bundle_id    = parts.join(".");
        ProjectDirs::from_path(PathBuf::from(bundle_id))
    }
}
