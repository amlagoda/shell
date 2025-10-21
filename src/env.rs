mod env {
    use std::env::{var, VarError};

    fn split_env_path() -> Result<Vec<String>, VarError> {
        match var("PATH") {
            Ok(r) => Ok(r
                .split(':')
                .map(|r| String::from(r))
                .collect::<Vec<String>>()),
            Err(e) => Err(e),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::set_var;

        #[test]
        fn test_split_env_path() {
            set_var("PATH", "foo:bar");

            assert_eq!(
                vec!["foo".to_string(), "bar".to_string()],
                split_env_path().unwrap()
            );
        }
    }
}
