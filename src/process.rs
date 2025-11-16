use std::io::{Error, Read};
use std::process::{Child, Command, Stdio};

pub fn new_process(command: &str, args: &Vec<&str>) -> Result<ProcessOutput, Error> {
    let mut process = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    process.wait()?;

    read_process_output(process)
}

pub struct ProcessOutput {
    stdout: Option<String>,
    stderr: Option<String>,
}

impl ProcessOutput {
    fn new(stdout: Option<String>, stderr: Option<String>) -> ProcessOutput {
        ProcessOutput { stdout, stderr }
    }

    pub fn stdout(&self) -> Option<&str> {
        self.stdout.as_ref().map(|r| r.as_str())
    }

    pub fn stderr(&self) -> Option<&str> {
        self.stderr.as_ref().map(|r| r.as_str())
    }
}

fn read_process_output(process: Child) -> Result<ProcessOutput, Error> {
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

    Ok(ProcessOutput::new(stdout, stderr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_new_process() {
        let r = get_fixture_dir();
        let process = new_process("ls", &vec!["notexists", r.as_str()]).unwrap();

        assert_eq!(
            "ls: notexists: No such file or directory",
            process.stderr().unwrap()
        );
        assert_eq!(format!("{}:\nfile", r), process.stdout().unwrap());
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
