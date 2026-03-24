use crate::fs::{get_read_file, get_write_file};
use crate::history::Log;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};

pub fn download(log: &mut Log, file_path: &str) -> Result<(), Error> {
    let err = Error::new(ErrorKind::NotFound, "No such file or directory");
    let file = get_read_file(file_path).map_err(|_| err)?;
    let buffer = BufReader::with_capacity(4096, file);
    let mut loaded = Vec::with_capacity(50);

    for line in buffer.lines() {
        loaded.push(line?);

        if loaded.len() == 50 {
            log.add(loaded.drain(..).collect()); // not mem::take to save capacity
        }
    }

    if !loaded.is_empty() {
        log.add(loaded);
    }

    Ok(())
}

pub fn upload(log: &mut Log, file_path: &str, append: bool) -> Result<(), Error> {
    let previous_index = if append {
        log.get_upload_index(file_path)
    } else {
        None
    };

    let (count, new_index) = upload_numbers(previous_index, log.len());

    if new_index.is_none() {
        return Ok(());
    }

    let (records, _) = log.lasts(count);
    let mut file = get_write_file(file_path, append)?;
    let mut buffer = BufWriter::with_capacity(4096, &mut file);

    for record in records {
        buffer.write_all(format!("{}\n", record).as_bytes())?;
    }

    buffer.flush()?;
    log.set_upload_index(file_path, new_index.unwrap());

    Ok(())
}

fn upload_numbers(
    previous_index: Option<usize>,
    current_len: usize,
) -> (Option<usize>, Option<usize>) {
    let mut count = None;
    let mut new_index = None;

    if let Some(previous_index) = previous_index {
        if current_len > (previous_index + 1) {
            count = Some(current_len - (previous_index + 1));
        }
    }

    if current_len > 0 {
        new_index = Some(current_len - 1)
    }

    (count, new_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_numbers() {
        let (previous_index, current_len) = (None, 0);
        assert_eq!((None, None), upload_numbers(previous_index, current_len));

        let (previous_index, current_len) = (None, 1);
        assert_eq!((None, Some(0)), upload_numbers(previous_index, current_len));

        let (previous_index, current_len) = (None, 2);
        assert_eq!((None, Some(1)), upload_numbers(previous_index, current_len));

        let (previous_index, current_len) = (Some(0), 0);
        assert_eq!((None, None), upload_numbers(previous_index, current_len));

        let (previous_index, current_len) = (Some(0), 1);
        assert_eq!((None, Some(0)), upload_numbers(previous_index, current_len));

        let (previous_index, current_len) = (Some(0), 2);
        assert_eq!(
            (Some(1), Some(1)),
            upload_numbers(previous_index, current_len)
        );

        let (previous_index, current_len) = (Some(1), 4);
        assert_eq!(
            (Some(2), Some(3)),
            upload_numbers(previous_index, current_len)
        );
    }
}
