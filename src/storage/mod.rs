mod r#struct;

use crate::storage::r#struct::KeyValue;

pub struct History {
    data: Vec<String>,
    current: usize,
    first_prev: bool,
    prev_done: bool,
    next_done: bool,
    file_index: KeyValue,
}

impl History {
    pub fn new() -> History {
        History {
            data: vec![],
            current: 0,
            first_prev: false,
            prev_done: false,
            next_done: true,
            file_index: KeyValue::new(),
        }
    }

    pub fn add(&mut self, value: String) {
        self.data.push(value);
        self.current = self.data.len() - 1;
    }

    pub fn all(&self) -> Option<Vec<String>> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.clone())
        }
    }

    pub fn prev(&mut self) -> Option<String> {
        if self.data.is_empty() || self.prev_done {
            return None;
        }

        if !self.first_prev {
            self.first_prev = true;

            if self.current == 0 {
                self.prev_done = true;
            }

            return Some(self.data[self.current].clone());
        }

        self.current -= 1;

        if self.current == 0 {
            self.prev_done = true;
        }

        if self.data.len() > 1 {
            self.next_done = false;
        }

        Some(self.data[self.current].clone())
    }

    pub fn next(&mut self) -> Option<String> {
        if self.data.is_empty() || self.next_done {
            return None;
        }

        self.prev_done = false;
        self.current += 1;

        if self.current == self.data.len() - 1 {
            self.next_done = true;
        }

        Some(self.data[self.current].clone())
    }

    pub fn reset(&mut self) {
        self.prev_done = false;
        self.next_done = true;
        self.first_prev = false;

        self.current = if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        };
    }
}
