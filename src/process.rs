use std::io::{Error, PipeReader, PipeWriter};
use std::process::{Child, Command as Process, Stdio};

pub fn spawn_pipe_process(
    bin: &str,
    args: Option<&Vec<&str>>,
    stdin: Option<PipeReader>,
    stdout: Option<PipeWriter>,
    stderr: Option<PipeWriter>,
) -> Result<Child, Error> {
    build_process(bin, args, stdin, stdout, stderr).spawn()
}

pub fn spawn_process(bin: &str, args: Option<&Vec<&str>>) -> Result<Child, Error> {
    build_process(bin, args, None, None, None).spawn()
}

fn build_process(
    bin: &str,
    args: Option<&Vec<&str>>,
    stdin: Option<PipeReader>,
    stdout: Option<PipeWriter>,
    stderr: Option<PipeWriter>,
) -> Process {
    let mut process = Process::new(bin);

    process.stdin(Stdio::piped());
    process.stdout(Stdio::piped());
    process.stderr(Stdio::piped());

    if let Some(args) = args {
        process.args(args);
    }

    if let Some(stdin) = stdin {
        process.stdin(stdin);
    }

    if let Some(stdout) = stdout {
        process.stdout(stdout);
    }

    if let Some(stderr) = stderr {
        process.stderr(stderr);
    }

    process
}
