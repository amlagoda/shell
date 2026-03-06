pub fn is_redirect(arg: &str) -> bool {
    [">", "1>", "2>", ">>", "1>>", "2>>"].contains(&arg)
}

pub fn to_redirect(redirect: &str, path: &str) -> Redirect {
    let (flow, mode) = parse_redirect(normalize_redirect(redirect).as_str());

    Redirect::new(flow, mode, path.to_string())
}

pub struct Redirect {
    flow: RedirectFlow,
    mode: RedirectMode,
    path: String,
}

impl Redirect {
    fn new(flow: RedirectFlow, mode: RedirectMode, path: String) -> Redirect {
        Redirect { flow, mode, path }
    }

    pub fn is_stderr(&self) -> bool {
        self.flow.is_stderr()
    }

    pub fn is_stdout(&self) -> bool {
        self.flow.is_stdout()
    }

    pub fn is_append(&self) -> bool {
        self.mode.is_append()
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

enum RedirectFlow {
    Stdout,
    Stderr,
}

impl RedirectFlow {
    fn is_stderr(&self) -> bool {
        match self {
            RedirectFlow::Stdout => false,
            RedirectFlow::Stderr => true,
        }
    }

    fn is_stdout(&self) -> bool {
        match self {
            RedirectFlow::Stdout => true,
            RedirectFlow::Stderr => false,
        }
    }
}

enum RedirectMode {
    Rewrite,
    Append,
}

impl RedirectMode {
    fn is_append(&self) -> bool {
        match self {
            RedirectMode::Rewrite => false,
            RedirectMode::Append => true,
        }
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
        let redirect = to_redirect("2>", "path");

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

        assert!(flow.is_stderr());

        assert!(mode.is_append());
    }
}
