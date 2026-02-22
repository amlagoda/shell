use libc::{c_char, dup2 as c_dup2, execvp as c_execvp, fork as c_fork, waitpid as c_waitpid};
use libc::{getpid as c_getpid, kill as c_kill, setpgid as c_setpgid, SIGKILL};
use std::ffi::CString;
use std::io::Error;
use std::iter::once;
// use std::process::{Command, Stdio as CommandStdio};
use std::ptr::{null, null_mut};

// pid - the pid of the parent process and the pgid of the group at the same time
/*pub fn kill_group_childs(pid: u32) -> Result<(), Error> {
    let process = Command::new("ps")
        .stdin(CommandStdio::null())
        .stdout(CommandStdio::piped())
        .stderr(CommandStdio::null())
        .args(["-g", pid.to_string().as_str(), "-o", "pid"])
        .spawn()?;

    let output = process.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout).map_err(|_| Error::other("from_utf8 error"))?;

    let pids = stdout
        .trim()
        .split("\n")
        .into_iter()
        .map(|r| r.trim())
        .enumerate()
        .filter(|&(i, val)| i != 0 && val.parse::<u32>().unwrap() != pid)
        .map(|(_, val)| val)
        .collect::<Vec<&str>>();

    for pid in pids {
        let pid = pid.parse::<i32>().unwrap();

        unsafe {
            c_kill(pid, libc::SIGKILL);
            c_waitpid(pid, null_mut(), 0);
        };
    }

    Ok(())
}*/

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
            .chain(args.unwrap_or(vec![]).into_iter())
            .collect();

        let mut args: Vec<CString> = vec![];

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
        unsafe { c_kill(self.pid as i32, SIGKILL) };
    }

    // pub fn is_dead(&self) -> bool {
    //     match unsafe { c_kill(self.pid as i32, 0) } {
    //         -1 => Error::last_os_error().raw_os_error() == Some(libc::ESRCH),
    //         _ => false,
    //     }
    // }
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
