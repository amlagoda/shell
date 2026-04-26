use crate::fmt::NewLine;

pub struct Setting {
    new_line: NewLine,
    bin_paths: Vec<String>,
    current_dir: String,
}

impl Setting {
    pub fn from(new_line: NewLine, bin_paths: Vec<String>, current_dir: String) -> Setting {
        Setting {
            new_line,
            bin_paths,
            current_dir,
        }
    }

    pub fn new_line(&self) -> &NewLine {
        &self.new_line
    }

    pub fn bin_paths(&self) -> Vec<&str> {
        self.bin_paths.iter().map(|r| r.as_str()).collect()
    }

    pub fn current_dir(&self) -> &str {
        self.current_dir.as_str()
    }
}
