mod pipeline;
mod redirect;

use crate::parser::pipeline::{is_pipeline, to_pipeline, Pipeline};
use crate::parser::redirect::{is_redirect, to_redirect, Redirect};
use std::collections::VecDeque;
use std::io::Error;

pub fn parse(input: &str) -> Result<Option<Vec<Parsed>>, Error> {
    to_parsed(parse_input(input))
}

pub struct Parsed {
    command: String,
    args: Option<Vec<String>>,
    redirect: Option<Redirect>,
    pipeline: Option<Pipeline>,
}

impl Parsed {
    fn new(
        command: String,
        args: Option<Vec<String>>,
        redirect: Option<Redirect>,
        pipeline: Option<Pipeline>,
    ) -> Parsed {
        Parsed {
            command,
            args,
            redirect,
            pipeline,
        }
    }

    pub fn command(&self) -> &str {
        self.command.as_str()
    }

    pub fn args(&self) -> Option<Vec<&str>> {
        self.args
            .as_ref()
            .map(|r| r.iter().map(|r| r.as_str()).collect::<Vec<&str>>())
    }

    pub fn redirect(&self) -> Option<&Redirect> {
        self.redirect.as_ref()
    }

    pub fn pipeline(&self) -> Option<&Pipeline> {
        self.pipeline.as_ref()
    }
}

fn to_parsed(mut args: VecDeque<String>) -> Result<Option<Vec<Parsed>>, Error> {
    let err = Error::other("parse error");
    let mut previous: Option<String> = None;
    let mut parsed = Parsed::new(String::new(), None, None, None);
    let mut parseds: Vec<Parsed> = vec![];
    let mut command_mode = false;
    let mut redirect_mode = false;
    let mut pipeline_mode = false;

    while !args.is_empty() {
        let current = args.pop_front().unwrap();
        let prev = previous.as_deref();

        if !command_mode && !is_command(current.as_str(), prev) {
            return Err(err);
        }

        if redirect_mode && !is_redirect_path(current.as_str(), prev) {
            return Err(err);
        }

        if pipeline_mode && !is_command(current.as_str(), prev) {
            return Err(err);
        }

        if is_command(current.as_str(), prev) {
            if pipeline_mode {
                parseds.push(parsed);
                parsed = Parsed::new(String::new(), None, None, None);
            }

            command_mode = true;
            pipeline_mode = false;
            parsed.command = current.clone();
        } else if is_redirect(current.as_str()) {
            redirect_mode = true;
        } else if is_redirect_path(current.as_str(), prev) {
            redirect_mode = false;
            parsed.redirect = to_redirect(prev.unwrap(), current.as_str());
        } else if is_pipeline(current.as_str()) {
            pipeline_mode = true;
            command_mode = false;
            parsed.pipeline = to_pipeline(current.as_str());
        } else {
            // is arg
            if parsed.args.is_none() {
                parsed.args = Some(vec![current.clone()]);
            } else {
                let mut args = parsed.args.unwrap();
                args.push(current.clone());
                parsed.args = Some(args);
            }
        }

        previous = Some(current);
    }

    if redirect_mode || pipeline_mode {
        return Err(err);
    }

    if !parsed.command().is_empty() {
        parseds.push(parsed);
    }

    if !parseds.is_empty() {
        Ok(Some(parseds))
    } else {
        Ok(None)
    }
}

fn is_command(current: &str, previous: Option<&str>) -> bool {
    !is_redirect(current)
        && !is_pipeline(current)
        && (previous.is_none() || is_pipeline(previous.unwrap()))
}

fn is_redirect_path(current: &str, previous: Option<&str>) -> bool {
    !is_redirect(current)
        && !is_pipeline(current)
        && previous.is_some()
        && is_redirect(previous.unwrap())
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
    fn test_parse() -> Result<(), Error> {
        assert!(parse(">").is_err());
        assert!(parse("cmd >").is_err());
        assert!(parse("> path").is_err());
        assert!(parse("> > path").is_err());
        assert!(parse("| > path").is_err());
        assert!(parse("cmd > >").is_err());
        assert!(parse("cmd > |").is_err());

        assert!(parse("|").is_err());
        assert!(parse("cmd |").is_err());
        assert!(parse("| cmd").is_err());
        assert!(parse("> | cmd").is_err());
        assert!(parse("| | cmd").is_err());
        assert!(parse("cmd | |").is_err());
        assert!(parse("cmd | >").is_err());

        assert!(parse("")?.is_none());

        let r = "cmd1 arg1 arg2 > path | cmd2 | cmd3 arg > path1";
        let mut parseds = parse(r)?.unwrap().into_iter();

        let p1 = parseds.next().unwrap();
        assert_eq!("cmd1", p1.command());
        assert_eq!(vec!("arg1", "arg2"), p1.args().unwrap());
        assert!(!p1.redirect().unwrap().is_stderr());
        assert!(!p1.redirect().unwrap().is_append());
        assert_eq!("path", p1.redirect().unwrap().path());
        assert!(p1.pipeline().unwrap().is_stdout());

        let p2 = parseds.next().unwrap();
        assert_eq!("cmd2", p2.command());
        assert!(p2.args().is_none());
        assert!(p2.redirect().is_none());
        assert!(p2.pipeline().unwrap().is_stdout());

        let p3 = parseds.next().unwrap();
        assert_eq!("cmd3", p3.command());
        assert_eq!(vec!("arg"), p3.args().unwrap());
        assert!(p3.redirect().is_some());
        assert!(p3.pipeline().is_none());

        Ok(())
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
