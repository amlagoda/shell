mod redirect;

use crate::parser::redirect::{is_redirect, to_redirect, Redirect};
use std::collections::VecDeque;
use std::io::{Error, ErrorKind};

pub fn parse(input: &str) -> Result<Parsed, Error> {
    to_parsed(parse_input(input))
}

#[derive(Debug)]
pub struct Parsed {
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub redirect: Option<Redirect>,
}

fn to_parsed(mut args: VecDeque<String>) -> Result<Parsed, Error> {
    let mut parsed = Parsed {
        command: None,
        args: None,
        redirect: None,
    };

    let mut args_new: Vec<String> = vec![];

    while !args.is_empty() {
        let arg = args.pop_front().unwrap();

        if is_redirect(arg.as_str()) {
            let path = args.pop_front();

            if parsed.command.is_none() || path.is_none() {
                return Err(Error::new(ErrorKind::InvalidInput, "parse error"));
            }

            let redirect = to_redirect(arg.as_str(), path.unwrap().as_str());
            parsed.redirect = Some(redirect);
        } else if parsed.command.is_none() {
            parsed.command = Some(arg);
        } else if parsed.redirect.is_none() {
            args_new.push(arg);
        } else {
            return Err(Error::new(ErrorKind::InvalidInput, "parse error"));
        }
    }

    if !args_new.is_empty() {
        parsed.args = Some(args_new);
    }

    Ok(parsed)
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
        let iter = input.next();

        if iter.is_none() {
            if !arg.is_empty() {
                args.push_back(arg);
            }
            break;
        }

        let symbol = iter.unwrap();
        let current_mode = mode[0];

        match current_mode {
            MODE_SHIELD => {
                arg.push(symbol);
                mode.reverse();
            }

            MODE_SINGLE => match symbol {
                '\'' => mode = [MODE_NORMAL, MODE_SINGLE],
                _ => arg.push(symbol),
            },

            MODE_DOUBLE => match symbol {
                '"' => mode = [MODE_NORMAL, MODE_DOUBLE],
                '\\' => match input.peek() {
                    Some(n) => {
                        if *n == '"' || *n == '\\' {
                            mode = [MODE_SHIELD, MODE_DOUBLE];
                        } else {
                            arg.push(symbol);
                        }
                    }
                    None => arg.push(symbol),
                },
                _ => arg.push(symbol),
            },

            // MODE_NORMAL
            _ => match symbol {
                '"' => mode = [MODE_DOUBLE, MODE_NORMAL],
                '\'' => mode = [MODE_SINGLE, MODE_NORMAL],
                '\\' => mode = [MODE_SHIELD, MODE_NORMAL],
                ' ' => {
                    if !arg.is_empty() {
                        args.push_back(arg);
                        arg = String::new();
                    }
                }
                _ => arg.push(symbol),
            },
        }
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let r = parse("").unwrap();
        assert!(r.command.is_none());
        assert!(r.args.is_none());
        assert!(r.redirect.is_none());

        let r = parse("command").unwrap();
        assert_eq!("command", r.command.unwrap());
        assert!(r.args.is_none());
        assert!(r.redirect.is_none());

        let r = parse("command arg1 arg2").unwrap();
        assert_eq!("command", r.command.unwrap());
        assert_eq!(vec!["arg1", "arg2"], r.args.unwrap());
        assert!(r.redirect.is_none());

        let r = parse("command arg > path").unwrap();
        assert_eq!("command", r.command.unwrap());
        assert_eq!(vec!["arg"], r.args.unwrap());
        assert!(r.redirect.is_some());

        let r = parse("command > path").unwrap();
        assert_eq!("command", r.command.unwrap());
        assert!(r.args.is_none());
        assert!(r.redirect.is_some());

        assert!(parse("command > path some").is_err());

        assert!(parse("command >").is_err());

        assert!(parse("> path").is_err());

        assert!(parse(">").is_err());
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            VecDeque::from(["echo", "1'"].map(|r| r.to_string())),
            parse_input(" echo  1\\'")
        );

        assert_eq!(
            VecDeque::from(["echo", " 1 2"].map(|r| r.to_string())),
            parse_input("echo ' 1 ''2'")
        );

        assert_eq!(
            VecDeque::from(["echo", "  ' \\ 1"].map(|r| r.to_string())),
            parse_input("echo \"  ' \\ 1\"")
        );
    }
}
