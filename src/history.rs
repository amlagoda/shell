use crate::fs::{get_read_file, get_write_file};
use crate::structure::KeyValue;
use crate::structure::RevIter;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};

pub struct Log {
    data: Vec<String>,
    navigator: RevIter,
    uploads: KeyValue,
}

impl Log {
    pub fn new() -> Log {
        Log {
            data: Vec::with_capacity(50),
            navigator: RevIter::new(0),
            uploads: KeyValue::new(),
        }
    }

    pub fn add(&mut self, mut values: Vec<String>) {
        self.data.append(&mut values);
        self.navigator = RevIter::new(self.data.len() - 1);
    }

    pub fn lasts(&self, count: Option<usize>) -> (impl Iterator<Item = &str>, usize) {
        let mut start = 0;
        let mut len = self.data.len();

        if let Some(count) = count {
            if len > 0 && count < len {
                start = len - count;
                len = len - start;
            }
        }

        (self.data[start..].iter().map(|r| r.as_str()), len)
    }

    // move to start
    pub fn next(&mut self) -> Option<String> {
        if let Some(index) = self.move_index(Direction::Next) {
            Some(self.data[index].clone())
        } else {
            None
        }
    }

    // move to end
    pub fn prev(&mut self) -> Option<String> {
        if let Some(index) = self.move_index(Direction::Prev) {
            Some(self.data[index].clone())
        } else {
            None
        }
    }

    fn move_index(&mut self, direction: Direction) -> Option<usize> {
        if self.data.is_empty() {
            return None;
        }

        match direction {
            Direction::Next => self.navigator.next(),
            Direction::Prev => self.navigator.prev(),
        }
    }

    pub fn reset(&mut self) {
        let mut last_index = 0;

        if !self.data.is_empty() {
            last_index = self.data.len() - 1;
        }

        self.navigator = RevIter::new(last_index);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_upload_index(&self, file_path: &str) -> Option<usize> {
        self.uploads.get(file_path)
    }

    pub fn set_upload_index(&mut self, file_path: &str, index: usize) {
        self.uploads.set(file_path, index)
    }
}

pub fn download(log: &mut Log, file_path: &str) -> Result<(), Error> {
    let err = Error::new(ErrorKind::NotFound, "No such file or directory");
    let file = get_read_file(file_path).map_err(|_| err)?;
    let buffer = BufReader::with_capacity(4096, file);
    let mut loaded = Vec::with_capacity(50);

    for line in buffer.lines() {
        loaded.push(line?);

        if loaded.len() == 50 {
            log.add(loaded.drain(..).collect());
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

enum Direction {
    Next,
    Prev,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lasts() {
        let mut log = Log::new();

        let (mut iter, len) = log.lasts(None);
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(1));
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        log.add(vec!["1".to_string()]);

        let (mut iter, len) = log.lasts(None);
        assert_eq!(Some("1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(0));
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(1));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(2));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        log.add(vec!["2".to_string()]);

        let (mut iter, len) = log.lasts(None);
        assert_eq!(Some("1"), iter.next());
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(2, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(0));
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(1));
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(2));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(2, len);
        drop(iter);

        let (mut iter, len) = log.lasts(Some(3));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(2, len);
        drop(iter);
    }

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
