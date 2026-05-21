use std::cmp::PartialEq;

pub fn is_redirect(arg: &str) -> bool {
    [">", "1>", "2>", ">>", "1>>", "2>>"].contains(&arg)
}

pub fn to_redirect(redirect: &str, path: &str) -> Option<Redirect> {
    if !is_redirect(redirect) {
        return None;
    }

    let (flow, mode) = parse_redirect(normalize_redirect(redirect).as_str());

    Some(Redirect::from(flow, mode, path.to_string()))
}

pub struct Redirect {
    flow: RedirectFlow,
    mode: RedirectMode,
    path: String,
}

impl Redirect {
    fn from(flow: RedirectFlow, mode: RedirectMode, path: String) -> Redirect {
        Redirect { flow, mode, path }
    }

    pub fn is_stderr(&self) -> bool {
        self.flow == RedirectFlow::Stderr
    }

    pub fn is_stdout(&self) -> bool {
        self.flow == RedirectFlow::Stdout
    }

    pub fn is_append(&self) -> bool {
        self.mode == RedirectMode::Append
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

enum RedirectFlow {
    Stdout,
    Stderr,
}

impl PartialEq for RedirectFlow {
    fn eq(&self, other: &RedirectFlow) -> bool {
        matches!(
            (self, other),
            (RedirectFlow::Stdout, RedirectFlow::Stdout)
                | (RedirectFlow::Stderr, RedirectFlow::Stderr)
        )
    }
}

enum RedirectMode {
    Rewrite,
    Append,
}

impl PartialEq for RedirectMode {
    fn eq(&self, other: &RedirectMode) -> bool {
        matches!(
            (self, other),
            (RedirectMode::Rewrite, RedirectMode::Rewrite)
                | (RedirectMode::Append, RedirectMode::Append)
        )
    }
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
        let redirect = to_redirect("2>", "path").unwrap();

        assert!(redirect.is_stderr());

        assert!(!redirect.is_append());

        assert_eq!("path", redirect.path());
    }

    #[test]
    fn test_normalize_redirect() {
        assert_eq!("1>".to_string(), normalize_redirect(">"));
    }

    #[test]
    fn test_parse_redirect() {
        let (flow, mode) = parse_redirect("2>>");

        assert!(flow == RedirectFlow::Stderr);
        assert!(mode == RedirectMode::Append);
    }
}
