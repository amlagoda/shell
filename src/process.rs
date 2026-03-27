use libc::{c_char, dup2 as c_dup2, execvp as c_execvp, fork as c_fork, waitpid as c_waitpid};
use libc::{getpid as c_getpid, kill as c_kill, setpgid as c_setpgid, SIGKILL, WNOHANG};
use std::ffi::CString;
use std::io::Error;
use std::iter::once;
use std::ptr::{null, null_mut};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn to_group(member_pid: u32, group_pid: u32) {
    unsafe { c_setpgid(member_pid as i32, group_pid as i32) };
}

pub fn pid() -> u32 {
    unsafe { c_getpid() as u32 }
}

pub fn kill_forks(forks: Vec<Fork>) {
    for fork in forks {
        fork.kill();
    }
}

pub struct Fork {
    pid: u32,
}

impl Fork {
    pub fn try_new() -> Result<Fork, Error> {
        let pid = unsafe { c_fork() };

        if pid >= 0 {
            Ok(Fork { pid: pid as u32 })
        } else {
            Err(Error::other("fork error"))
        }
    }

    pub fn is_child(&self) -> bool {
        self.pid == 0
    }

    pub fn set_stdin(&self, file_descriptor: u32) -> Result<(), Error> {
        self.set_io(&Stdio::Stdin, file_descriptor)
    }

    pub fn set_stdout(&self, file_descriptor: u32) -> Result<(), Error> {
        self.set_io(&Stdio::Stdout, file_descriptor)
    }

    pub fn set_stderr(&self, file_descriptor: u32) -> Result<(), Error> {
        self.set_io(&Stdio::Stderr, file_descriptor)
    }

    fn set_io(&self, io: &Stdio, file_descriptor: u32) -> Result<(), Error> {
        let status = unsafe { c_dup2(file_descriptor as i32, io.as_uint() as i32) };

        if status == -1 {
            Err(Error::other("dup2 error"))
        } else {
            Ok(())
        }
    }

    // reload the binary file of the process and transfer control to it
    // any return value means failure
    pub fn hot_reload_bin(&self, bin: &str, args: Option<Vec<&str>>) -> Error {
        let merged_args: Vec<&str> = vec![bin]
            .into_iter()
            .chain(args.unwrap_or(vec![]))
            .collect();

        let mut args: Vec<CString> = Vec::with_capacity(merged_args.capacity());

        for arg in merged_args {
            match CString::new(arg) {
                Ok(arg) => args.push(arg),
                Err(_) => return Error::other("cstring error"),
            }
        }

        let bin = args[0].clone();

        let args: Vec<*const c_char> = args
            .iter()
            .map(|arg| arg.as_ptr())
            .chain(once(null()))
            .collect();

        unsafe { c_execvp(bin.as_ptr(), args.as_ptr()) };

        Error::other("execvp error")
    }

    pub fn blocking_waiting(&self) {
        unsafe { c_waitpid(self.pid as i32, null_mut(), 0) };
    }

    pub fn kill(&self) {
        unsafe {
            c_kill(self.pid as i32, SIGKILL);

            let start = Instant::now();
            let timeout = Duration::from_secs(1);

            loop {
                let status = c_waitpid(self.pid as i32, null_mut(), WNOHANG);

                // success wait or D-state process
                if status == self.pid as i32 || start.elapsed() > timeout {
                    break;
                }

                sleep(Duration::from_millis(10));
            }
        };
    }
}

enum Stdio {
    Stdin,
    Stdout,
    Stderr,
}

impl Stdio {
    fn as_uint(&self) -> u32 {
        match self {
            Stdio::Stdin => 0,
            Stdio::Stdout => 1,
            Stdio::Stderr => 2,
        }
    }
}
