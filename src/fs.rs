pub mod fs {
    use std::fs::{read_dir, DirEntry, OpenOptions, ReadDir};
    use std::io::{Error, Write};
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    pub fn write_to_file(path: &str, content: &str, append: bool) -> Result<(), Error> {
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

    pub fn search_executable_file_in_paths(name: &str, paths: &Vec<&str>) -> Option<String> {
        for path in paths {
            let dir = read_dir(path);

            if dir.is_err() {
                // errors remains here
                // because we need to go down the list
                continue;
            }

            let r = search_executable_file_in_dir(name, dir.unwrap());

            if r.is_some() {
                return r;
            }
        }

        None
    }

    fn search_executable_file_in_dir(name: &str, dir: ReadDir) -> Option<String> {
        for entry in dir {
            if entry.is_err() {
                // errors remains here
                // because we need to go down the list
                continue;
            };

            let r = match_name_and_executable_file(name, &entry.unwrap());

            if r.is_some() {
                return r;
            }
        }

        None
    }

    fn match_name_and_executable_file(name: &str, entry: &DirEntry) -> Option<String> {
        if !is_executable_file(entry).ok()? {
            return None;
        }

        if name != entry.file_name().into_string().ok()? {
            return None;
        }

        entry.path().to_str().map(|r| Some(r.to_string()))?
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
        use std::io::Read;
        use std::time::{SystemTime, UNIX_EPOCH};

        #[test]
        fn test_search_executable_file_in_paths() {
            let r = get_fixture_dir();
            let paths = vec![r.as_str()];

            let r = search_executable_file_in_paths("exe", &paths).unwrap();
            assert_eq!(format!("{}exe", get_fixture_dir()), r);

            let r = search_executable_file_in_paths("not_exe", &paths);
            assert!(r.is_none());

            let r = search_executable_file_in_paths("not_exists", &paths);
            assert!(r.is_none());
        }

        #[test]
        fn test_write_to_file() {
            let name = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_millis();
            let path = format!("/tmp/{}", name);

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

        fn get_fixture_dir() -> String {
            // ends with a slash
            format!("{}/test/fixture/fs/", get_current_dir())
        }

        fn get_current_dir() -> String {
            // does not end with a slash
            current_dir().unwrap().to_str().unwrap().to_string()
        }
    }
}
