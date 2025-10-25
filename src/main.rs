use crate::command::command::command;
use crate::env::env::split_env_path;
use crate::fs::fs::write_to_file;
use crate::parser::parser::parse;
use crossterm::{
    cursor::MoveLeft,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::process::ExitCode;

mod command;
mod env;
mod fs;
mod parser;

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
    let mut redirect: Option<[String; 3]> = None;
    let mut is_exit = false;
    let mut is_enter = false;

    let r = split_env_path();
    if r.is_err() {
        println!("{}", r.unwrap_err().to_string());
        return ExitCode::FAILURE;
    }
    let r = r.unwrap();

    let bin_paths = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

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
                    KeyCode::Enter => is_enter = true,

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
                        if input == "ech" {
                            input.push_str("o ");
                            print = Some(String::from("o "));
                        }

                        if input == "exi" {
                            input.push_str("t ");
                            print = Some(String::from("t "));
                        }
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
                        } else if c == 'j' {
                            match event.modifiers {
                                KeyModifiers::CONTROL => {
                                    is_enter = true;
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

        if is_enter {
            let (name, args, r) = parse(input.as_str());
            redirect = r;

            match name {
                Some(r) => {
                    let args = args.iter().map(|r| r.as_str()).collect::<Vec<&str>>();
                    (output, error, is_exit) = command(r.as_str(), &args, &bin_paths);
                }
                None => error = Some(String::from(": not found")),
            }

            input.clear();
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
                        Err(e) => {
                            println!("{}: {}", path, e.to_string());
                            return ExitCode::FAILURE;
                        }
                    }
                } else {
                    let err = match error {
                        Some(e) => format!("{}\n", e),
                        None => String::new(),
                    };

                    error = None;

                    match write_to_file(path.as_str(), err.as_str(), mode == ">>") {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}: {}", path, e.to_string());
                            return ExitCode::FAILURE;
                        }
                    }
                }
            }
            None => {}
        }

        redirect = None;

        let mut to_print = [error, output].into_iter().peekable();

        loop {
            match to_print.next() {
                Some(r) => match r {
                    Some(r) => {
                        let mut rows = r.split("\n");

                        loop {
                            match rows.next() {
                                Some(r) => match write!(stdout, "\r\n{}", r) {
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
                                },
                                None => break,
                            }
                        }
                    }
                    None => {}
                },
                None => {
                    error = None;
                    output = None;
                    break;
                }
            }
        }

        if is_enter && !is_exit {
            match write!(stdout, "\r\n$ ",) {
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

            is_enter = false;
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
