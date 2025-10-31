pub mod process {
    use std::io::{Error, Read};
    use std::process::Child;

    pub fn read_process_output(process: Child) -> Result<[Option<String>; 2], Error> {
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

        Ok([stderr, stdout])
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::current_dir;
        use std::process::{Command, Stdio};

        #[test]
        fn test_read_process_output() {
            let mut process = Command::new("ls")
                .arg("notexist")
                .arg(get_fixture_dir())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();

            process.wait().unwrap();

            let [stderr, stdout] = read_process_output(process).unwrap();

            assert_eq!("ls: notexist: No such file or directory", stderr.unwrap());
            assert_eq!(format!("{}:\nfile", get_fixture_dir()), stdout.unwrap());
        }

        fn get_fixture_dir() -> String {
            // ends with a slash
            format!("{}/test/fixture/command/process/", get_current_dir())
        }

        fn get_current_dir() -> String {
            // does not end with a slash
            current_dir().unwrap().to_str().unwrap().to_string()
        }
    }
}
