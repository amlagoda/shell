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
                        another => {
                            output = command_from_path(another, iter);
                        }
                    },
                    None => {}
                }

                println!("{}", output);
            }
            Err(_) => {
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

fn command_from_path(name: &str, args: SplitWhitespace) -> String {
    let path = search_path(name);

    if path.len() == 0 {
        return format!("{}: command not found", name);
    }

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

fn command_type(mut iter: SplitWhitespace) -> String {
    let commands = ["type", "echo", "exit"];

    match iter.next() {
        Some(command) => {
            if commands.contains(&command) {
                return format!("{} is a shell builtin", command);
            }

            let path = search_path(&command);

            if path.len() > 0 {
                return format!("{} is {}", command, path);
            }

            format!("{}: not found", command)
        }
        None => String::from(": not found"),
    }
}

fn command_echo(iter: SplitWhitespace) -> String {
    format!("{}", iter.collect::<Vec<&str>>().join(" "))
}

fn search_path(command: &str) -> String {
    for path in paths() {
        match fs::read_dir(path) {
            // проверено: существует, каталог, доступен
            Ok(dir) => {
                for entry in dir {
                    match entry {
                        Ok(entry) => {
                            if !is_executable_file(&entry) {
                                continue;
                            }

                            let file_name = match entry.file_name().into_string() {
                                Ok(file_name) => file_name,
                                Err(_) => String::new(),
                            };

                            if command == file_name {
                                return match entry.path().to_str() {
                                    Some(path) => String::from(path),
                                    None => String::new(),
                                };
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        };
    }

    String::new()
}

fn paths() -> Vec<String> {
    match env::var("PATH") {
        Ok(path) => path
            .split(':')
            .map(|path| String::from(path))
            .collect::<Vec<String>>(),
        Err(_) => Vec::new(),
    }
}

fn is_executable_file(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(meta) => {
            if meta.is_dir() {
                return false;
            }
            // windows?
            meta.permissions().mode() & 0o111 != 0
        }
        Err(_) => false,
    }
}
