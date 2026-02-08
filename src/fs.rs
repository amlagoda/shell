use std::fs::{read_dir, DirEntry, File, OpenOptions, ReadDir};
use std::io::Error;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn open_file(path: &str, append: bool) -> Result<File, Error> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(Path::new(path))?;

    Ok(file)
}

pub fn search_executable_file_in_paths(name: &str, paths: &Vec<&str>) -> Option<String> {
    // errors remains here
    // because we need to go down the list
    for path in paths {
        if let Ok(dir) = read_dir(path) {
            if let Some(r) = search_executable_file_in_dir(name, dir) {
                return Some(r);
            }
        }
    }

    None
}

pub fn search_executable_files_in_paths(
    starts_with: &str,
    paths: &Vec<&str>,
) -> Option<Vec<String>> {
    let mut files = vec![];

    for path in paths {
        // errors remains here
        // because we need to go down the list
        if let Ok(dir) = read_dir(path) {
            if let Some(mut r) = search_executable_files_in_dir(starts_with, dir) {
                files.append(&mut r);
            }
        }
    }

    if files.is_empty() {
        None
    } else {
        Some(files)
    }
}

fn search_executable_file_in_dir(name: &str, dir: ReadDir) -> Option<String> {
    // errors remains here
    // because we need to go down the list
    for entry in dir.flatten() {
        if let Some(r) = name_equals_and_executable(name, &entry) {
            return Some(r);
        }
    }

    None
}

fn search_executable_files_in_dir(starts_with: &str, dir: ReadDir) -> Option<Vec<String>> {
    // errors remains here
    // because we need to go down the list
    let mut files = vec![];

    for entry in dir.flatten() {
        if let Some(r) = name_starts_with_and_executable(starts_with, &entry) {
            files.push(r);
        }
    }

    if files.is_empty() {
        None
    } else {
        Some(files)
    }
}

fn name_equals_and_executable(name: &str, entry: &DirEntry) -> Option<String> {
    if !is_executable_file(entry).ok()? {
        return None;
    }

    if name != entry.file_name().into_string().ok()? {
        return None;
    }

    entry.path().to_str().map(|r| Some(r.to_string()))?
}

fn name_starts_with_and_executable(starts_with: &str, entry: &DirEntry) -> Option<String> {
    if !is_executable_file(entry).ok()? {
        return None;
    }

    if !entry
        .file_name()
        .into_string()
        .ok()?
        .starts_with(starts_with)
    {
        return None;
    }

    entry.path().to_str().map(|r| Some(r.to_string()))?
}

fn is_executable_file(entry: &DirEntry) -> Result<bool, Error> {
    let r = entry.metadata()?;

    if r.is_dir() {
        Ok(false)
    } else {
        Ok(r.permissions().mode() & 0o111 != 0) // windows no
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_search_executable_file_in_paths() {
        let r = get_fixture_dir();
        let paths = vec![r.as_str()];

        let r = search_executable_file_in_paths("exe", &paths).unwrap();
        assert_eq!(format!("{}exe", get_fixture_dir()), r);

        let r = search_executable_file_in_paths("not_exe", &paths);
        assert!(r.is_none());

        let r = search_executable_file_in_paths("not_exists", &paths);
        assert!(r.is_none());
    }

    #[test]
    fn test_search_executable_files_in_paths() {
        let r = get_fixture_dir();
        let paths = vec![r.as_str()];

        let r = search_executable_files_in_paths("ex", &paths).unwrap();
        let f = vec![format!("{}exe", get_fixture_dir())];
        assert_eq!(f, r);

        let r = search_executable_files_in_paths("not", &paths);
        assert!(r.is_none());
    }

    fn get_fixture_dir() -> String {
        // ends with a slash
        format!("{}/test/fixture/fs/", get_current_dir())
    }

    fn get_current_dir() -> String {
        // does not end with a slash
        current_dir().unwrap().to_str().unwrap().to_string()
    }
}
