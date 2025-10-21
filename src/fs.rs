mod fs {
    use std::fs::{DirEntry, OpenOptions};
    use std::io::{Error, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    fn write_to_file(path: &str, content: &str, append: bool) -> Result<(), Error> {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(!append)
            .append(append)
            .open(Path::new(path));

        match file {
            Ok(mut r) => match r.write_all(content.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn is_executable_file(entry: &DirEntry) -> Result<bool, Error> {
        match entry.metadata() {
            Ok(md) => {
                if md.is_dir() {
                    Ok(false)
                } else {
                    Ok(md.permissions().mode() & 0o111 != 0) // windows no
                }
            }
            Err(e) => Err(e),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::current_dir;
        use std::fs::read_dir;

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
