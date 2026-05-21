use std::cmp::PartialEq;

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

impl PartialEq for Pipeline {
    fn eq(&self, other: &Pipeline) -> bool {
        matches!(
            (self, other),
            (Pipeline::Stdout, Pipeline::Stdout) | (Pipeline::StdoutStderr, Pipeline::StdoutStderr)
        )
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
        assert!(to_pipeline("|&").unwrap() == Pipeline::StdoutStderr);
    }
}
