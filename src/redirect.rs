mod redirect {
    fn is_redirect(arg: &str) -> bool {
        [">", "1>", "2>", ">>", "1>>", "2>>"].contains(&arg)
    }

    fn normalize_redirect(redirect: &str) -> String {
        if [">", ">>"].contains(&redirect) {
            format!("1{}", redirect)
        } else {
            redirect.to_string()
        }
    }

    fn parse_redirect(redirect: &str) -> [String; 2] {
        let mut flow = String::new();
        let mut mode = String::new();

        for r in redirect.chars() {
            if flow.len() == 0 {
                flow.push(r);
            } else {
                mode.push(r);
            }
        }

        [flow, mode]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_redirect() {
            assert!(is_redirect(">"));
        }

        #[test]
        fn test_normalize_redirect() {
            assert_eq!("1>".to_string(), normalize_redirect(">"));
        }

        #[test]
        fn test_parse_redirect() {
            assert_eq!(["2".to_string(), ">>".to_string()], parse_redirect("2>>"));
        }
    }
}
