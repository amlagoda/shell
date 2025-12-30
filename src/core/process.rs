use libc::{dup2 as c_dup2, fork as c_fork};
use std::io::Error;

struct Process {
    pid: Pid,
}

impl Process {
    fn try_new() -> Result<Process, Error> {
        let pid = unsafe { c_fork() };

        if pid >= 0 {
            Ok(Process {
                pid: Pid::new(pid as u32),
            })
        } else {
            Err(Error::other("fork error"))
        }
    }

    fn is_child(&self) -> bool {
        self.pid.get_pid() == 0
    }

    fn set_stdin(&self, file_descriptor: u32) -> Result<(), Error> {
        self.set_io(Stdio::Stdin, file_descriptor)
    }

    fn set_stdout(&self, file_descriptor: u32) -> Result<(), Error> {
        self.set_io(Stdio::Stdout, file_descriptor)
    }

    fn set_stderr(&self, file_descriptor: u32) -> Result<(), Error> {
        self.set_io(Stdio::Stderr, file_descriptor)
    }

    fn set_io(&self, io: Stdio, file_descriptor: u32) -> Result<(), Error> {
        let status = unsafe { c_dup2(file_descriptor as i32, io.as_int() as i32) };

        if status == -1 {
            Err(Error::other("dup2 error"))
        } else {
            Ok(())
        }
    }
}

struct Pid {
    pid: u32,
}

impl Pid {
    fn new(pid: u32) -> Pid {
        Pid { pid }
    }

    fn get_pid(&self) -> u32 {
        self.pid
    }
}

enum Stdio {
    Stdin,
    Stdout,
    Stderr,
}

impl Stdio {
    fn as_int(&self) -> u32 {
        match self {
            Stdio::Stdin => 0,
            Stdio::Stdout => 1,
            Stdio::Stderr => 2,
        }
    }
}
