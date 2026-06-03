mod structure;

use self::structure::FileType;
use crate::rule::Comprasion;
use std::fs::{read_dir, DirEntry};
use std::io::Error;

pub use structure::FindFilesResult;

pub fn find_bins_by_name(name: &str, paths: &Vec<&str>) -> FindFilesResult {
    let rule = Comprasion::Equal(name.to_string());

    find_files(paths, Some(&vec![&FileType::Executable]), Some(&rule), true)
}

pub fn find_bins_starts_with(starts_with: &str, paths: &Vec<&str>) -> FindFilesResult {
    let rule = Comprasion::AssertedStartsWith(starts_with.to_string());

    find_files(paths, Some(&vec![&FileType::Executable]), Some(&rule), true)
}

pub fn find_files_starts_with(starts_with: &str, paths: &Vec<&str>) -> FindFilesResult {
    let rule = Comprasion::AssertedStartsWith(starts_with.to_string());

    let rrule = if starts_with.is_empty() {
        None
    } else {
        Some(&rule)
    };

    find_files(paths, Some(&vec![&FileType::File]), rrule, true)
}

fn get_name(entry: &DirEntry) -> Option<String> {
    entry.file_name().into_string().ok()
}

fn get_path(entry: &DirEntry) -> Option<String> {
    entry.path().to_str().map(|r| Some(r.to_string()))?
}

// ignore_errors
// - search path is not a directory
// - reading file metadata (permissions, etc)
fn find_files(
    search_paths: &Vec<&str>,
    only_types: Option<&Vec<&FileType>>,
    name_rule: Option<&Comprasion>,
    ignore_errors: bool,
) -> FindFilesResult {
    if search_paths.is_empty() {
        let msg = "search paths is empty";
        return FindFilesResult::Err(Error::other(msg));
    }

    let mut found = vec![];

    for path in search_paths {
        let dir = read_dir(path);

        if dir.is_err() {
            if ignore_errors {
                continue;
            }

            let msg = format!("{}: dir is invalid", path);
            return FindFilesResult::Err(Error::other(msg));
        }

        for file in dir.unwrap().flatten() {
            match match_file(&file, name_rule, only_types) {
                FindFilesResult::None => continue,
                FindFilesResult::Err(err) => {
                    if ignore_errors {
                        continue;
                    } else {
                        return FindFilesResult::Err(err);
                    }
                }
                FindFilesResult::Some(paths) => {
                    let path = paths.first().unwrap().to_string();
                    found.push(path)
                }
            }
        }
    }

    if found.is_empty() {
        FindFilesResult::None
    } else {
        FindFilesResult::Some(found)
    }
}

fn match_file(
    file: &DirEntry,
    name_rule: Option<&Comprasion>,
    only_types: Option<&Vec<&FileType>>,
) -> FindFilesResult {
    let name = get_name(file);

    if name.is_none() {
        return FindFilesResult::None; // temporary file
    }

    let name = name.unwrap();

    if let Some(name_rule) = name_rule {
        if !name_rule.assert(name.as_str()) {
            return FindFilesResult::None;
        }
    }

    let metadata = file.metadata();

    if metadata.is_err() {
        let msg = format!("{}: metadata reading error", name);
        return FindFilesResult::Err(Error::other(msg));
    }

    let metadata = metadata.unwrap();

    if let Some(only_types) = only_types {
        if !only_types.iter().any(|r| r.assert(&metadata)) {
            return FindFilesResult::None;
        }
    }

    if let Some(mut path) = get_path(file) {
        if metadata.is_dir() {
            path = format!("{}/", path);
        }

        return FindFilesResult::Some(vec![path]);
    }

    FindFilesResult::None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::get_current_dir;

    #[test]
    fn test_find() -> Result<(), Error> {
        let fixture_dir = get_fixture_dir()?;
        let search_paths = vec![fixture_dir.as_str()];

        let symlink = format!("{}exe_symlink", fixture_dir.as_str());
        let dir = format!("{}dir/", fixture_dir.as_str());
        let file = format!("{}not_exe", fixture_dir.as_str());
        let executable = format!("{}exe", fixture_dir.as_str());

        let files = find_files(&search_paths, None, None, false).unwrap();
        assert!(files.contains(&symlink));
        assert!(files.contains(&dir));
        assert!(files.contains(&file));
        assert!(files.contains(&executable));

        let only_types = vec![&FileType::Dir];
        let files = find_files(&search_paths, Some(&only_types), None, false).unwrap();

        assert!(!files.contains(&symlink));
        assert!(files.contains(&dir));
        assert!(!files.contains(&file));
        assert!(!files.contains(&executable));

        let only_types = vec![&FileType::SymLink];
        let files = find_files(&search_paths, Some(&only_types), None, false).unwrap();

        assert!(files.contains(&symlink));
        assert!(!files.contains(&dir));
        assert!(!files.contains(&file));
        assert!(!files.contains(&executable));

        let only_types = vec![&FileType::Executable];
        let files = find_files(&search_paths, Some(&only_types), None, false).unwrap();

        assert!(!files.contains(&symlink));
        assert!(!files.contains(&dir));
        assert!(!files.contains(&file));
        assert!(files.contains(&executable));

        let only_types = vec![&FileType::File];
        let files = find_files(&search_paths, Some(&only_types), None, false).unwrap();

        assert!(!files.contains(&symlink));
        assert!(!files.contains(&dir));
        assert!(files.contains(&file));
        assert!(files.contains(&executable));

        let name_rule = Comprasion::Equal("dir".to_string());
        let files = find_files(&search_paths, None, Some(&name_rule), false).unwrap();
        assert!(!files.contains(&symlink));
        assert!(files.contains(&dir));
        assert!(!files.contains(&file));
        assert!(!files.contains(&executable));

        let name_rule = Comprasion::AssertedStartsWith("exe".to_string());
        let files = find_files(&search_paths, None, Some(&name_rule), false).unwrap();
        assert!(files.contains(&symlink));
        assert!(!files.contains(&dir));
        assert!(!files.contains(&file));
        assert!(files.contains(&executable));

        Ok(())
    }

    fn get_fixture_dir() -> Result<String, Error> {
        // ends with a slash
        Ok(format!("{}/test/fixture/fs/", get_current_dir()?))
    }
}
