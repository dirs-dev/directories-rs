use std::path::PathBuf;
use ProjectDirs;

#[cfg(target_os = "windows")]
pub fn project_dirs_from(_qualifier: &str, organization: &str, application: &str) -> Option<ProjectDirs> {
    ProjectDirs::from_path(PathBuf::new().join(organization).join(application))
}

#[cfg(target_os = "macos")]
pub fn project_dirs_from(qualifier: &str, organization: &str, application: &str) -> Option<ProjectDirs> {
    // we should replace more characters, according to RFC1034 identifier rules
    let organization = organization.replace(" ", "-");
    let application  = application.replace(" ", "-");
    let mut parts    = vec![qualifier, organization, application]; parts.retain(|e| !e.is_empty());
    let bundle_id    = parts.join(".");
    ProjectDirs::from_path(PathBuf::from(bundle_id))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn project_dirs_from(_qualifier: &str, _organization: &str, application: &str) -> Option<ProjectDirs> {
    ProjectDirs::from_path(PathBuf::from(&trim_and_lowercase_then_replace_spaces(application, "")))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn trim_and_lowercase_then_replace_spaces(name: &str, replacement: &str) -> String {
    let mut buf = String::with_capacity(name.len());
    let mut parts = name.split_whitespace();
    let mut current_part = parts.next();
    let replace = !replacement.is_empty();
    while current_part.is_some() {
        let value = current_part.unwrap().to_lowercase();
        buf.push_str(&value);
        current_part = parts.next();
        if replace && current_part.is_some() {
            buf.push_str(replacement);
        }
    }
    buf
}

#[cfg(test)]
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod tests {
    use super::trim_and_lowercase_then_replace_spaces;

    #[test]
    fn test_trim_and_lowercase_then_replace_spaces() {
        let input1    = "Bar App";
        let actual1   = trim_and_lowercase_then_replace_spaces(input1, "-");
        let expected1 = "bar-app";
        assert_eq!(expected1, actual1);

        let input2    = "BarApp-Foo";
        let actual2   = trim_and_lowercase_then_replace_spaces(input2, "-");
        let expected2 = "barapp-foo";
        assert_eq!(expected2, actual2);

        let input3    = " Bar App ";
        let actual3   = trim_and_lowercase_then_replace_spaces(input3, "-");
        let expected3 = "bar-app";
        assert_eq!(expected3, actual3);

        let input4    = "  Bar  App  ";
        let actual4   = trim_and_lowercase_then_replace_spaces(input4, "-");
        let expected4 = "bar-app";
        assert_eq!(expected4, actual4);
    }

    #[test]
    fn test_file_user_dirs_exists() {
        let base_dirs      = ::BaseDirs::new();
        let user_dirs_file = base_dirs.unwrap().config_dir().join("user-dirs.dirs");
        println!("{:?} exists: {:?}", user_dirs_file, user_dirs_file.exists());
    }
}
