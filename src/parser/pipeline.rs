pub fn is_pipeline(arg: &str) -> bool {
    ["|", "|&"].contains(&arg)
}

pub fn to_pipeline(pipeline: &str) -> Option<Pipeline> {
    if !is_pipeline(pipeline) {
        return None;
    }

    if pipeline == "|&" {
        Some(Pipeline::StdoutStderr)
    } else {
        Some(Pipeline::Stdout)
    }
}

pub enum Pipeline {
    Stdout,
    StdoutStderr,
}

impl Pipeline {
    #[cfg(test)]
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
        assert!(!to_pipeline("|&").unwrap().is_stdout());
    }
}
