// отключает предупреждение компилятора о неиспользуемом импорте
#[allow(unused_imports)]
// импорт: std - стандартная библиотека, io - раздел input/output
// self - видимо весь раздел для обращения без префикса
// Write - трейт записи
// {self, Write} - видимо короткая форма записи
use std::io::{self, Write};
// коды состояния возвращаемые текущим процессом своему родителю при
// нормальном завершении
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::process::ExitCode;
use std::str::SplitWhitespace;
// без этого не работает permissions().mode()
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
// для чтения output дочернего процесса
use std::env::VarError;
use std::fs::ReadDir;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> ExitCode {
    // изменяемая строка в памяти кучи
    let mut input = String::new();

    loop {
        // очистка буфера
        input.clear();
        // буфферизация вывода
        print!("$ ");

        // stdout() - создание дескриптора стандартного вывода текущего процесса
        // std::io::Stdout
        // flush() - немедленный вывод буферизованной строки
        match io::stdout().flush() {
            Ok(_n) => {}
            Err(_) => {
                return ExitCode::FAILURE;
            }
        }

        // stdin() - создание дескриптора стандартного потока ввода std::io::Stdin
        // read_line(&mut input) - блокирует дескриптор, считывает сроку и
        // помещает в буффер переданный в параметре. Строка считывается до
        // достижения новой строки, которое определяется наличием байта 0xA.
        // Поэтому нужно ставить ограничение с помощью std::io::Read::take, на
        // случай если байт не передан
        // Добавляется к уже имеющейся строке буффера, поэтому буффер нужно
        // очищать с помощью std::String::clear

        match io::stdin().read_line(&mut input) {
            // _ подчеркивание выключает предупреждение неиспользуемой переменной
            Ok(_len) => {
                let input: &str = input.trim();

                if input == "exit 0" {
                    break;
                }
                // split_whitespace() - разбивает строку по пробелам считая
                // не одиночный за один std::str::SplitWhitespace
                let mut iter = input.split_whitespace();
                // next() - std::str::SplitWhitespace
                // для пустого ввода
                let mut output = format!("{}: command not found", input);

                match iter.next() {
                    Some(command) => match command {
                        "type" => {
                            output = command_type(iter);
                        }
                        "echo" => {
                            output = command_echo(iter);
                        }
                        "pwd" => {
                            output = command_pwd(command);
                        }
                        "cd" => {
                            output = command_cd(command, iter);
                        }
                        another => {
                            output = command_from_path(another, iter);
                        }
                    },
                    None => {}
                }

                if output.len() > 0 {
                    println!("{}", output);
                }
            }
            Err(_) => {
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

fn command_cd(name: &str, mut args: SplitWhitespace) -> String {
    let mut path = match args.next() {
        Some(path) => String::from(path),
        None => String::new(),
    };

    if path == "~" {
        path = match env::home_dir() {
            Some(path) => match path.to_str() {
                Some(path) => String::from(path),
                None => String::new(),
            },
            None => String::new(),
        };
    }

    if !is_dir_exists(&path) {
        return format!("{}: {}: No such file or directory", name, path);
    }

    match env::set_current_dir(&path) {
        Ok(_) => String::new(),
        Err(_) => format!("{}: failed to run command", name),
    }
}

fn is_dir_exists(path: &str) -> bool {
    match fs::read_dir(path) {
        // проверено: существует, каталог, доступен
        Ok(_) => true,
        Err(_) => false,
    }
}

fn command_pwd(name: &str) -> String {
    match env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(path) => String::from(path),
            None => String::new(),
        },
        Err(_) => format!("{}: failed to run command", name),
    }
}

fn command_from_path(name: &str, args: SplitWhitespace) -> String {
    match search_command_in_env_path(name) {
        Ok(path) => {
            match path {
                Some(_) => {
                    // нужно чтобы передаваемое название name было в PATH
                    // иначе передавать path
                    let mut command = Command::new(name);

                    for arg in args {
                        command.arg(arg);
                    }

                    match command.stdout(Stdio::piped()).spawn() {
                        Ok(command) => {
                            // ограничение take?
                            match command.stdout {
                                Some(mut stdout) => {
                                    let mut output = String::new();

                                    match stdout.read_to_string(&mut output) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            return format!("{}: failed to run command", name);
                                        }
                                    };

                                    String::from(output.as_str().trim())
                                }
                                None => String::new(),
                            }
                        }
                        Err(_) => format!("{}: failed to run command", name),
                    }
                }
                None => format!("{}: command not found", name),
            }
        }
        Err(_) => format!("{}: command not found", name), // обработать
    }
}

fn command_type(mut iter: SplitWhitespace) -> String {
    let commands = ["type", "echo", "exit", "pwd", "cd"];

    match iter.next() {
        Some(command) => {
            if commands.contains(&command) {
                return format!("{} is a shell builtin", command);
            }

            match search_command_in_env_path(&command) {
                Ok(path) => match path {
                    Some(path) => {
                        return format!("{} is {}", command, path);
                    }
                    None => {}
                },
                Err(_) => {} // обработать
            }

            format!("{}: not found", command)
        }
        None => String::from(": not found"),
    }
}

fn command_echo(args: SplitWhitespace) -> String {
    format!("{}", args.collect::<Vec<&str>>().join(" "))
}

fn search_command_in_env_path(command: &str) -> Result<Option<String>, Error> {
    match split_env_path() {
        Ok(paths) => {
            for path in paths {
                match fs::read_dir(path) {
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

fn split_env_path() -> Result<Vec<String>, VarError> {
    match env::var("PATH") {
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
