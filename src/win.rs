#![cfg(target_os = "windows")]

use std;
use std::path::PathBuf;
use std::iter::FromIterator;

extern crate winapi;
use self::winapi::um::knownfolders;
use self::winapi::um::combaseapi;
use self::winapi::um::shlobj;
use self::winapi::um::shtypes;
use self::winapi::um::winnt;

use BaseDirs;
use UserDirs;
use ProjectDirs;

impl BaseDirs {
    /// Creates a `BaseDirs` struct which holds the paths to user-invisible directories for cache, config, etc. data on the system.
    /// The returned struct is a snapshot of the state of the system at the time `new()` was invoked.
    pub fn new() -> BaseDirs {
        let home_dir       = unsafe { known_folder(&knownfolders::FOLDERID_Profile) };
        let data_dir       = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) };
        let data_local_dir = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) };
        let cache_dir      = data_local_dir.clone();
        let config_dir     = data_dir.clone();

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
    pub fn new() -> UserDirs {
        let home_dir       = unsafe { known_folder(&knownfolders::FOLDERID_Profile) };
        let audio_dir      = unsafe { known_folder(&knownfolders::FOLDERID_Music) };
        let desktop_dir    = unsafe { known_folder(&knownfolders::FOLDERID_Desktop) };
        let document_dir   = unsafe { known_folder(&knownfolders::FOLDERID_Documents) };
        let download_dir   = unsafe { known_folder(&knownfolders::FOLDERID_Downloads) };
        let picture_dir    = unsafe { known_folder(&knownfolders::FOLDERID_Pictures) };
        let public_dir     = unsafe { known_folder(&knownfolders::FOLDERID_Public) };
        let template_dir   = unsafe { known_folder(&knownfolders::FOLDERID_Templates) };
        // see https://github.com/soc/directories-rs/issues/18
        // let trash_dir      = unsafe { known_folder(&knownfolders::FOLDERID_RecycleBinFolder) };
        let video_dir      = unsafe { known_folder(&knownfolders::FOLDERID_Videos) };

        UserDirs {
            home_dir:     home_dir,
            audio_dir:    Some(audio_dir),
            desktop_dir:  Some(desktop_dir),
            document_dir: Some(document_dir),
            download_dir: Some(download_dir),
            font_dir:     None,
            picture_dir:  Some(picture_dir),
            public_dir:   Some(public_dir),
            template_dir: Some(template_dir),
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
    pub fn from_path(project_path: PathBuf) -> ProjectDirs {
        let app_data_local   = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) }.join(&project_path);
        let app_data_roaming = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) }.join(&project_path);
        let cache_dir      = app_data_local.join("cache");
        let data_local_dir = app_data_local.join("data");
        let config_dir     = app_data_roaming.join("config");
        let data_dir       = app_data_roaming.join("data");

        ProjectDirs {
            project_path:   project_path,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            runtime_dir:    None
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
    #[allow(unused_variables)]
    pub fn from(qualifier: &str, organization: &str, project: &str) -> ProjectDirs {
        ProjectDirs::from_path(PathBuf::from_iter(&[organization, project]))
    }
}

unsafe fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> PathBuf {
    let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
    let _result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
    let len = length_of_u16_string(path_ptr);
    let path = std::slice::from_raw_parts(path_ptr, len);
    let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
    combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
    PathBuf::from(ostr)
}

unsafe fn length_of_u16_string(ptr: *mut u16) -> usize {
    let mut index = 0;
    while *ptr.offset(index as isize) != 0 as u16 {
        index += 1;
    }
    index
}
