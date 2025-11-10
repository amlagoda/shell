pub fn is_pipeline(arg: &str) -> bool {
    ["|", "|&"].contains(&arg)
}

pub fn to_pipeline(pipeline: &str, input: String) -> Pipeline {
    Pipeline {
        flow: pipeline_flow(pipeline),
        input,
    }
}

struct Pipeline {
    flow: PipelineFlow,
    input: String,
}

impl Pipeline {
    fn new(flow: PipelineFlow, input: String) -> Pipeline {
        Pipeline { flow, input }
    }

    fn is_stdout(&self) -> bool {
        self.flow.is_stdout()
    }

    fn input(&self) -> &str {
        self.input.as_str()
    }
}

enum PipelineFlow {
    Stdout,
    StdoutStderr,
}

impl PipelineFlow {
    fn is_stdout(&self) -> bool {
        match self {
            PipelineFlow::Stdout => true,
            PipelineFlow::StdoutStderr => false,
        }
    }
}

fn pipeline_flow(pipeline: &str) -> PipelineFlow {
    if pipeline == "|&" {
        PipelineFlow::StdoutStderr
    } else {
        PipelineFlow::Stdout
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
    fn test_pipeline_flow() {
        assert!(!pipeline_flow("|&").is_stdout());
    }
}
