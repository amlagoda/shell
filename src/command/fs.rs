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

    /*fn search_command_in_env_path(command: &str) -> Result<Option<String>, Error> {
        match split_env_path() {
            Ok(paths) => {
                for path in paths {
                    match read_dir(path) {
                        Ok(mut r) => match search_command_in_dir(command, &mut r) {
                            Some(r) => return Ok(Some(r)),
                            None => continue,
                        },
                        // path not exists, is not dir and permissions errors
                        // remain here because we need to go down the list
                        Err(_) => continue,
                    }
                }

                Ok(None)
            }
            Err(e) => Err(Error::new(ErrorKind::Interrupted, e)),
        }
    }*/

    /*fn search_command_in_dir(command: &str, dir: &mut ReadDir) -> Option<String> {
        for entry in dir {
            match entry {
                Ok(r) => match match_name_and_executable_file(command, &r) {
                    Ok(path) => match path {
                        Some(r) => return Some(r),
                        None => continue,
                    },
                    // read file metadata error and
                    // file name not unicode error remains here
                    // because we need to go down the list
                    Err(_) => continue,
                },
                // fetching the next entry error remain here
                // because we need to go down the list
                Err(_) => continue,
            }
        }

        None
    }*/

    fn match_name_and_executable_file(command: &str, file: &DirEntry) -> Option<String> {
        if !is_executable_file(file).ok()? {
            return None;
        }

        if command != file.file_name().into_string().ok()? {
            return None;
        }

        file.path().to_str().map(|r| Some(r.to_string()))?
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::current_dir;
        use std::fs::read_dir;
        use std::io::Read;
        use std::time::{SystemTime, UNIX_EPOCH};

        #[test]
        fn test_match_name_and_executable_file() {
            let mut path = get_current_dir();
            path.push_str("/test/fixture/fs/");

            for r in read_dir(Path::new(path.as_str())).unwrap() {
                let file = r.unwrap();
                let file_name = file.file_name().into_string().unwrap();

                if file_name == "executable" {
                    let r = match_name_and_executable_file("executable", &file).unwrap();
                    assert_eq!(file.path().to_str().unwrap(), r);

                    let r = match_name_and_executable_file("another", &file);
                    assert_eq!(None, r);
                }

                if file_name == "non_executable" {
                    let r = match_name_and_executable_file("non_executable", &file);
                    assert_eq!(None, r);
                }
            }
        }

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

        fn get_current_dir() -> String {
            current_dir().unwrap().to_str().unwrap().to_string()
        }
    }
}
