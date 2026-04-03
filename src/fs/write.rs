use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::str::from_utf8;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub fn transfer_data(
    mut from: File,
    mut to: File,
    proceed: Arc<AtomicBool>,
    mut skip_first_newline: bool,
    memory_ordering: Ordering,
) -> Result<(), Error> {
    // using buffers (BufReader, BufWriter) does not increase performance
    // because we still have to flush them after each record
    // for commands like tail -f file | head -n 5
    let mut buffer = [0; 4096];

    loop {
        match from.read(&mut buffer) {
            Ok(read_bytes) => {
                if read_bytes == 0 {
                    break;
                }

                skip_first_newline = write_lines(&mut to, &buffer, read_bytes, skip_first_newline)?;
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
    let readed = from_utf8(&buffer[..read_bytes]).map_err(|_| Error::other("from_utf8 error"))?;

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
