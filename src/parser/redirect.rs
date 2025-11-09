pub fn is_redirect(arg: &str) -> bool {
    [">", "1>", "2>", ">>", "1>>", "2>>"].contains(&arg)
}

pub fn normalize_and_parse_redirect(arg: &str) -> Redirect {
    parse_redirect(&normalize_redirect(arg))
}

pub struct Redirect {
    flow: RedirectFlow,
    mode: RedirectMode,
}

pub enum RedirectFlow {
    Stdout,
    Stderr,
}

pub enum RedirectMode {
    Rewrite,
    Append,
}

fn normalize_redirect(redirect: &str) -> String {
    if [">", ">>"].contains(&redirect) {
        format!("1{}", redirect)
    } else {
        redirect.to_string()
    }
}

fn parse_redirect(redirect: &str) -> Redirect {
    let chars = redirect.chars().collect::<Vec<char>>();
    let is_stderr = chars.first().is_some_and(|r| r == &'2');
    let is_append = chars.len() == 3;

    Redirect {
        flow: if is_stderr {
            RedirectFlow::Stderr
        } else {
            RedirectFlow::Stdout
        },
        mode: if is_append {
            RedirectMode::Append
        } else {
            RedirectMode::Rewrite
        },
    }
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
        let r = normalize_and_parse_redirect("2>");

        let flow = match r.flow {
            RedirectFlow::Stdout => "stdout",
            RedirectFlow::Stderr => "stderr",
        };

        let mode = match r.mode {
            RedirectMode::Rewrite => "rewrite",
            RedirectMode::Append => "append",
        };

        assert_eq!("stderr", flow);
        assert_eq!("rewrite", mode);
    }

    #[test]
    fn test_normalize_redirect() {
        assert_eq!("1>".to_string(), normalize_redirect(">"));
    }

    #[test]
    fn test_parse_redirect() {
        let r = parse_redirect("2>>");

        let flow = match r.flow {
            RedirectFlow::Stdout => "stdout",
            RedirectFlow::Stderr => "stderr",
        };

        let mode = match r.mode {
            RedirectMode::Rewrite => "rewrite",
            RedirectMode::Append => "append",
        };

        assert_eq!("stderr", flow);
        assert_eq!("append", mode);
    }
}
