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
