use std::collections::VecDeque;
use std::env::{current_dir, home_dir, set_current_dir, var, VarError};
use std::fs::{read_dir, DirEntry, File, ReadDir};
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

fn main() -> ExitCode {
    const COMMAND_TYPE: &str = "type";
    const COMMAND_ECHO: &str = "echo";
    const COMMAND_PWD: &str = "pwd";
    const COMMAND_CD: &str = "cd";
    const COMMAND_EXIT: &str = "exit";

    let mut input = String::new();
    let mut output = String::from("enter the command");
    let mut redirect: Option<[String; 3]>;

    loop {
        input.clear();
        print!("$ ");

        match stdout().flush() {
            Ok(_) => {}
            Err(_) => return ExitCode::FAILURE,
        }

        match stdin().read_line(&mut input) {
            Ok(_) => {
                let (command, args, r) = parse_args(parse_input(input.as_str()));
                redirect = r;

                match command {
                    Some(command) => match command.as_str() {
                        COMMAND_TYPE => {
                            let commands = Vec::from([
                                COMMAND_TYPE,
                                COMMAND_ECHO,
                                COMMAND_PWD,
                                COMMAND_CD,
                                COMMAND_EXIT,
                            ]);

                            match command_type(args, &commands) {
                                Ok(r) => output = r,
                                Err(_) => return ExitCode::FAILURE,
                            }
                        }

                        COMMAND_ECHO => output = command_echo(args),

                        COMMAND_PWD => match command_pwd() {
                            Ok(r) => output = r,
                            Err(_) => return ExitCode::FAILURE,
                        },

                        COMMAND_CD => match command_cd(args) {
                            Ok(r) => output = r,
                            Err(_) => return ExitCode::FAILURE,
                        },

                        COMMAND_EXIT => return ExitCode::SUCCESS,

                        another => match command_from_env_path(another, args) {
                            Ok(r) => match r {
                                Some(r) => output = r,
                                None => output = format!("{}: command not found", command),
                            },
                            Err(_) => return ExitCode::FAILURE,
                        },
                    },
                    None => {}
                }
            }
            Err(_) => return ExitCode::FAILURE,
        }

        if output.len() > 0 {
            match redirect {
                Some(r) => {
                    let [_flow, _mode, path] = r;

                    match write_to_file(path.as_str(), output.as_str()) {
                        Ok(_) => {}
                        // catalog not found and permissions errors
                        Err(_) => println!("{}: No such file or directory", path),
                    }
                }
                None => println!("{}", output),
            }
        }
    }
}

fn parse_args(
    mut args: VecDeque<String>,
) -> (Option<String>, VecDeque<String>, Option<[String; 3]>) {
    let command = args.pop_front();
    let mut args_new: VecDeque<String> = VecDeque::new();
    let mut redirect = [String::new(), String::new(), String::new()];
    let mut is_path = false;

    loop {
        match args.pop_front() {
            Some(r) => {
                if is_path {
                    redirect[2] = r.clone();
                    break;
                }

                if is_redirect(r.as_str()) {
                    let r = normalize_redirect(r.as_str());
                    [redirect[0], redirect[1]] = parse_redirect(r.as_str());
                    is_path = true;
                } else {
                    args_new.push_back(r.clone());
                }
            }
            None => break,
        }
    }

    if redirect[2].len() > 0 {
        (command, args_new, Some(redirect))
    } else {
        (command, args_new, None)
    }
}

fn is_redirect(arg: &str) -> bool {
    const REDIRECTS: [&str; 6] = [">", "1>", "2>", ">>", "1>>", "2>>"];
    REDIRECTS.contains(&arg)
}

fn normalize_redirect(redirect: &str) -> String {
    if [">", ">>"].contains(&redirect) {
        format!("1{}", redirect)
    } else {
        String::from(redirect)
    }
}

fn parse_redirect(redirect: &str) -> [String; 2] {
    let mut flow = String::new();
    let mut mode = String::new();

    for r in redirect.chars() {
        if flow.len() == 0 {
            flow.push(r);
        } else {
            mode.push(r);
        }
    }

    [flow, mode]
}

fn parse_input(input: &str) -> VecDeque<String> {
    const MODE_NORMAL: u8 = 1;
    const MODE_SINGLE: u8 = 2;
    const MODE_DOUBLE: u8 = 3;
    const MODE_SHIELD: u8 = 4;

    let mut mode = [MODE_NORMAL, MODE_NORMAL]; // current, previous
    let mut input = input.trim().chars().peekable();
    let mut arg = String::new();
    let mut args: VecDeque<String> = VecDeque::new();

    loop {
        match input.next() {
            Some(r) => {
                match mode[0] {
                    MODE_SHIELD => {
                        arg.push(r);
                        mode.reverse();
                    }
                    MODE_SINGLE => match r {
                        '\'' => mode = [MODE_NORMAL, MODE_SINGLE],
                        _ => arg.push(r),
                    },
                    MODE_DOUBLE => match r {
                        '"' => mode = [MODE_NORMAL, MODE_DOUBLE],
                        '\\' => match input.peek() {
                            Some(n) => {
                                if *n == '"' || *n == '\\' {
                                    mode = [MODE_SHIELD, MODE_DOUBLE];
                                } else {
                                    arg.push(r);
                                }
                            }
                            None => arg.push(r),
                        },
                        _ => arg.push(r),
                    },
                    // MODE_NORMAL
                    _ => match r {
                        '"' => mode = [MODE_DOUBLE, MODE_NORMAL],
                        '\'' => mode = [MODE_SINGLE, MODE_NORMAL],
                        '\\' => mode = [MODE_SHIELD, MODE_NORMAL],
                        ' ' => {
                            if arg.len() > 0 {
                                args.push_back(arg);
                                arg = String::new();
                            }
                        }
                        _ => arg.push(r),
                    },
                }
            }
            None => {
                if arg.len() > 0 {
                    args.push_back(arg);
                    break;
                }
            }
        }
    }

    args
}

fn command_from_env_path(command: &str, args: VecDeque<String>) -> Result<Option<String>, Error> {
    match search_command_in_env_path(command) {
        Ok(path) => match path {
            Some(_) => {
                let mut process = Command::new(command);

                for arg in args {
                    process.arg(arg);
                }

                match process.stdout(Stdio::piped()).spawn() {
                    Ok(mut process) => match process.wait() {
                        Ok(_) => match process.stdout {
                            // take?
                            Some(mut r) => {
                                let mut output = String::new();

                                match r.read_to_string(&mut output) {
                                    Ok(_) => Ok(Some(output.trim().to_string())),
                                    Err(e) => Err(e), // not unicode
                                }
                            }
                            None => Ok(Some(String::new())),
                        },
                        Err(e) => Err(e), // fail exit status
                    },
                    Err(e) => Err(e), // ?
                }
            }
            None => Ok(None),
        },
        Err(e) => Err(e), // PATH not present, PATH not unicode
    }
}

fn command_cd(args: VecDeque<String>) -> Result<String, Error> {
    let mut path = match args.iter().next() {
        Some(r) => String::from(r),
        None => String::new(),
    };

    if path == "~" {
        path = match home_dir() {
            Some(r) => match r.to_str() {
                Some(r) => String::from(r),
                None => path,
            },
            None => path,
        };
    }

    if !is_allowed_dir(&path) {
        return Ok(format!("cd: {}: No such file or directory", path));
    }

    match set_current_dir(&path) {
        Ok(_) => Ok(String::new()),
        Err(e) => Err(e),
    }
}

fn command_pwd() -> Result<String, Error> {
    match current_dir() {
        Ok(path) => match path.to_str() {
            Some(r) => Ok(String::from(r)),
            None => Err(Error::new(ErrorKind::InvalidFilename, "")), // not unicode error
        },
        Err(e) => Err(e), // not exists or permissions errors
    }
}

fn command_type(args: VecDeque<String>, commands: &Vec<&str>) -> Result<String, Error> {
    match args.iter().next() {
        Some(command) => {
            if commands.contains(&command.as_str()) {
                return Ok(format!("{} is a shell builtin", command));
            }

            match search_command_in_env_path(&command) {
                Ok(path) => match path {
                    Some(r) => Ok(format!("{} is {}", command, r)),
                    None => Ok(format!("{}: not found", command)),
                },
                Err(e) => Err(e), // PATH not present, PATH not unicode
            }
        }
        None => Ok(String::from(": not found")),
    }
}

fn command_echo(args: VecDeque<String>) -> String {
    String::from(
        args.iter()
            .map(|arg| arg.as_str())
            .collect::<Vec<&str>>()
            .join(" "),
    )
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
        // PATH not present, PATH not unicode
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
}

fn match_command_and_file(command: &str, entry: &DirEntry) -> Result<Option<String>, Error> {
    match is_executable_file(entry) {
        Ok(is_exe) => {
            if !is_exe {
                return Ok(None);
            }

            let file_name = match entry.file_name().into_string() {
                Ok(r) => r,
                // no unicode error
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
        Ok(_) => true, // exists, is dir, allowed
        Err(_) => false,
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

fn write_to_file(path: &str, content: &str) -> Result<(), Error> {
    // Create a file if it does not exist, and will truncate it if it does
    match File::create(Path::new(path)) {
        Ok(mut r) => match r.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e), // catalog not found and permissions errors
    }
}
