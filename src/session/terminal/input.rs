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

    pub fn get(&self) -> &str {
        self.data.as_str()
    }

    pub fn has_user_typing(&self) -> bool {
        self.has_user_typing
    }

    pub fn push_as_user(&mut self, data: &str) {
        self.data.push_str(data);
        self.has_user_typing = true;
    }

    pub fn push_as_system(&mut self, data: &str) {
        self.data.push_str(data);
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

    pub fn reset(&mut self) {
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

        input.push_as_system("a");
        assert_eq!("a", input.get());
        assert_eq!(false, input.has_user_typing());

        input.push_as_user("b");
        assert_eq!("ab", input.get());
        assert_eq!(true, input.has_user_typing());

        input.push_as_system("c");
        assert_eq!("abc", input.get());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(0);
        assert_eq!("abc", input.get());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(1);
        assert_eq!("ab", input.get());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(2);
        assert_eq!("", input.get());
        assert_eq!(false, input.has_user_typing());

        input.push_as_user("abc");
        input.reset();
        assert_eq!("", input.get());
        assert_eq!(false, input.has_user_typing());
    }
}
