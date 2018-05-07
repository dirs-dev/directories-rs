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

pub fn base_dirs() -> BaseDirs {
    let home_dir       = known_folder(&knownfolders::FOLDERID_Profile);
    let data_dir       = known_folder(&knownfolders::FOLDERID_RoamingAppData);
    let data_local_dir = known_folder(&knownfolders::FOLDERID_LocalAppData);
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

pub fn user_dirs() -> UserDirs {
    let home_dir       = known_folder(&knownfolders::FOLDERID_Profile);
    let audio_dir      = known_folder(&knownfolders::FOLDERID_Music);
    let desktop_dir    = known_folder(&knownfolders::FOLDERID_Desktop);
    let document_dir   = known_folder(&knownfolders::FOLDERID_Documents);
    let download_dir   = known_folder(&knownfolders::FOLDERID_Downloads);
    let picture_dir    = known_folder(&knownfolders::FOLDERID_Pictures);
    let public_dir     = known_folder(&knownfolders::FOLDERID_Public);
    let template_dir   = known_folder(&knownfolders::FOLDERID_Templates);
    let video_dir      = known_folder(&knownfolders::FOLDERID_Videos);

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
        video_dir:    Some(video_dir)
    }
}

pub fn project_dirs_from_path(project_path: PathBuf) -> ProjectDirs {
    let app_data_local   = known_folder(&knownfolders::FOLDERID_LocalAppData).join(&project_path);
    let app_data_roaming = known_folder(&knownfolders::FOLDERID_RoamingAppData).join(&project_path);
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

pub fn project_dirs_from(_qualifier: &str, organization: &str, application: &str) -> ProjectDirs {
    ProjectDirs::from_path(PathBuf::from_iter(&[organization, application]))
}

fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> PathBuf {
    unsafe {
        let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
        let result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
        if result != winerror::S_OK {
            panic!("SHGetKnownFolderPath failed with {}", result);
        }
        let len = winbase::lstrlenW(path_ptr) as usize;
        let path = std::slice::from_raw_parts(path_ptr, len);
        let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
        combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
        PathBuf::from(ostr)
    }
}
