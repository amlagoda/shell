#[derive(Default)]
pub struct RevIter {
    current: usize,
    last: usize,
    is_started: bool,
}

impl RevIter {
    pub fn from(last: usize) -> RevIter {
        RevIter {
            current: last,
            last,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_iter() {
        let mut rev_iter = RevIter::default();
        assert_eq!(None, rev_iter.prev());
        assert_eq!(Some(0), rev_iter.next());
        assert_eq!(None, rev_iter.next());
        assert_eq!(None, rev_iter.prev());

        let mut rev_iter = RevIter::from(2);
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
}
