mod command;
use command::command::{COMMAND_CD, COMMAND_ECHO, COMMAND_EXIT, COMMAND_PWD, COMMAND_TYPE};
use crossterm::{
    cursor::MoveLeft,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::collections::VecDeque;
use std::env::{current_dir, home_dir, set_current_dir, var, VarError};
use std::fs::{read_dir, DirEntry, OpenOptions, ReadDir};
use std::io::{stdin, stdout, Error, ErrorKind, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Child, Command, ExitCode, Stdio};

fn main() -> ExitCode {
    match enable_raw_mode() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return ExitCode::FAILURE;
        }
    }

    let mut stdout = stdout();
    let mut input = String::new();
    let mut output: Option<String> = None;
    let mut error: Option<String> = None;
    let mut print: Option<String> = None;
    let mut is_exit = false;

    match write!(stdout, "$ ") {
        Ok(_) => match stdout.flush() {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e.to_string());
                return ExitCode::FAILURE;
            }
        },
        Err(e) => {
            println!("{}", e.to_string());
            return ExitCode::FAILURE;
        }
    }

    loop {
        match read() {
            Ok(r) => match r {
                Event::Key(event) => match event.code {
                    KeyCode::Enter => {
                        //(Option<String>, VecDeque<String>, Option<[String; 3]>)
                        let (name, args, _) = parse_args(parse_input(input.as_str()));

                        match name {
                            Some(r) => (output, error, is_exit) = command(r.as_str(), args),
                            None => error = Some(String::from(": not found")),
                        }

                        input.clear();
                    }

                    KeyCode::Backspace => {
                        if input.len() == 0 {
                            continue;
                        }

                        match execute!(stdout, MoveLeft(1), Clear(ClearType::UntilNewLine)) {
                            Ok(_) => {
                                input.pop();
                            }
                            Err(e) => {
                                println!("{}", e.to_string());
                                return ExitCode::FAILURE;
                            }
                        }
                    }

                    KeyCode::Tab => {
                        if input != "ex" {
                            continue;
                        }

                        input.push_str("it");
                        print = Some(String::from("it"));
                    }

                    KeyCode::Char(c) => {
                        if c == 'c' {
                            match event.modifiers {
                                KeyModifiers::CONTROL => {
                                    print = Some(String::from("^C"));
                                    is_exit = true;
                                }
                                _ => {
                                    input.push(c);
                                    print = Some(String::from(c));
                                }
                            }
                        } else {
                            input.push(c);
                            print = Some(String::from(c));
                        }
                    }

                    _ => {}
                },
                _ => {}
            },
            Err(e) => {
                println!("{}", e.to_string());
                return ExitCode::FAILURE;
            }
        }

        match print {
            Some(r) => match write!(stdout, "{}", r) {
                Ok(_) => match stdout.flush() {
                    Ok(_) => print = None,
                    Err(e) => {
                        println!("{}", e.to_string());
                        return ExitCode::FAILURE;
                    }
                },
                Err(e) => {
                    println!("{}", e.to_string());
                    return ExitCode::FAILURE;
                }
            },
            None => {}
        }

        match error {
            Some(err) => {
                let message = match output {
                    Some(_) => format!("\r\n{}\r\n", err),
                    None => format!("\r\n{}\r\n$ ", err),
                };

                match write!(stdout, "{}", message) {
                    Ok(_) => match stdout.flush() {
                        Ok(_) => error = None,
                        Err(e) => {
                            println!("{}", e.to_string());
                            return ExitCode::FAILURE;
                        }
                    },
                    Err(e) => {
                        println!("{}", e.to_string());
                        return ExitCode::FAILURE;
                    }
                }
            }
            None => {}
        }

        match output {
            Some(r) => match write!(stdout, "{}", format!("\r\n{}\r\n$ ", r)) {
                Ok(_) => match stdout.flush() {
                    Ok(_) => output = None,
                    Err(e) => {
                        println!("{}", e.to_string());
                        return ExitCode::FAILURE;
                    }
                },
                Err(e) => {
                    println!("{}", e.to_string());
                    return ExitCode::FAILURE;
                }
            },
            None => {}
        }

        if is_exit {
            break;
        }
    }

    match disable_raw_mode() {
        Ok(_) => {
            println!(""); // %
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("{}", e.to_string());
            ExitCode::FAILURE
        }
    }
}

/*fn main() {


    let mut input = String::new();

    let mut redirect: Option<[String; 3]>;


    loop {
        input.clear();

        match stdin().read_line(&mut input) {
            Ok(_) => {

                redirect = r;


            }
            Err(e) => {
                println!("{}", e.to_string());
                return ExitCode::FAILURE;
            }
        }

        match redirect {
            Some(rd) => {
                let [flow, mode, path] = rd;

                if flow == "1" {
                    let out = match output {
                        Some(r) => format!("{}\n", r),
                        None => String::new(),
                    };

                    output = None;

                    match write_to_file(path.as_str(), out.as_str(), mode == ">>") {
                        Ok(_) => {}
                        Err(e) => println!("{}: {}", path, e.to_string()),
                    }
                } else {
                    let err = match error {
                        Some(e) => format!("{}\n", e),
                        None => String::new(),
                    };

                    error = None;

                    match write_to_file(path.as_str(), err.as_str(), mode == ">>") {
                        Ok(_) => {}
                        Err(e) => println!("{}: {}", path, e.to_string()),
                    }
                }
            }
            None => {}
        }


    }
}*/

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
                }
                break;
            }
        }
    }

    args
}

fn command(command: &str, args: VecDeque<String>) -> (Option<String>, Option<String>, bool) {
    let mut output: Option<String> = None;
    let mut error: Option<String> = None;
    let mut is_exit = false;

    match command {
        COMMAND_TYPE => {
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
        }

        COMMAND_ECHO => output = Some(command_echo(args)),

        COMMAND_PWD => match command_pwd() {
            Ok(r) => output = Some(r),
            Err(e) => error = Some(e.to_string()),
        },

        COMMAND_CD => match command_cd(args) {
            Ok(_) => output = None,
            Err(e) => error = Some(e.to_string()),
        },

        COMMAND_EXIT => is_exit = true,

        another => match command_from_env_path(another, args) {
            Ok(r) => match r {
                Some(r) => [output, error] = r,
                None => {}
            },
            Err(e) => error = Some(e.to_string()),
        },
    }

    (output, error, is_exit)
}

fn command_from_env_path(
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
                        Ok(_) => match read_process_output_to_strings(r) {
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
}

fn command_cd(args: VecDeque<String>) -> Result<(), Error> {
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
        let msg = format!("cd: {}: No such file or directory", path);
        return Err(Error::new(ErrorKind::NotFound, msg));
    }

    match set_current_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn command_pwd() -> Result<String, Error> {
    match current_dir() {
        Ok(path) => match path.to_str() {
            Some(r) => Ok(String::from(r)),
            None => Err(Error::new(ErrorKind::InvalidFilename, "invalid file name")),
        },
        Err(e) => Err(e),
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
}

fn command_echo(args: VecDeque<String>) -> String {
    String::from(
        args.iter()
            .map(|arg| arg.as_str())
            .collect::<Vec<&str>>()
            .join(" "),
    )
}

fn read_process_output_to_strings(process: Child) -> Result<[Option<String>; 2], Error> {
    let mut stdout = None;
    let mut stderr = None;

    // take?
    match process.stdout {
        Some(mut r) => {
            let mut output = String::new();

            match r.read_to_string(&mut output) {
                Ok(_) => {
                    if output.len() > 0 {
                        stdout = Some(output.trim().to_string());
                    }
                }
                Err(e) => return Err(e),
            }
        }
        None => {}
    }

    match process.stderr {
        Some(mut e) => {
            let mut error = String::new();

            match e.read_to_string(&mut error) {
                Ok(_) => {
                    if error.len() > 0 {
                        stderr = Some(error.trim().to_string());
                    }
                }
                Err(e) => return Err(e),
            }
        }
        None => {}
    }

    Ok([stdout, stderr])
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
