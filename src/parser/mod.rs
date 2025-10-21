mod redirect;

mod parser {
    use crate::parser::redirect::redirect::{is_redirect, normalize_redirect, parse_redirect};
    use std::collections::VecDeque;

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

    #[cfg(test)]
    mod tests {
        use super::*;

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
