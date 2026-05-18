mod find;
mod write;

use libc::{dup as c_dup, fcntl as c_fcntl, F_SETFL, O_NONBLOCK};
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::os::fd::FromRawFd;
use std::path::Path;

pub use find::{find_file, find_files};
pub use write::transfer_data;

// clone descriptor
pub fn to_cloned_file(file_descriptor: u32) -> Result<File, Error> {
    let file_descriptor = unsafe { c_dup(file_descriptor as i32) };

    if file_descriptor == -1 {
        return Err(Error::other("dup error"));
    }

    Ok(unsafe { File::from_raw_fd(c_dup(file_descriptor as i32)) })
}

pub fn to_nonblock_file(file_descriptor: i32) -> Result<File, Error> {
    if file_descriptor < 0 {
        return Err(Error::other("incorrect file descriptor"));
    }

    let status = unsafe { c_fcntl(file_descriptor as i32, F_SETFL, O_NONBLOCK) };

    if status == -1 {
        return Err(Error::other("fcntl error"));
    }

    let file = unsafe { File::from_raw_fd(file_descriptor as i32) };

    Ok(file)
}

pub fn get_read_file(path: &str) -> Result<File, Error> {
    OpenOptions::new().read(true).open(Path::new(path))
}

pub fn get_write_file(path: &str, append: bool) -> Result<File, Error> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(Path::new(path))
}
