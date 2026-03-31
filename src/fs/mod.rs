mod search;

use libc::{dup as c_dup, fcntl as c_fcntl, F_SETFL, O_NONBLOCK};
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::os::fd::FromRawFd;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub use search::search_executable_file_in_paths;
pub use search::search_executable_files_in_paths;

pub fn to_nonblock_file(file_descriptor: u32) -> Result<File, Error> {
    let status = unsafe { c_fcntl(file_descriptor as i32, F_SETFL, O_NONBLOCK) };

    if status == -1 {
        return Err(Error::other("fcntl error"));
    }

    let file = unsafe { File::from_raw_fd(file_descriptor as i32) };

    Ok(file)
}

pub fn to_independent_file(file_descriptor: u32) -> File {
    unsafe { File::from_raw_fd(c_dup(file_descriptor as i32)) }
}

pub fn transfer_data(
    mut from: File,
    mut to: File,
    proceed: Arc<AtomicBool>,
    mut skip_first_newline: bool,
    memory_ordering: Ordering,
) -> Result<(), Error> {
    let mut buffer = [0; 4096];

    loop {
        match from.read(&mut buffer) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    break;
                }

                skip_first_newline = write_lines(&mut to, &buffer, read_bytes, skip_first_newline)?;
                buffer = [0; 4096];
            }
            Err(err) => {
                if err.kind() == ErrorKind::WouldBlock {
                    if !proceed.load(memory_ordering) {
                        while let Ok(read_bytes) = from.read(&mut buffer) {
                            if read_bytes == 0 {
                                break;
                            }

                            skip_first_newline =
                                write_lines(&mut to, &buffer, read_bytes, skip_first_newline)?;
                            buffer = [0; 4096];
                        }
                        break;
                    }

                    sleep(Duration::from_millis(10));
                    continue;
                }

                // when running the tests uncategorized error
                // not reproducible locally
                return Ok(());
            }
        }
    }

    Ok(())
}

fn write_lines(
    target: &mut File,
    buffer: &[u8; 4096],
    read_bytes: usize,
    mut skip_first_newline: bool,
) -> Result<bool, Error> {
    let readed = String::from_utf8(buffer[..read_bytes].to_vec())
        .map_err(|_| Error::other("from_utf8 error"))?;

    // skip unnecessary newlines
    for line in readed.split("\n").filter(|r| !["\n", "\0", ""].contains(r)) {
        if skip_first_newline {
            skip_first_newline = false;
            write!(target, "{}", line)?;
        } else {
            write!(target, "\r\n{}", line)?;
        }

        target.flush()?;
    }

    Ok(skip_first_newline)
}

pub fn get_write_file(path: &str, append: bool) -> Result<File, Error> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(Path::new(path))?;

    Ok(file)
}

pub fn get_read_file(path: &str) -> Result<File, Error> {
    let file = OpenOptions::new().read(true).open(Path::new(path))?;

    Ok(file)
}
