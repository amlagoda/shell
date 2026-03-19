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
