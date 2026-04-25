use std::fs::File;

pub struct Stdio {
    stdin: File,
    stdout: File,
    stderr: File,
}

impl Stdio {
    pub fn from(stdin: File, stdout: File, stderr: File) -> Stdio {
        Stdio {
            stdin,
            stdout,
            stderr,
        }
    }

    pub fn stdin(&mut self) -> &mut File {
        &mut self.stdin
    }

    pub fn stdout(&mut self) -> &mut File {
        &mut self.stdout
    }

    pub fn stderr(&mut self) -> &mut File {
        &mut self.stderr
    }
}
