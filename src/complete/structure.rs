struct FileFindData {
    find_path: String,
    file_prefix: Option<String>,
}

impl FileFindData {
    fn from(find_path: String, file_prefix: Option<String>) -> FileFindData {
        FileFindData {
            find_path,
            file_prefix,
        }
    }

    fn find_path(&self) -> &str {
        self.find_path.as_str()
    }

    fn file_prefix(&self) -> Option<&str> {
        self.file_prefix.as_deref()
    }
}
