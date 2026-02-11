pub struct NewLine {
    pub stdout_start: bool,
    pub stdout_end: bool,
    pub stderr_start: bool,
    pub stderr_end: bool,
}

impl NewLine {
    pub fn new() -> NewLine {
        NewLine {
            stdout_start: false,
            stdout_end: false,
            stderr_start: false,
            stderr_end: false,
        }
    }

    pub fn stdout_start(&self) -> String {
        to_string(self.stdout_start)
    }

    pub fn stdout_end(&self) -> String {
        to_string(self.stdout_end)
    }

    pub fn stderr_start(&self) -> String {
        to_string(self.stderr_start)
    }

    pub fn stderr_end(&self) -> String {
        to_string(self.stderr_end)
    }
}

fn to_string(val: bool) -> String {
    if val {
        "\r\n".to_string()
    } else {
        "".to_string()
    }
}
