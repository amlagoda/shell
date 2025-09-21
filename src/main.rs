// отключает предупреждение компилятора о неиспользуемом импорте
#[allow(unused_imports)]
// импорт: std - стандартная библиотека, io - раздел input/output
// self - видимо весь раздел для обращения без префикса
// Write - трейт записи
// {self, Write} - видимо короткая форма записи
use std::io::{self, Write};
// коды состояния возвращаемые текущим процессом своему родителю при
// нормальном завершении
use std::process::ExitCode;
use std::str::SplitWhitespace;

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
            Err(_error) => {
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
                let mut output = format!("{}: command not found", input);

                match iter.next() {
                    Some(command) => match command {
                        "type" => {
                            output = command_type(iter);
                        }
                        "echo" => {
                            output = command_echo(iter);
                        }
                        _ => {}
                    },
                    None => {}
                }

                println!("{}", output);
            }
            Err(_error) => {
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

fn command_type(mut iter: SplitWhitespace) -> String {
    let commands = ["type", "echo", "exit"];

    match iter.next() {
        Some(command) => {
            if commands.contains(&command) {
                return format!("{} is a shell builtin", command);
            }

            let path = search_command(&command);

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

fn search_command(command: &str) -> String {
    String::from("path")
}
