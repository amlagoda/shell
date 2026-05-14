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

    pub fn get(&self) -> Option<&str> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.as_str())
        }
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

    pub fn remove_last(&mut self, mut count: usize) {
        if count == 0 || self.data.is_empty() {
            return;
        }

        if count >= self.data.chars().count() {
            self.data.clear();
            self.has_user_typing = false;
        } else {
            while count > 0 && self.data.pop().is_some() {
                count -= 1;
            }
        }
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
        assert_eq!(Some("a"), input.get());
        assert_eq!(false, input.has_user_typing());

        input.push_as_user("b");
        assert_eq!(Some("ab"), input.get());
        assert_eq!(true, input.has_user_typing());

        input.push_as_system("c");
        assert_eq!(Some("abc"), input.get());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(0);
        assert_eq!(Some("abc"), input.get());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(1);
        assert_eq!(Some("ab"), input.get());
        assert_eq!(true, input.has_user_typing());

        input.remove_last(2);
        assert_eq!(None, input.get());
        assert_eq!(false, input.has_user_typing());

        input.push_as_user("abc");
        input.reset();
        assert_eq!(None, input.get());
        assert_eq!(false, input.has_user_typing());
    }
}
