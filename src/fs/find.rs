use std::fs::{read_dir, DirEntry, ReadDir};
use std::io::Error;
use std::os::unix::fs::PermissionsExt;

pub fn find_file(name: &str, paths: &Vec<&str>, only_executable: bool) -> Option<String> {
    // errors remains here
    // because we need to go down the list
    for path in paths {
        if let Ok(dir) = read_dir(path) {
            if let Some(found_path) = find_file_in_dir(name, dir, only_executable) {
                return Some(found_path);
            }
        }
    }

    None
}

pub fn find_files(
    starts_with: &str,
    paths: &Vec<&str>,
    only_executable: bool,
) -> Option<Vec<String>> {
    // errors remains here
    // because we need to go down the list
    let mut found_paths = Vec::with_capacity(10);

    for path in paths {
        if let Ok(dir) = read_dir(path) {
            if let Some(mut r) = find_files_in_dir(starts_with, dir, only_executable) {
                found_paths.append(&mut r);
            }
        }
    }

    if found_paths.is_empty() {
        None
    } else {
        Some(found_paths)
    }
}

fn find_file_in_dir(name: &str, dir: ReadDir, only_executable: bool) -> Option<String> {
    // errors remains here
    // because we need to go down the list
    for entry in dir.flatten() {
        let current_name = get_name(&entry);

        if current_name.is_none() {
            continue;
        }

        if name != current_name.unwrap() {
            continue;
        }

        if is_file(&entry, only_executable).is_ok_and(|s| s) {
            if let Some(path) = get_path(&entry) {
                return Some(path);
            }
        }
    }

    None
}

fn find_files_in_dir(
    starts_with: &str,
    dir: ReadDir,
    only_executable: bool,
) -> Option<Vec<String>> {
    // errors remains here
    // because we need to go down the list
    let mut paths = Vec::with_capacity(10);

    for entry in dir.flatten() {
        let name = get_name(&entry);

        if name.is_none() {
            continue;
        }

        if !name.unwrap().starts_with(starts_with) {
            continue;
        }

        if is_file(&entry, only_executable).is_ok_and(|s| s) {
            if let Some(path) = get_path(&entry) {
                paths.push(path);
            }
        }
    }

    if paths.is_empty() {
        None
    } else {
        Some(paths)
    }
}

fn get_name(entry: &DirEntry) -> Option<String> {
    entry.file_name().into_string().ok()
}

fn get_path(entry: &DirEntry) -> Option<String> {
    entry.path().to_str().map(|r| Some(r.to_string()))?
}

fn is_file(entry: &DirEntry, is_executable: bool) -> Result<bool, Error> {
    let metadata = entry.metadata()?;

    if metadata.is_dir() {
        return Ok(false);
    }

    if is_executable {
        Ok(metadata.permissions().mode() & 0o111 != 0) // windows no
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::get_current_dir;

    #[test]
    fn test_find_file() {
        let r = get_fixture_dir();
        let paths = vec![r.as_str()];
        let only_executable = true;

        let r = find_file("exe", &paths, only_executable).unwrap();
        assert_eq!(format!("{}exe", get_fixture_dir()), r);

        let r = find_file("not_exe", &paths, only_executable);
        assert!(r.is_none());

        let r = find_file("not_exists", &paths, only_executable);
        assert!(r.is_none());
    }

    #[test]
    fn test_find_files() {
        let r = get_fixture_dir();
        let paths = vec![r.as_str()];
        let only_executable = true;

        let r = find_files("ex", &paths, only_executable).unwrap();
        let f = vec![format!("{}exe", get_fixture_dir())];
        assert_eq!(f, r);

        let r = find_files("not", &paths, only_executable);
        assert!(r.is_none());
    }

    fn get_fixture_dir() -> String {
        // ends with a slash
        format!("{}/test/fixture/fs/", get_current_dir())
    }
}
