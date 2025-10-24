mod fs;
mod process;

pub mod command {
    use crate::command::fs::fs::search_executable_file_in_paths;
    use crate::command::process::process::read_process_output;
    use std::env::{current_dir, home_dir, set_current_dir};
    use std::fs::read_dir;
    use std::io::{Error, ErrorKind};
    use std::process::{Command, Stdio};

    const COMMAND_TYPE: &str = "type";
    const COMMAND_ECHO: &str = "echo";
    const COMMAND_PWD: &str = "pwd";
    const COMMAND_CD: &str = "cd";
    const COMMAND_EXIT: &str = "exit";

    pub fn command(
        name: &str,
        args: &Vec<&str>,
        bin_paths: &Vec<&str>,
    ) -> (Option<String>, Option<String>, bool) {
        let mut output: Option<String> = None;
        let mut error: Option<String> = None;
        let mut is_exit = false;

        match name {
            COMMAND_TYPE => {
                let commands = vec![
                    COMMAND_TYPE,
                    COMMAND_ECHO,
                    COMMAND_PWD,
                    COMMAND_CD,
                    COMMAND_EXIT,
                ];

                let command = *args.iter().next().unwrap_or(&"");

                match command_type(command, &commands, bin_paths) {
                    Ok(r) => output = Some(r),
                    Err(e) => error = Some(e.to_string()),
                }
            }

            COMMAND_ECHO => output = Some(command_echo(args)),

            COMMAND_PWD => match command_pwd() {
                Ok(r) => output = Some(r),
                Err(e) => error = Some(e.to_string()),
            },

            COMMAND_CD => {
                let path = *args.iter().next().unwrap_or(&"");

                match command_cd(path) {
                    Ok(_) => {}
                    Err(e) => error = Some(e.to_string()),
                }
            }

            COMMAND_EXIT => is_exit = true,

            another => match command_from_paths(another, args, bin_paths) {
                Ok(r) => [error, output] = r,
                Err(e) => error = Some(e.to_string()),
            },
        }

        (output, error, is_exit)
    }

    fn command_type(
        command: &str,
        commands: &Vec<&str>,
        paths: &Vec<&str>,
    ) -> Result<String, Error> {
        if commands.contains(&command) {
            return Ok(format!("{} is a shell builtin", command));
        }

        let r = search_executable_file_in_paths(command, paths);

        if r.is_some() {
            return Ok(format!("{} is {}", command, r.unwrap()));
        }

        let msg = format!("{}: not found", command);
        Err(Error::new(ErrorKind::NotFound, msg))
    }

    fn command_echo(args: &Vec<&str>) -> String {
        args.iter()
            .map(|r| *r)
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
            let e1 = Error::new(ErrorKind::NotFound, "cd ~: Path is empty");
            let e2 = Error::new(ErrorKind::InvalidFilename, "cd ~: Path non-UTF-8");

            path = home_dir().ok_or(e1)?.to_str().ok_or(e2)?.to_string();
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

    fn command_from_paths(
        command: &str,
        args: &Vec<&str>,
        paths: &Vec<&str>,
    ) -> Result<[Option<String>; 2], Error> {
        if search_executable_file_in_paths(command, paths).is_none() {
            let msg = format!("{}: not found", command);
            return Err(Error::new(ErrorKind::NotFound, msg));
        }

        let mut process = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        process.wait()?;

        read_process_output(process)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::env::env::split_env_path;

        #[test]
        fn test_command_from_paths() {
            let path = get_fixture_dir();
            let args = vec!["not_exists", path.as_str()];

            let r = split_env_path().unwrap();
            let paths = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

            let [stderr, stdout] = command_from_paths("ls", &args, &paths).unwrap();

            let msg = "ls: not_exists: No such file or directory";
            assert_eq!(msg, stderr.unwrap());

            let msg = format!("{}:\nexe\nnot_exe", path);
            assert_eq!(msg, stdout.unwrap());
        }

        #[test]
        fn test_command_type() {
            let path = get_fixture_dir();
            let paths = vec![path.as_str()];

            let r = command_type("not_exe", &vec![], &paths);
            assert_eq!("not_exe: not found", r.unwrap_err().to_string());

            let r = command_type("exe", &vec!["exe"], &paths);
            assert_eq!("exe is a shell builtin", r.unwrap());

            let r = command_type("exe", &vec![], &paths);
            assert_eq!(format!("exe is {}exe", path), r.unwrap());
        }

        #[test]
        fn test_command_echo() {
            assert_eq!("foo bar", command_echo(&Vec::from(["foo", "bar"])));
        }

        // we are not testing the command_cd because
        // it affects the global state

        #[test]
        fn test_command_pwd() {
            assert_eq!(get_current_dir(), command_pwd().unwrap());
        }

        fn get_fixture_dir() -> String {
            // ends with a slash
            format!("{}/test/fixture/command/mod/", get_current_dir())
        }

        fn get_current_dir() -> String {
            // does not end with a slash
            current_dir().unwrap().to_str().unwrap().to_string()
        }
    }
}
