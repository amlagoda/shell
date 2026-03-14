pub struct Storage {
    data: Vec<String>,
    current: Option<usize>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            data: vec![],
            current: None,
        }
    }

    pub fn add(&mut self, value: String) {
        self.data.push(value);

        if let Some(current) = self.current {
            self.current = Some(current + 1);
        } else {
            self.current = Some(0);
        }
    }

    pub fn all(&self) -> Option<Vec<&str>> {
        if self.data.is_empty() {
            None
        } else {
            let data = self.data.iter().map(|value| value.as_str()).collect();
            Some(data)
        }
    }

    pub fn prev(&mut self) -> Option<String> {
        if let Some(current) = self.current {
            if current > 0 {
                self.current = Some(current - 1);
            }
            Some(self.data[current].clone())
        } else {
            None
        }
    }
}
