use std;
use std::path::PathBuf;

extern crate winapi;
use self::winapi::um::knownfolders;
use self::winapi::um::combaseapi;
use self::winapi::um::shlobj;
use self::winapi::um::shtypes;
use self::winapi::um::winnt;

use BaseDirs;
use ProjectDirs;
use strip_qualification;

pub fn base_dirs() -> BaseDirs {
    let home       = unsafe { known_folder(&knownfolders::FOLDERID_Profile) };
    let data       = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) };
    let data_local = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) };
    let desktop    = unsafe { known_folder(&knownfolders::FOLDERID_Desktop) };
    let document   = unsafe { known_folder(&knownfolders::FOLDERID_Documents) };
    let download   = unsafe { known_folder(&knownfolders::FOLDERID_Downloads) };
    let audio      = unsafe { known_folder(&knownfolders::FOLDERID_Music) };
    let picture    = unsafe { known_folder(&knownfolders::FOLDERID_Pictures) };
    let public     = unsafe { known_folder(&knownfolders::FOLDERID_Public) };
    let template   = unsafe { known_folder(&knownfolders::FOLDERID_Templates) };
    let video      = unsafe { known_folder(&knownfolders::FOLDERID_Videos) };

    let cache      = data_local.join("cache");
    let config     = data.clone();

    BaseDirs {
        home:       home,
        cache:      cache,
        config:     config,
        data:       data,
        data_local: data_local,
        executable: None,
        runtime:    None,

        audio:      audio,
        desktop:    desktop,
        document:   document,
        download:   download,
        font:       None,
        picture:    picture,
        public:     public,
        template:   Some(template),
        video:      video
    }
}

impl ProjectDirs {
    pub fn from_unprocessed_string(value: &str) -> ProjectDirs {
        let project_name = String::from(value);
        let data_local   = unsafe { known_folder(&knownfolders::FOLDERID_LocalAppData) };

        let cache        = data_local.join(&value).join("cache");
        let data_local   = data_local.join(&value);
        let data         = unsafe { known_folder(&knownfolders::FOLDERID_RoamingAppData) }.join(&value);

        let config       = data.clone();

        ProjectDirs {
            project_name: project_name,
            cache:        cache,
            config:       config,
            data:         data,
            data_local:   data_local,
            runtime:      None
        }
    }

    pub fn from_project_name(project_name: &str) -> ProjectDirs {
        ProjectDirs::from_unprocessed_string(project_name)
    }

    pub fn from_qualified_project_name(qualified_project_name: &str) -> ProjectDirs {
        let name = strip_qualification(qualified_project_name).trim();
        ProjectDirs::from_unprocessed_string(name)
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