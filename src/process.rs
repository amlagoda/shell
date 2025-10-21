mod process {
    use std::io::{Error, Read};
    use std::process::Child;

    fn read_process_output(process: Child) -> Result<[Option<String>; 2], Error> {
        let mut stderr = None;
        let mut stdout = None;

        if process.stderr.is_some() {
            let mut r = process.stderr.unwrap();
            let mut output = String::new();
            let r = r.read_to_string(&mut output); // take?

            if r.is_err() {
                return Err(r.unwrap_err());
            }

            if output.len() > 0 {
                stderr = Some(output.trim().to_string());
            }
        }

        if process.stdout.is_some() {
            let mut r = process.stdout.unwrap();
            let mut output = String::new();
            let r = r.read_to_string(&mut output); // take?

            if r.is_err() {
                return Err(r.unwrap_err());
            }

            if output.len() > 0 {
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
            let mut process = Command::new("ls");
            process.stdout(Stdio::piped());
            process.stderr(Stdio::piped());
            process.arg("fake_path");

            let mut path = current_dir().unwrap();
            path.push("test/fixture/process/");
            let path = path.to_str().unwrap();

            process.arg(path);

            let mut process = process.spawn().unwrap();
            process.wait().unwrap();

            let r = [
                Some("ls: fake_path: No such file or directory".to_string()),
                Some(format!("{}:\nfile.txt", path)),
            ];
            assert_eq!(r, read_process_output(process).unwrap());
        }
    }
}
