pub fn is_pipeline(arg: &str) -> bool {
    ["|", "|&"].contains(&arg)
}

pub fn to_pipeline(pipeline: &str) -> Pipeline {
    if pipeline == "|&" {
        Pipeline::StdoutStderr
    } else {
        Pipeline::Stdout
    }
}

pub enum Pipeline {
    Stdout,
    StdoutStderr,
}

impl Pipeline {
    pub fn is_stdout(&self) -> bool {
        match self {
            Pipeline::Stdout => true,
            Pipeline::StdoutStderr => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pipeline() {
        assert!(is_pipeline("|&"));
    }

    #[test]
    fn test_to_pipeline() {
        assert!(!to_pipeline("|&").is_stdout());
    }
}
