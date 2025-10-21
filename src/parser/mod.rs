mod redirect;

mod parser {
    use crate::parser::redirect::redirect::{is_redirect, normalize_redirect, parse_redirect};
    use std::collections::VecDeque;

    fn parse(input: &str) -> (Option<String>, VecDeque<String>, Option<[String; 3]>) {
        group_args(parse_input(input))
    }

    fn group_args(
        mut args: VecDeque<String>,
    ) -> (Option<String>, VecDeque<String>, Option<[String; 3]>) {
        let command = args.pop_front();
        let mut args_new: VecDeque<String> = VecDeque::new();
        let mut redirect = [String::new(), String::new(), String::new()];
        let mut is_path = false;

        loop {
            let r = args.pop_front();

            if r.is_none() {
                break;
            }

            let arg = r.unwrap();

            if is_path {
                redirect[2] = arg;
                break;
            }

            if is_redirect(arg.as_str()) {
                let r = normalize_redirect(arg.as_str());
                [redirect[0], redirect[1]] = parse_redirect(r.as_str());
                is_path = true;
            } else {
                args_new.push_back(arg);
            }
        }

        if redirect[2].len() > 0 {
            (command, args_new, Some(redirect))
        } else {
            (command, args_new, None)
        }
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
                if arg.len() > 0 {
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
                        if arg.len() > 0 {
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
            let expected = (
                Some("echo".to_string()),
                VecDeque::from(["foo".to_string()]),
                Some(["1".to_string(), ">".to_string(), "path".to_string()]),
            );

            assert_eq!(expected, parse("echo foo > path"));
        }

        #[test]
        fn test_group_args1() {
            let expected = (
                Some("echo".to_string()),
                VecDeque::from(["foo".to_string(), "bar".to_string()]),
                None,
            );
            let r = VecDeque::from([
                "echo".to_string(),
                "foo".to_string(),
                "bar".to_string(),
                ">".to_string(),
            ]);

            assert_eq!(expected, group_args(r));
        }

        #[test]
        fn test_parse_input() {
            assert_eq!(
                VecDeque::from(["echo".to_string(), "1'".to_string()]),
                parse_input(" echo  1\\'")
            );

            assert_eq!(
                VecDeque::from(["echo".to_string(), " 1 2".to_string()]),
                parse_input("echo ' 1 ''2'")
            );

            assert_eq!(
                VecDeque::from(["echo".to_string(), "  ' \\ 1".to_string()]),
                parse_input("echo \"  ' \\ 1\"")
            );
        }
    }
}
