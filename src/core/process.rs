use crate::core::io::Stdio;
use libc::{c_char, dup2 as c_dup2, execvp as c_execvp, fork as c_fork};
use std::ffi::CString;
use std::io::Error;
use std::iter::once;
use std::ptr::null;

struct Fork {
    pid: u32,
}

impl Fork {
    fn try_new() -> Result<Fork, Error> {
        let pid = unsafe { c_fork() };

        if pid >= 0 {
            Ok(Fork { pid: pid as u32 })
        } else {
            Err(Error::other("fork error"))
        }
    }

    fn is_child(&self) -> bool {
        self.pid == 0
    }

    fn get_pid(&self) -> u32 {
        self.pid
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
        let status = unsafe { c_dup2(file_descriptor as i32, io.as_uint() as i32) };

        if status == -1 {
            Err(Error::other("dup2 error"))
        } else {
            Ok(())
        }
    }

    // reload the binary file of the process and transfer control to it
    // any return value means failure
    fn hot_reload_bin(&self, bin: &str, args: Option<Vec<&str>>) -> Error {
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
            .into_iter()
            .map(|arg| arg.as_ptr())
            .chain(once(null()))
            .collect();

        unsafe { c_execvp(bin.as_ptr(), args.as_ptr()) };

        Error::other("execvp error")
    }
}
