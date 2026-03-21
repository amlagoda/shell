use crate::structure::KeyValue;
use crate::structure::RevIter;

pub struct Log {
    data: Vec<String>,
    navigator: RevIter,
    uploads: KeyValue,
}

impl Log {
    pub fn new() -> Log {
        Log {
            data: Vec::with_capacity(30),
            navigator: RevIter::new(0),
            uploads: KeyValue::new(),
        }
    }

    pub fn add(&mut self, value: String) {
        self.data.push(value);
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

enum Direction {
    Next,
    Prev,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lasts() {
        let mut history = Log::new();

        let (mut iter, len) = history.lasts(None);
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(1));
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        history.add("1".to_string());

        let (mut iter, len) = history.lasts(None);
        assert_eq!(Some("1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(0));
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(1));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(2));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        history.add("2".to_string());

        let (mut iter, len) = history.lasts(None);
        assert_eq!(Some("1"), iter.next());
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(2, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(0));
        assert_eq!(None, iter.next());
        assert_eq!(0, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(1));
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(1, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(2));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(2, len);
        drop(iter);

        let (mut iter, len) = history.lasts(Some(3));
        assert_eq!(Some("1"), iter.next());
        assert_eq!(Some("2"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(2, len);
        drop(iter);
    }
}
