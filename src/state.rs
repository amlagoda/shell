pub struct Storage {
    data: Vec<String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage { data: vec![] }
    }

    pub fn add(&mut self, value: String) {
        self.data.push(value)
    }

    pub fn get(&self) -> Option<Vec<&str>> {
        if self.data.is_empty() {
            None
        } else {
            let data = self.data.iter().map(|value| value.as_str()).collect();
            Some(data)
        }
    }
}
