use crate::rule::Comprasion;
use std::cmp::PartialEq;
use std::fs::{read_dir, DirEntry, Metadata};
use std::io::Error;
use std::os::unix::fs::PermissionsExt;

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
        return FindFilesResult::Err(Error::other("search paths is empty"));
    }

    let mut found = vec![];

    for path in search_paths {
        let dir = read_dir(path);

        if dir.is_err() {
            if ignore_errors {
                continue;
            } else {
                let msg = format!("{}: dir is invalid", path);
                return FindFilesResult::Err(Error::other(msg));
            }
        }

        for file in dir.unwrap().flatten() {
            let name = get_name(&file);
            if name.is_none() {
                continue; // temporary file
            }

            let name = name.unwrap();
            if let Some(name_rule) = name_rule {
                if !name_rule.assert(name.as_str()) {
                    continue;
                }
            }

            let metadata = file.metadata();
            if metadata.is_err() {
                if ignore_errors {
                    continue;
                } else {
                    let msg = format!("{}: metadata reading error", name);
                    return FindFilesResult::Err(Error::other(msg));
                }
            }
            let metadata = metadata.unwrap();

            if let Some(only_types) = only_types {
                if !only_types.iter().any(|r| r.assert(&metadata)) {
                    continue;
                }
            }

            if let Some(path) = get_path(&file) {
                if metadata.is_dir() {
                    found.push(format!("{}/", path));
                } else {
                    found.push(path);
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

#[derive(Debug)]
pub enum FindFilesResult {
    Err(Error),
    None,
    Some(Vec<String>),
}

impl FindFilesResult {
    pub fn unwrap(self) -> Vec<String> {
        match self {
            FindFilesResult::None => panic!("called unwrap on None"),
            FindFilesResult::Err(_) => panic!("called unwrap on Err"),
            FindFilesResult::Some(r) => r,
        }
    }

    pub fn is_some(&self) -> bool {
        matches!(self, FindFilesResult::Some(_))
    }
}

impl PartialEq for FindFilesResult {
    fn eq(&self, other: &FindFilesResult) -> bool {
        match (self, other) {
            (FindFilesResult::None, FindFilesResult::None) => true,
            (FindFilesResult::Err(a), FindFilesResult::Err(b)) => a.to_string() == b.to_string(),
            (FindFilesResult::Some(a), FindFilesResult::Some(b)) => a == b,
            _ => false,
        }
    }
}

enum FileType {
    File,
    SymLink,
    Executable,
    Dir,
}

impl FileType {
    fn assert(&self, metadata: &Metadata) -> bool {
        match self {
            FileType::File => metadata.is_file(),
            FileType::SymLink => metadata.is_symlink(),
            FileType::Executable => {
                metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0)
                // windows no
            }
            FileType::Dir => metadata.is_dir(),
        }
    }
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
