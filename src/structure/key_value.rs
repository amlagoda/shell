use std::default::Default;

// for small, rarely used collection
pub struct KeyValue {
    data: Vec<(String, usize)>,
}

impl Default for KeyValue {
    fn default() -> KeyValue {
        KeyValue {
            data: Vec::with_capacity(50),
        }
    }
}

impl KeyValue {
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
    fn test_key_value() {
        let mut key_value = KeyValue::default();

        key_value.set("key", 1);
        assert_eq!(1, key_value.get("key").unwrap());

        key_value.set("key", 2);
        assert_eq!(2, key_value.get("key").unwrap());

        assert_eq!(None, key_value.get("not exists"));
    }
}
