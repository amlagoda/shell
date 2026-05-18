pub struct NewLine {
    stdout_start: bool,
    stdout_end: bool,
    stderr_start: bool,
    stderr_end: bool,
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

    pub fn set_stdout_start(&mut self, value: bool) {
        self.stdout_start = value;
    }

    pub fn set_stdout_end(&mut self, value: bool) {
        self.stdout_end = value;
    }

    pub fn set_stderr_start(&mut self, value: bool) {
        self.stderr_start = value;
    }

    pub fn set_stderr_end(&mut self, value: bool) {
        self.stderr_end = value;
    }
}

fn to_string(val: bool) -> String {
    if val {
        "\n".to_string() // not \r\n
    } else {
        "".to_string()
    }
}

pub fn bell() -> String {
    "\x07".to_string()
}

pub fn new_line_raw() -> String {
    "\r\n".to_string()
}
