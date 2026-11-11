use crate::fs::{find_all_starts_with, find_bins_starts_with, FindFilesResult};

pub fn find_bins_starts_with_as_names(starts_with: &str, paths: &Vec<&str>) -> Option<Vec<String>> {
    if let FindFilesResult::Some(paths) = find_bins_starts_with(starts_with, paths) {
        let paths = paths.iter().map(|r| r.as_str()).collect();
        Some(paths_to_names(&paths))
    } else {
        None
    }
}

pub fn find_all_starts_with_as_names(find_data: &FileFindData) -> Option<Vec<String>> {
    let starts_with = find_data.file_prefix().unwrap_or("");
    let paths = vec![find_data.find_path()];

    if let FindFilesResult::Some(paths) = find_all_starts_with(starts_with, &paths) {
        let paths = paths.iter().map(|r| r.as_str()).collect();
        Some(paths_to_names(&paths))
    } else {
        None
    }
}

pub fn to_find_data(input: &str, default_path: &str) -> FileFindData {
    let input = input.trim();

    let default_path = if !default_path.trim().ends_with("/") {
        format!("{}/", default_path.trim())
    } else {
        default_path.trim().to_string()
    };

    if input.is_empty() {
        return FileFindData::from(default_path, None);
    }

    if !input.contains("/") {
        return FileFindData::from(default_path, Some(input.to_string()));
    }

    let input: Vec<&str> = input.split("/").collect();
    let file_prefix = input.last().unwrap();

    if file_prefix.is_empty() {
        FileFindData::from(input.join("/"), None)
    } else {
        let find_path = format!("{}/", input[0..input.len() - 1].join("/"));
        FileFindData::from(find_path, Some(file_prefix.to_string()))
    }
}

pub fn paths_to_names(paths: &Vec<&str>) -> Vec<String> {
    let mut names = vec![];

    for path in paths {
        let name = if path.ends_with("/") {
            let name = path.trim_end_matches(['/']);
            format!("{}/", name.split("/").last().unwrap())
        } else {
            path.split("/").last().unwrap().to_string()
        };

        names.push(name);
    }

    names
}

#[derive(Debug)]
pub struct FileFindData {
    find_path: String,
    file_prefix: Option<String>,
}

impl FileFindData {
    pub fn from(find_path: String, file_prefix: Option<String>) -> FileFindData {
        FileFindData {
            find_path,
            file_prefix,
        }
    }

    pub fn find_path(&self) -> &str {
        self.find_path.as_str()
    }

    pub fn file_prefix(&self) -> Option<&str> {
        self.file_prefix.as_deref()
    }
}
