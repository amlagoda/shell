pub struct Input {
    data: String,
    has_user_typing: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            data: String::new(),
            has_user_typing: false,
        }
    }

    pub fn data(&self) -> &str {
        self.data.as_str()
    }

    pub fn has_user_typing(&self) -> bool {
        self.has_user_typing
    }

    pub fn push(&mut self, data: &str, is_user_typing: bool) {
        self.data.push_str(data);

        if is_user_typing {
            self.has_user_typing = true;
        }
    }

    pub fn remove_last(&mut self, count: usize) {
        if count == 0 || self.data.is_empty() {
            return;
        }

        if count >= self.data.len() {
            self.data = String::new();
            self.has_user_typing = false;
            return;
        }

        self.data = self
            .data
            .chars()
            .take(self.data.chars().count() - count)
            .collect();
    }

    pub fn clear(&mut self) {
        self.data = String::new();
        self.has_user_typing = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut input = Input::new();

        input.push("a", false);
        assert_eq!("a", input.data());
        assert_eq!(false, input.has_user_typing());

        input.push("b", true);
        assert_eq!("ab", input.data());
        assert_eq!(true, input.has_user_typing());

        input.push("c", false);
        assert_eq!("abc", input.data());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(0);
        assert_eq!("abc", input.data());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(1);
        assert_eq!("ab", input.data());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(2);
        assert_eq!("", input.data());
        assert_eq!(false, input.has_user_typing());

        input.push("abc", true);
        input.clear();
        assert_eq!("", input.data());
        assert_eq!(false, input.has_user_typing());
    }
}
