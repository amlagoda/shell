pub fn is_redirect(arg: &str) -> bool {
    [">", "1>", "2>", ">>", "1>>", "2>>"].contains(&arg)
}

pub fn to_redirect(redirect: &str, path: &str) -> Redirect {
    let (flow, mode) = parse_redirect(normalize_redirect(redirect).as_str());

    Redirect {
        flow: flow,
        mode: mode,
        path: path.to_string(),
    }
}

#[derive(Debug)]
pub struct Redirect {
    flow: RedirectFlow,
    mode: RedirectMode,
    path: String,
}

#[derive(Debug)]
pub enum RedirectFlow {
    Stdout,
    Stderr,
}

#[derive(Debug)]
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

fn parse_redirect(redirect: &str) -> (RedirectFlow, RedirectMode) {
    let chars = redirect.chars().collect::<Vec<char>>();
    let is_stderr = chars.first().is_some_and(|r| r == &'2');
    let is_append = chars.len() == 3;

    let flow = if is_stderr {
        RedirectFlow::Stderr
    } else {
        RedirectFlow::Stdout
    };

    let mode = if is_append {
        RedirectMode::Append
    } else {
        RedirectMode::Rewrite
    };

    (flow, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_redirect() {
        assert!(is_redirect(">"));
    }

    #[test]
    fn test_to_redirect() {
        let r = to_redirect("2>", "path");

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
        assert_eq!("path", r.path);
    }

    #[test]
    fn test_normalize_redirect() {
        assert_eq!("1>".to_string(), normalize_redirect(">"));
    }

    #[test]
    fn test_parse_redirect() {
        let (flow, mode) = parse_redirect("2>>");

        let flow = match flow {
            RedirectFlow::Stdout => "stdout",
            RedirectFlow::Stderr => "stderr",
        };

        let mode = match mode {
            RedirectMode::Rewrite => "rewrite",
            RedirectMode::Append => "append",
        };

        assert_eq!("stderr", flow);
        assert_eq!("append", mode);
    }
}
