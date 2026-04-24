use std::fs::File;
use std::os::fd::FromRawFd;

pub struct Stdio {
    stdin: File,
    stdout: File,
    stderr: File,
}

impl Stdio {
    pub fn new() -> Stdio {
        unsafe {
            Stdio {
                stdin: File::from_raw_fd(0),
                stdout: File::from_raw_fd(0),
                stderr: File::from_raw_fd(0),
            }
        }
    }

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
