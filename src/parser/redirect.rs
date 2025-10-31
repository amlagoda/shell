pub fn is_redirect(arg: &str) -> bool {
    [">", "1>", "2>", ">>", "1>>", "2>>"].contains(&arg)
}

pub fn normalize_and_parse_redirect(arg: &str) -> [String; 2] {
    parse_redirect(&normalize_redirect(arg))
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
        if flow.is_empty() {
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
    fn test_normalize_and_parse_redirect() {
        assert_eq!(
            ["2", ">"].map(|r| r.to_string()),
            normalize_and_parse_redirect("2>")
        )
    }

    #[test]
    fn test_normalize_redirect() {
        assert_eq!("1>".to_string(), normalize_redirect(">"));
    }

    #[test]
    fn test_parse_redirect() {
        assert_eq!(["2", ">>"].map(|r| r.to_string()), parse_redirect("2>>"));
    }
}
