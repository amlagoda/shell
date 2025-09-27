use std::env::{current_dir, home_dir, set_current_dir, var, VarError};
use std::fs::{read_dir, DirEntry, ReadDir};
use std::io::{stdin, stdout, Write};
use std::io::{Error, ErrorKind, Read};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, ExitCode, Stdio};
use std::str::SplitWhitespace;

fn main() -> ExitCode {
    const COMMAND_TYPE: &str = "type";
    const COMMAND_ECHO: &str = "echo";
    const COMMAND_PWD: &str = "pwd";
    const COMMAND_CD: &str = "cd";
    const COMMAND_EXIT: &str = "exit";

    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");

        match stdout().flush() {
            Ok(_) => {}
            Err(_) => return ExitCode::FAILURE,
        }

        match stdin().read_line(&mut input) {
            Ok(_) => {
                let input: &str = input.trim();

                if input == "exit 0" {
                    break;
                }

                let mut iter = input.split_whitespace();
                let mut output = format!("{}: command not found", input); // for empty input

                match iter.next() {
                    Some(command) => match command {
                        COMMAND_TYPE => {
                            output = command_type(
                                iter,
                                &Vec::from([
                                    COMMAND_TYPE,
                                    COMMAND_ECHO,
                                    COMMAND_PWD,
                                    COMMAND_CD,
                                    COMMAND_EXIT,
                                ]),
                            );
                        }
                        COMMAND_ECHO => {
                            output = command_echo(iter);
                        }
                        COMMAND_PWD => {
                            output = command_pwd(command);
                        }
                        COMMAND_CD => {
                            output = command_cd(command, iter);
                        }
                        another => {
                            output = command_from_env_path(another, iter);
                        }
                    },
                    None => {}
                }

                if output.len() > 0 {
                    println!("{}", output);
                }
            }
            Err(_) => return ExitCode::FAILURE,
        }
    }

    ExitCode::SUCCESS
}

fn command_from_env_path(name: &str, args: SplitWhitespace) -> String {
    match search_command_in_env_path(name) {
        Ok(path) => match path {
            Some(_) => {
                let mut command = Command::new(name);

                for arg in args {
                    command.arg(arg);
                }

                match command.stdout(Stdio::piped()).spawn() {
                    Ok(command) => {
                        // take?
                        match command.stdout {
                            Some(mut r) => {
                                let mut output = String::new();

                                match r.read_to_string(&mut output) {
                                    Ok(_) => String::from(output.as_str().trim()),
                                    Err(_) => format!("{}: failed to run command", name),
                                }
                            }
                            None => String::new(),
                        }
                    }
                    Err(_) => format!("{}: failed to run command", name),
                }
            }
            None => format!("{}: command not found", name),
        },
        Err(_) => format!("{}: command not found", name),
    }
}

fn command_cd(name: &str, mut args: SplitWhitespace) -> String {
    let mut path = match args.next() {
        Some(r) => String::from(r),
        None => String::new(),
    };

    if path == "~" {
        path = match home_dir() {
            Some(path) => match path.to_str() {
                Some(r) => String::from(r),
                None => String::new(),
            },
            None => String::new(),
        };
    }

    if !is_allowed_dir(&path) {
        return format!("{}: {}: No such file or directory", name, path);
    }

    match set_current_dir(&path) {
        Ok(_) => String::new(),
        Err(_) => format!("{}: failed to run command", name),
    }
}

fn command_pwd(name: &str) -> String {
    match current_dir() {
        Ok(path) => match path.to_str() {
            Some(r) => String::from(r),
            None => format!("{}: failed to run command", name),
        },
        Err(_) => format!("{}: failed to run command", name),
    }
}

fn command_type(mut args: SplitWhitespace, commands: &Vec<&str>) -> String {
    match args.next() {
        Some(command) => {
            if commands.contains(&command) {
                return format!("{} is a shell builtin", command);
            }

            match search_command_in_env_path(&command) {
                Ok(path) => match path {
                    Some(r) => format!("{} is {}", command, r),
                    None => format!("{}: not found", command),
                },
                Err(_) => format!("{}: not found", command),
            }
        }
        None => String::from(": not found"),
    }
}

fn command_echo(args: SplitWhitespace) -> String {
    format!("{}", args.collect::<Vec<&str>>().join(" "))
}

fn split_env_path() -> Result<Vec<String>, VarError> {
    match var("PATH") {
        Ok(env) => {
            let paths = env
                .split(':')
                .map(|path| String::from(path))
                .collect::<Vec<String>>();
            Ok(paths)
        }
        Err(e) => Err(e),
    }
}

fn search_command_in_env_path(command: &str) -> Result<Option<String>, Error> {
    match split_env_path() {
        Ok(paths) => {
            for path in paths {
                match read_dir(path) {
                    // exists, is dir, allowed
                    Ok(mut r) => match search_command_in_dir(command, &mut r) {
                        Some(r) => return Ok(Some(r)),
                        None => continue,
                    },
                    Err(_) => continue, // errors remain here
                }
            }

            Ok(None)
        }
        Err(e) => Err(Error::new(ErrorKind::Interrupted, e)),
    }
}

fn search_command_in_dir(command: &str, dir: &mut ReadDir) -> Option<String> {
    for entry in dir {
        match entry {
            Ok(r) => match match_command_and_file(command, &r) {
                Ok(path) => match path {
                    Some(r) => return Some(r),
                    None => continue,
                },
                Err(_) => continue, // errors remain here
            },
            Err(_) => continue, // errors remain here
        }
    }

    None
}

fn match_command_and_file(command: &str, entry: &DirEntry) -> Result<Option<String>, Error> {
    match is_executable_file(entry) {
        Ok(is_exe) => {
            if !is_exe {
                return Ok(None);
            }

            let file_name = match entry.file_name().into_string() {
                Ok(r) => r,
                Err(_) => return Err(Error::new(ErrorKind::InvalidFilename, "")),
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
}

fn is_allowed_dir(path: &str) -> bool {
    match read_dir(path) {
        // exists, is dir, allowed
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_executable_file(entry: &DirEntry) -> Result<bool, Error> {
    match entry.metadata() {
        Ok(md) => {
            if md.is_dir() {
                Ok(false)
            } else {
                // windows?
                Ok(md.permissions().mode() & 0o111 != 0)
            }
        }
        Err(e) => Err(e),
    }
}
