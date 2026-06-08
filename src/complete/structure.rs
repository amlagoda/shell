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

#[derive(Debug, PartialEq)]
pub struct Completion {
    selected: Option<String>,
    variants: Option<Vec<String>>,
}

impl Completion {
    pub fn from_selected(selected: String) -> Completion {
        Completion {
            selected: Some(selected),
            variants: None,
        }
    }

    pub fn from_variants(variants: Vec<String>) -> Completion {
        Completion {
            selected: None,
            variants: Some(variants),
        }
    }

    pub fn is_selected(&self) -> bool {
        self.selected.is_some()
    }

    pub fn get_selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    pub fn get_variants(&self) -> Option<Vec<&str>> {
        self.variants
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
    }
}
