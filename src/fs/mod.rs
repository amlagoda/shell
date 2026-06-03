mod find;
mod write;

use libc::{dup as c_dup, fcntl as c_fcntl, F_SETFL, O_NONBLOCK};
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::os::fd::FromRawFd;
use std::path::Path;

pub use find::FindFilesResult;
pub use find::{find_bins_by_name, find_bins_starts_with, find_files_starts_with};
pub use write::transfer_data;

pub fn clone_descriptor_as_file(file_descriptor: i32) -> Result<File, Error> {
    if file_descriptor < 0 {
        return Err(Error::other("incorrect file descriptor"));
    }

    let cloned = unsafe { c_dup(file_descriptor) };

    if cloned == -1 {
        return Err(Error::other("dup error"));
    }

    Ok(unsafe { File::from_raw_fd(cloned) })
}

pub fn clone_descriptor_as_nonblock_file(file_descriptor: i32) -> Result<File, Error> {
    if file_descriptor < 0 {
        return Err(Error::other("incorrect file descriptor"));
    }

    let cloned = unsafe { c_dup(file_descriptor) };

    if cloned == -1 {
        return Err(Error::other("dup error"));
    }

    let status = unsafe { c_fcntl(cloned, F_SETFL, O_NONBLOCK) };

    if status == -1 {
        return Err(Error::other("fcntl error"));
    }

    Ok(unsafe { File::from_raw_fd(cloned) })
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
