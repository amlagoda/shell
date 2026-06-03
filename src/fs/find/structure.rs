use std::cmp::PartialEq;
use std::fs::Metadata;
use std::io::Error;
use std::os::unix::fs::PermissionsExt;

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

pub enum FileType {
    File,
    SymLink,
    Executable,
    Dir,
}

impl FileType {
    pub fn assert(&self, metadata: &Metadata) -> bool {
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
