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
                let commands = Vec::from([
                    COMMAND_TYPE,
                    COMMAND_ECHO,
                    COMMAND_PWD,
                    COMMAND_CD,
                    COMMAND_EXIT,
                ]);

                let command = *args.iter().next().unwrap_or(&"");
                output = command_type(command, &commands, &bin_paths);
            }

            COMMAND_ECHO => output = Some(command_echo(args)),

            COMMAND_PWD => match command_pwd() {
                Ok(r) => output = Some(r),
                Err(e) => error = Some(e.to_string()),
            },

            COMMAND_CD => {
                let path = *args.iter().next().unwrap_or(&"");

                match command_cd(path) {
                    Ok(_) => output = None,
                    Err(e) => error = Some(e.to_string()),
                }
            }

            COMMAND_EXIT => is_exit = true,

            another => {
                let args = args.iter().map(|r| *r).collect();

                match command_from_paths(another, &args, &bin_paths) {
                    Ok(r) => [output, error] = r,
                    Err(e) => error = Some(e.to_string()),
                }
            }
        }

        (output, error, is_exit)
    }

    fn command_type(command: &str, commands: &Vec<&str>, paths: &Vec<&str>) -> Option<String> {
        if commands.contains(&command) {
            return Some(format!("{} is a shell builtin", command));
        }

        let r = search_executable_file_in_paths(command, paths);

        if r.is_some() {
            return Some(format!("{} is {}", command, r.unwrap()));
        }

        Some(format!("{}: not found", command))
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
        use std::path::Path;
        use std::env::var;

        /*#[test]
        fn test_command_from_paths() {
            let paths = var("PATH").unwrap();
            let paths = paths.split(':').collect::<Vec<&str>>();

            let r = command_from_paths("nonexists", &vec!(), &paths);
            assert_eq!("nonexists: not found", r.unwrap_err().to_string());

            let [output, error] = command_from_paths(
                "ls",
                &vec!("executable", "nonexists"),
                &paths
            ).unwrap();

            //assert_eq!("", output.unwrap());
            //assert_eq!("", error.unwrap());
        }*/

        //#[test]
        /*fn test_command_type() {
            let commands = vec!["pwd"];
            let r = get_fixture_dir();
            let paths = vec![r.as_str()];

            assert_eq!(
                "pwd is a shell builtin",
                command_type("pwd", &commands, &paths).unwrap()
            );

            assert_eq!(
                format!("executable is {}", format!("{}executable", r)),
                command_type("executable", &commands, &paths).unwrap()
            );

            assert_eq!(
                "another: not found",
                command_type("another", &commands, &paths).unwrap()
            );

            assert_eq!(": not found", command_type("", &commands, &paths).unwrap());
        }*/

        /*#[test]
        fn test_command_cd_and_command_pwd() {
            let initial_dir = get_current_dir();

            assert_eq!(initial_dir, command_pwd().unwrap());

            assert_eq!((), command_cd("/private/tmp").unwrap());
            assert_eq!("/private/tmp", command_pwd().unwrap());

            set_current_dir(Path::new(initial_dir.as_str())).unwrap();

            assert_eq!(
                "cd: fake: No such file or directory".to_string(),
                command_cd("fake").unwrap_err().to_string()
            );
        }*/

        /*#[test]
        fn test_command_echo() {
            assert_eq!(
                "foo bar",
                command_echo(vec!("foo".to_string(), "bar".to_string()))
            );
        }

        fn get_fixture_dir() -> String {
            // ends with a slash
            format!("{}/test/fixture/command/", get_current_dir())
        }

        fn get_current_dir() -> String {
            // does not end with a slash
            current_dir().unwrap().to_str().unwrap().to_string()
        }*/
    }
}
