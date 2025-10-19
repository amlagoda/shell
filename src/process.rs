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

            let path = current_dir();
            assert!(path.is_ok());
            let mut path = path.unwrap();
            path.push("test/fixture/process/");
            let path = path.to_str();
            assert!(path.is_some());
            let path = path.unwrap();

            process.arg(path);

            let process = process.spawn();
            assert!(process.is_ok());
            let mut process = process.unwrap();
            let r = process.wait();
            assert!(r.is_ok());

            let r = [
                Some("ls: fake_path: No such file or directory".to_string()),
                Some(format!("{}:\nfile.txt", path)),
            ];
            let output = read_process_output(process);
            assert!(output.is_ok());
            assert_eq!(r, output.unwrap());
        }
    }
}
