use std;
use std::path::PathBuf;
use std::iter::FromIterator;

extern crate winapi;
use self::winapi::shared::winerror;
use self::winapi::um::knownfolders;
use self::winapi::um::combaseapi;
use self::winapi::um::shlobj;
use self::winapi::um::shtypes;
use self::winapi::um::winbase;
use self::winapi::um::winnt;


use BaseDirs;
use UserDirs;
use ProjectDirs;

pub fn base_dirs() -> Option<BaseDirs> {
    let home_dir       = known_folder(&knownfolders::FOLDERID_Profile);
    let data_dir       = known_folder(&knownfolders::FOLDERID_RoamingAppData);
    let data_local_dir = known_folder(&knownfolders::FOLDERID_LocalAppData);
    if let (Some(home_dir), Some(data_dir), Some(data_local_dir)) = (home_dir, data_dir, data_local_dir) {
        let cache_dir  = data_local_dir.clone();
        let config_dir = data_dir.clone();

        let base_dirs = BaseDirs {
            home_dir:       home_dir,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            executable_dir: None,
            runtime_dir:    None
        };
        Some(base_dirs)
    } else {
        None
    }
}

pub fn user_dirs() -> Option<UserDirs> {
    if let Some(home_dir) = known_folder(&knownfolders::FOLDERID_Profile) {
        let audio_dir     = known_folder(&knownfolders::FOLDERID_Music);
        let desktop_dir   = known_folder(&knownfolders::FOLDERID_Desktop);
        let document_dir  = known_folder(&knownfolders::FOLDERID_Documents);
        let download_dir  = known_folder(&knownfolders::FOLDERID_Downloads);
        let picture_dir   = known_folder(&knownfolders::FOLDERID_Pictures);
        let public_dir    = known_folder(&knownfolders::FOLDERID_Public);
        let template_dir  = known_folder(&knownfolders::FOLDERID_Templates);
        let video_dir     = known_folder(&knownfolders::FOLDERID_Videos);

        let user_dirs = UserDirs {
            home_dir:     home_dir,
            audio_dir:    audio_dir,
            desktop_dir:  desktop_dir,
            document_dir: document_dir,
            download_dir: download_dir,
            font_dir:     None,
            picture_dir:  picture_dir,
            public_dir:   public_dir,
            template_dir: template_dir,
            video_dir:    video_dir
        };
        Some(user_dirs)
    } else {
        None
    }
}

pub fn project_dirs_from_path(project_path: PathBuf) -> Option<ProjectDirs> {
    let app_data_local   = known_folder(&knownfolders::FOLDERID_LocalAppData);
    let app_data_roaming = known_folder(&knownfolders::FOLDERID_RoamingAppData);
    if let (Some(app_data_local), Some(app_data_roaming)) = (app_data_local, app_data_roaming) {
        let app_data_local   = app_data_local.join(&project_path);
        let app_data_roaming = app_data_roaming.join(&project_path);
        let cache_dir        = app_data_local.join("cache");
        let data_local_dir   = app_data_local.join("data");
        let config_dir       = app_data_roaming.join("config");
        let data_dir         = app_data_roaming.join("data");

        let project_dirs = ProjectDirs {
            project_path:   project_path,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            runtime_dir:    None
        };
        Some(project_dirs)
    } else {
        None
    }

}

pub fn project_dirs_from(_qualifier: &str, organization: &str, application: &str) -> Option<ProjectDirs> {
    ProjectDirs::from_path(PathBuf::from_iter(&[organization, application]))
}

fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> Option<PathBuf> {
    unsafe {
        let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
        let result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
        if result == winerror::S_OK {
            let len = winbase::lstrlenW(path_ptr) as usize;
            let path = std::slice::from_raw_parts(path_ptr, len);
            let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
            combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
            Some(PathBuf::from(ostr))
        } else {
            None
        }
    }
}