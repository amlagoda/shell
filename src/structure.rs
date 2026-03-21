pub struct RevIter {
    current: usize,
    last: usize,
    is_started: bool,
}

impl RevIter {
    pub fn new(last: usize) -> RevIter {
        RevIter {
            current: last,
            last: last,
            is_started: false,
        }
    }

    // move to start
    pub fn next(&mut self) -> Option<usize> {
        if !self.is_started {
            self.is_started = true;
            Some(self.current)
        } else if self.current > 0 {
            self.current -= 1;
            Some(self.current)
        } else {
            None
        }
    }

    // move to end
    pub fn prev(&mut self) -> Option<usize> {
        if self.current < self.last {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// for small collection
pub struct KeyValue {
    data: Vec<(String, usize)>,
}

impl KeyValue {
    pub fn new() -> KeyValue {
        KeyValue { data: vec![] }
    }

    pub fn get(&self, key: &str) -> Option<usize> {
        for (inkey, value) in self.data.iter() {
            if inkey == key {
                return Some(*value);
            }
        }

        None
    }

    pub fn set(&mut self, key: &str, value: usize) {
        for (num, (inkey, _)) in self.data.iter().enumerate() {
            if inkey == key {
                self.data[num] = (key.to_string(), value);
                return;
            }
        }

        self.data.push((key.to_string(), value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_iter() {
        let mut rev_iter = RevIter::new(0);
        assert_eq!(None, rev_iter.prev());
        assert_eq!(Some(0), rev_iter.next());
        assert_eq!(None, rev_iter.next());
        assert_eq!(None, rev_iter.prev());

        let mut rev_iter = RevIter::new(2);
        assert_eq!(None, rev_iter.prev());
        assert_eq!(Some(2), rev_iter.next());
        assert_eq!(Some(1), rev_iter.next());
        assert_eq!(Some(0), rev_iter.next());
        assert_eq!(None, rev_iter.next());
        assert_eq!(Some(1), rev_iter.prev());
        assert_eq!(Some(2), rev_iter.prev());
        assert_eq!(None, rev_iter.prev());
        assert_eq!(Some(1), rev_iter.next());
        assert_eq!(Some(0), rev_iter.next());
        assert_eq!(None, rev_iter.next());
    }

    #[test]
    fn test_key_value() {
        let mut key_value = KeyValue::new();

        key_value.set("key", 1);
        assert_eq!(1, key_value.get("key").unwrap());

        key_value.set("key", 2);
        assert_eq!(2, key_value.get("key").unwrap());

        assert_eq!(None, key_value.get("not exists"));
    }
}
