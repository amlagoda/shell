pub mod command {
    use std::env::{current_dir, home_dir, set_current_dir};
    use std::fs::read_dir;
    use std::io::{Error, ErrorKind};

    const COMMAND_TYPE: &str = "type";
    const COMMAND_ECHO: &str = "echo";
    const COMMAND_PWD: &str = "pwd";
    const COMMAND_CD: &str = "cd";
    const COMMAND_EXIT: &str = "exit";

    pub fn command(
        name: &str,
        args: Vec<String>,
        bin_paths: Vec<String>,
    ) -> (Option<String>, Option<String>, bool) {
        let mut output: Option<String> = None;
        let mut error: Option<String> = None;
        let mut is_exit = false;

        match name {
            /*COMMAND_TYPE => {
                let commands = Vec::from([
                    COMMAND_TYPE,
                    COMMAND_ECHO,
                    COMMAND_PWD,
                    COMMAND_CD,
                    COMMAND_EXIT,
                ]);

                match command_type(args, &commands) {
                    Ok(r) => output = Some(r),
                    Err(e) => error = Some(e.to_string()),
                }
            }*/
            COMMAND_ECHO => output = Some(command_echo(args)),

            COMMAND_PWD => match command_pwd() {
                Ok(r) => output = Some(r),
                Err(e) => error = Some(e.to_string()),
            },

            COMMAND_CD => {
                let arg = args.iter().next().map(|r| r.as_str()).unwrap_or("");

                match command_cd(arg) {
                    Ok(_) => output = None,
                    Err(e) => error = Some(e.to_string()),
                }
            }

            COMMAND_EXIT => is_exit = true,

            /*another => match command_from_env_path(another, args) {
                Ok(r) => match r {
                    Some(r) => [output, error] = r,
                    None => {}
                },
                Err(e) => error = Some(e.to_string()),
            },*/
            _ => {}
        }

        (output, error, is_exit)
    }

    // use crate::command::process::process::read_process_output;
    // use std::collections::VecDeque;
    // use std::io::{stdout, Error, ErrorKind, Read, Write};
    // use std::env::{current_dir, home_dir, set_current_dir, var, VarError};
    // use std::fs::{read_dir, DirEntry, OpenOptions, ReadDir};
    // use std::process::{Child, Command, ExitCode, Stdio};

    /*fn command_type(args: VecDeque<String>, commands: &Vec<&str>) -> Result<String, Error> {
        match args.iter().next() {
            Some(command) => {
                if commands.contains(&command.as_str()) {
                    return Ok(format!("{} is a shell builtin", command));
                }

                match search_command_in_env_path(&command) {
                    Ok(path) => match path {
                        Some(r) => Ok(format!("{} is {}", command, r)),
                        None => {
                            let msg = format!("{}: not found", command);
                            Err(Error::new(ErrorKind::NotFound, msg))
                        }
                    },
                    Err(e) => Err(e),
                }
            }
            None => Err(Error::new(ErrorKind::NotFound, ": not found")),
        }
    }*/

    fn command_echo(args: Vec<String>) -> String {
        args.iter()
            .map(|r| r.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
            .to_string()
    }

    fn command_pwd() -> Result<String, Error> {
        let e1 = Error::new(ErrorKind::NotFound, "pwd: Not found");
        let e2 = Error::new(ErrorKind::InvalidFilename, "pwd: Invalid file name");
        let path = current_dir().map_err(|_| e1)?;
        let path = path.to_str().ok_or(e2)?;

        Ok(path.to_string())
    }

    fn command_cd(path: &str) -> Result<(), Error> {
        let mut path = path.to_string();

        if path == "~" {
            path = home_dir()
                .ok_or(Error::new(ErrorKind::NotFound, "cd ~: Path is empty"))?
                .to_str()
                .ok_or(Error::new(
                    ErrorKind::InvalidFilename,
                    "cd ~: Path non-UTF-8",
                ))?
                .to_string();
        }

        let r = read_dir(path.as_str()); // exists, is dir, allowed
        if r.is_err() {
            let msg = format!("cd: {}: No such file or directory", path);
            return Err(Error::new(ErrorKind::NotFound, msg));
        }

        let r = set_current_dir(path.as_str());
        if r.is_err() {
            let msg = format!("cd: {}: {}", path.as_str(), r.unwrap_err().to_string());
            return Err(Error::new(ErrorKind::Other, msg));
        }

        Ok(())
    }

    /*fn command_from_env_path(
        command: &str,
        args: VecDeque<String>,
    ) -> Result<Option<[Option<String>; 2]>, Error> {
        match search_command_in_env_path(command) {
            Ok(path) => match path {
                Some(_) => {
                    let mut process = Command::new(command);

                    for arg in args {
                        process.arg(arg);
                    }

                    match process
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                    {
                        Ok(mut r) => match r.wait() {
                            Ok(_) => match read_process_output(r) {
                                Ok(r) => Ok(Some(r)),
                                Err(e) => Err(e),
                            },
                            Err(e) => Err(e),
                        },
                        Err(e) => Err(e),
                    }
                }
                None => {
                    let msg = format!("{}: not found", command);
                    Err(Error::new(ErrorKind::NotFound, msg))
                }
            },
            Err(e) => Err(e),
        }
    }*/

    /*fn match_command_and_file(command: &str, entry: &DirEntry) -> Result<Option<String>, Error> {
        match is_executable_file(entry) {
            Ok(is_exe) => {
                if !is_exe {
                    return Ok(None);
                }

                let file_name = match entry.file_name().into_string() {
                    Ok(r) => r,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidFilename, "invalid file name")),
                };

                if command != file_name {
                    return Ok(None);
                }

                match entry.path().to_str() {
                    Some(r) => Ok(Some(String::from(r))),
                    None => Ok(None),
                }
            }
            Err(e) => Err(e),
        }
    }*/

    /*fn search_command_in_dir(command: &str, dir: &mut ReadDir) -> Option<String> {
        for entry in dir {
            match entry {
                Ok(r) => match match_command_and_file(command, &r) {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_command_cd() {
            assert_eq!((), command_cd("/tmp").unwrap());

            assert_eq!(
                "cd: fake: No such file or directory".to_string(),
                command_cd("fake").unwrap_err().to_string()
            );
        }

        #[test]
        fn test_command_pwd() {
            assert_eq!(
                current_dir().unwrap().to_str().unwrap().to_string(),
                command_pwd().unwrap()
            );
        }

        #[test]
        fn test_command_echo() {
            assert_eq!(
                "foo bar",
                command_echo(vec!("foo".to_string(), "bar".to_string()))
            );
        }
    }
}
