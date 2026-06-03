pub enum Comprasion {
    Equal(String),
    PatternStartsWith(String),
    PatternStartsWithNotEqual(String),
    AssertedStartsWith(String),
    // AssertedStartsWithNotEqual(String),
}

impl Comprasion {
    pub fn assert(&self, asserted: &str) -> bool {
        match self {
            Comprasion::Equal(pattern) => pattern == asserted,
            Comprasion::PatternStartsWith(pattern) => pattern.starts_with(asserted),
            Comprasion::PatternStartsWithNotEqual(pattern) => {
                pattern.starts_with(asserted) && pattern != asserted
            }
            Comprasion::AssertedStartsWith(pattern) => asserted.starts_with(pattern),
            // Comprasion::AssertedStartsWithNotEqual(pattern) => {
            //     asserted.starts_with(pattern) && asserted != pattern
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_comprasion() -> Result<(), Error> {
        let rule = Comprasion::Equal("foo".to_string());
        assert!(rule.assert("foo"));
        assert!(!rule.assert("fo"));

        let rule = Comprasion::PatternStartsWith("foo".to_string());
        assert!(rule.assert("foo"));
        assert!(rule.assert("fo"));
        assert!(rule.assert("f"));
        assert!(rule.assert(""));
        assert!(!rule.assert("fooo"));

        let rule = Comprasion::PatternStartsWithNotEqual("foo".to_string());
        assert!(!rule.assert("foo"));
        assert!(rule.assert("fo"));
        assert!(rule.assert("f"));
        assert!(rule.assert(""));
        assert!(!rule.assert("fooo"));

        let rule = Comprasion::AssertedStartsWith("foo".to_string());
        assert!(rule.assert("fooo"));
        assert!(rule.assert("foo"));
        assert!(!rule.assert("fo"));

        // let rule = Comprasion::AssertedStartsWithNotEqual("foo".to_string());
        // assert!(rule.assert("fooo"));
        // assert!(!rule.assert("foo"));
        // assert!(!rule.assert("fo"));

        Ok(())
    }
}
