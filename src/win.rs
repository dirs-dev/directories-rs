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
use ProjectDirs;

pub fn base_dirs() -> BaseDirs {
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

    BaseDirs {
        home_dir:       home_dir,
        cache_dir:      cache_dir,
        config_dir:     config_dir,
        data_dir:       data_dir,
        data_local_dir: data_local_dir,
        executable_dir: None,
        runtime_dir:    None,

        audio_dir:      Some(audio_dir),
        desktop_dir:    Some(desktop_dir),
        document_dir:   Some(document_dir),
        download_dir:   Some(download_dir),
        font_dir:       None,
        picture_dir:    Some(picture_dir),
        public_dir:     Some(public_dir),
        template_dir:   Some(template_dir),
        video_dir:      Some(video_dir)
    }
}

impl ProjectDirs {
    pub fn from_path(project_path: PathBuf) -> ProjectDirs {
        let data_dir       = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) }.join(&project_path);
        let config_dir     = data_dir.clone();
        let data_local_dir = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) }.join(&project_path);
        let cache_dir      = data_local_dir.join("cache");

        ProjectDirs {
            project_path:   project_path,
            cache_dir:      cache_dir,
            config_dir:     config_dir,
            data_dir:       data_dir,
            data_local_dir: data_local_dir,
            runtime_dir:    None
        }
    }

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
