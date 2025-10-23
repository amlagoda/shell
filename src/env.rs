mod env {
    use std::env::{var, VarError};

    fn split_env_path() -> Result<Vec<String>, VarError> {
        Ok(var("PATH")?
            .split(':')
            .map(|r| String::from(r))
            .collect::<Vec<String>>())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::set_var;

        #[test]
        fn test_split_env_path() {
            let initial = var("PATH").unwrap();

            set_var("PATH", "foo:bar");
            assert_eq!(
                vec!["foo".to_string(), "bar".to_string()],
                split_env_path().unwrap()
            );
            set_var("PATH", initial);
        }
    }
}
