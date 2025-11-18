use std::io::{Error, Read};
use std::process::{Child, Command, Stdio};

pub fn run_process(command: &str, args: &Vec<&str>) -> Result<ProcessResult, Error> {
    let mut process = Command::new(command)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    process.wait()?;

    to_result(process)
}

pub struct ProcessResult {
    stderr: Option<String>,
    stdout: Option<String>,
}

impl ProcessResult {
    pub fn stderr(&self) -> Option<&str> {
        self.stderr.as_ref().map(|r| r.as_str())
    }

    pub fn stdout(&self) -> Option<&str> {
        self.stdout.as_ref().map(|r| r.as_str())
    }
}

fn to_result(process: Child) -> Result<ProcessResult, Error> {
    let mut stderr = None;
    let mut stdout = None;

    if let Some(mut r) = process.stderr {
        let mut output = String::new();
        r.read_to_string(&mut output)?;

        if !output.is_empty() {
            stderr = Some(output.trim().to_string());
        }
    }

    if let Some(mut r) = process.stdout {
        let mut output = String::new();
        r.read_to_string(&mut output)?;

        if !output.is_empty() {
            stdout = Some(output.trim().to_string());
        }
    }

    Ok(ProcessResult { stderr, stdout })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_run_process() {
        let path = get_fixture_dir();
        let result = run_process("ls", &vec!["notexists", path.as_str()]).unwrap();

        assert_eq!(
            "ls: notexists: No such file or directory",
            result.stderr().unwrap()
        );

        assert_eq!(
            format!("{}:\nfile", path.as_str()),
            result.stdout().unwrap()
        );
    }

    fn get_fixture_dir() -> String {
        // ends with a slash
        format!("{}/test/fixture/process/", get_current_dir())
    }

    fn get_current_dir() -> String {
        // does not end with a slash
        current_dir().unwrap().to_str().unwrap().to_string()
    }
}
