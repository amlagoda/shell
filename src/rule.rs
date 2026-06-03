pub enum Equal {
    Exac(String),
    StartsWith(String),
    StartsWithNotExac(String),
}

impl Equal {
    pub fn assert(&self, value: &str) -> bool {
        match self {
            Equal::Exac(r) => r == value,
            Equal::StartsWith(r) => value.starts_with(r),
            Equal::StartsWithNotExac(r) => value.starts_with(r) && value != r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_equal() -> Result<(), Error> {
        let equal = Equal::Exac("foo".to_string());
        assert!(equal.assert("foo"));
        assert!(!equal.assert("bar"));
        assert!(!equal.assert("fo"));
        assert!(!equal.assert("fooo"));

        let equal = Equal::StartsWith("foo".to_string());
        assert!(equal.assert("foo"));
        assert!(!equal.assert("fo"));
        assert!(!equal.assert("f"));
        assert!(!equal.assert(""));
        assert!(equal.assert("fooo"));
        assert!(!equal.assert("o"));

        let equal = Equal::StartsWithNotExac("foo".to_string());
        assert!(!equal.assert("foo"));
        assert!(equal.assert("fooo"));

        Ok(())
    }
}
