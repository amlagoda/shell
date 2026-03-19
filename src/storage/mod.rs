mod r#struct;

use crate::storage::r#struct::RevIter;

pub struct History {
    data: Vec<String>,
    navigator: RevIter,
}

impl History {
    pub fn new() -> History {
        History {
            data: Vec::with_capacity(10),
            navigator: RevIter::new(0),
        }
    }

    pub fn add(&mut self, value: String) {
        self.data.push(value);
        self.navigator = RevIter::new(self.data.len() - 1);
    }

    pub fn all(&self) -> Option<Vec<String>> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.clone())
        }
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
}

enum Direction {
    Next,
    Prev,
}
