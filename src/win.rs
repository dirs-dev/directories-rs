use std;
use std::path::PathBuf;

extern crate winapi;
use self::winapi::um::knownfolders;
use self::winapi::um::combaseapi;
use self::winapi::um::shlobj;
use self::winapi::um::shtypes;
use self::winapi::um::winnt;

use BaseDirectories;
use ProjectDirectories;
use strip_qualification;

pub fn base_directories() -> BaseDirectories {
    let home_dir       = unsafe { known_folder(&knownfolders::FOLDERID_Profile) };
    let data_dir       = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) };
    let data_local_dir = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) };
    let desktop_dir    = unsafe { known_folder(&knownfolders::FOLDERID_Desktop) };
    let document_dir   = unsafe { known_folder(&knownfolders::FOLDERID_Documents) };
    let download_dir   = unsafe { known_folder(&knownfolders::FOLDERID_Downloads) };
    let audio_dir      = unsafe { known_folder(&knownfolders::FOLDERID_Music) };
    let picture_dir    = unsafe { known_folder(&knownfolders::FOLDERID_Pictures) };
    let public_dir     = unsafe { known_folder(&knownfolders::FOLDERID_Public) };
    let template_dir   = unsafe { known_folder(&knownfolders::FOLDERID_Templates) };
    let video_dir      = unsafe { known_folder(&knownfolders::FOLDERID_Videos) };

    let cache_dir      = data_local_dir.join("cache");
    let config_dir     = data_dir.clone();

    BaseDirectories {
        home_dir:       home_dir,
        cache_dir:      cache_dir,
        config_dir:     config_dir,
        data_dir:       data_dir,
        data_local_dir: data_local_dir,
        executable_dir: None,
        runtime_dir:    None,

        audio_dir:      audio_dir,
        desktop_dir:    desktop_dir,
        document_dir:   document_dir,
        download_dir:   download_dir,
        font_dir:       None,
        picture_dir:    picture_dir,
        public_dir:     public_dir,
        template_dir:   Some(template_dir),
        video_dir:      video_dir
    }
}

impl ProjectDirectories {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirectories {
        let project_name             = String::from(value);
        let data_local_dir           = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) };

        let project_cache_dir        = data_local_dir.join(&value).join("cache");
        let project_data_local_dir   = data_local_dir.join(&value);
        let project_data_dir = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) }.join(&value);

        let project_config_dir       = project_data_dir.clone();

        ProjectDirectories {
            project_name:           project_name,
            project_cache_dir:      project_cache_dir,
            project_config_dir:     project_config_dir,
            project_data_dir:       project_data_dir,
            project_data_local_dir: project_data_local_dir,
            project_runtime_dir:    None
        }
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirectories {
        ProjectDirectories::from_unprocessed_string(project_name)
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirectories {
        let name = strip_qualification(qualified_project_name).trim();
        ProjectDirectories::from_unprocessed_string(name)
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