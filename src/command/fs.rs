mod fs {
    use std::fs::{DirEntry, OpenOptions};
    use std::io::{Error, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    fn write_to_file(path: &str, content: &str, append: bool) -> Result<(), Error> {
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(!append)
            .append(append)
            .open(Path::new(path))?
            .write_all(content.as_bytes())?;

        Ok(())
    }

    fn is_executable_file(entry: &DirEntry) -> Result<bool, Error> {
        let r = entry.metadata()?;

        if r.is_dir() {
            Ok(false)
        } else {
            Ok(r.permissions().mode() & 0o111 != 0) // windows no
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::current_dir;
        use std::fs::read_dir;
        use std::io::Read;
        use std::time::{SystemTime, UNIX_EPOCH};

        #[test]
        fn test_write_to_file() {
            let path = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_millis();
            let path = format!("/tmp/{}", path);

            let r1 = write_to_file(path.as_str(), "Hello world!", false);
            assert_eq!((), r1.unwrap());
            let r2 = write_to_file(path.as_str(), "Good weather!", true);
            assert_eq!((), r2.unwrap());

            let mut file = OpenOptions::new()
                .read(true)
                .open(Path::new(&path))
                .unwrap();
            let mut r = String::new();
            file.read_to_string(&mut r).unwrap();
            assert_eq!("Hello world!Good weather!", r);
        }

        #[test]
        fn test_is_executable_file() {
            let mut path = current_dir().unwrap();
            path.push("test/fixture/fs/");
            let path = path.to_str().unwrap();
            let file = read_dir(path).unwrap().next().unwrap().unwrap();
            assert!(is_executable_file(&file).unwrap());
        }
    }
}
