use crate::fmt::NewLine;

pub struct Setting {
    mode: ProgramMode,
    new_line: NewLine,
    bin_paths: Vec<String>,
    current_dir: String,
    available_commands: Vec<String>,
}

impl Setting {
    pub fn from(
        mode: ProgramMode,
        new_line: NewLine,
        bin_paths: Vec<String>,
        current_dir: String,
        available_commands: Vec<String>,
    ) -> Setting {
        Setting {
            mode,
            new_line,
            bin_paths,
            current_dir,
            available_commands,
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

    pub fn available_commands(&self) -> Vec<&str> {
        self.available_commands.iter().map(|r| r.as_str()).collect()
    }

    pub fn bell(&self) -> &str {
        "\x07"
    }

    pub fn is_interactive_mode(&self) -> bool {
        matches!(self.mode, ProgramMode::Interactive)
    }
}

pub enum ProgramMode {
    Interactive,
    Command,
}
