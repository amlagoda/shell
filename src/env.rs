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

            let r = split_env_path();
            assert!(r.is_ok());

            let n = vec!["foo".to_string(), "bar".to_string()];
            assert_eq!(n, r.unwrap());
        }
    }
}
