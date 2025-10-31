pub mod keyboard {
    use crate::fs::fs::search_executable_files_in_paths;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    pub fn handle_key(
        mut input: String,
        key: &KeyEvent,
        previous_key: &Option<KeyEvent>,
        commands: &Vec<&str>,
        bin_paths: &Vec<&str>,
    ) -> (String, Option<String>, Option<String>, bool, bool, bool) {
        let mut to_print: Option<String> = None;
        let mut hint: Option<String> = None;
        let mut is_enter = false;
        let mut is_exit = false;
        let mut is_backspace = false;

        match key.code {
            KeyCode::Enter => is_enter = true,

            KeyCode::Backspace => {
                if !input.is_empty() {
                    input.pop();
                    is_backspace = true;
                }
            }

            KeyCode::Tab => {
                to_print = Some("\x07".to_string());

                let r = complete(input.as_str(), commands, bin_paths);

                if let Some((end, variants)) = r {
                    if let Some(r) = variants {
                        if let Some(f) = previous_key {
                            if f.code == KeyCode::Tab {
                                hint = Some(r.join("  "));
                                to_print = None;
                            }
                        }
                    } else if let Some(r) = end {
                        input.push_str(format!("{} ", r).as_str());
                        to_print = Some(format!("{} ", r));
                    }
                }
            }

            KeyCode::Char(r) => {
                let is_ctrl = key.modifiers == KeyModifiers::CONTROL;

                if r == 'c' && is_ctrl {
                    to_print = Some("^C".to_string());
                    is_exit = true;
                } else if r == 'j' && is_ctrl {
                    is_enter = true;
                } else {
                    input.push(r);
                    to_print = Some(r.to_string());
                }
            }

            _ => {}
        }

        (input, to_print, hint, is_enter, is_exit, is_backspace)
    }

    fn complete(
        input: &str,
        commands: &Vec<&str>,
        paths: &Vec<&str>,
    ) -> Option<(Option<String>, Option<Vec<String>>)> {
        let mut variants: Option<Vec<String>> = None;

        if let Some((end, var)) = complete_input(input, commands) {
            if end.is_some() {
                return Some((end, None));
            }

            variants = var;
        }

        if let Some(r) = search_executable_files_in_paths(input, paths) {
            let r = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();
            let r = paths_to_names(&r);
            let names = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

            if let Some((end, var)) = complete_input(input, &names) {
                if end.is_some() {
                    return Some((end, None));
                }

                if let Some(mut r) = variants {
                    r.append(&mut var.unwrap());
                    variants = Some(r);
                } else {
                    variants = var;
                }
            }
        }

        if let Some(mut r) = variants {
            r.sort_unstable();
            r.dedup();
            Some((None, Some(r)))
        } else {
            None
        }
    }

    fn complete_input(
        input: &str,
        variants: &Vec<&str>,
    ) -> Option<(Option<String>, Option<Vec<String>>)> {
        if input.is_empty() || variants.is_empty() {
            return None;
        }

        let mut matches: Vec<String> = vec![];

        for r in variants {
            if r.starts_with(input) && r != &input {
                matches.push(r.to_string());
            }
        }

        let len = matches.len();

        if len == 0 {
            return None;
        }

        if len == 1 {
            let end = matches[0].replacen(input, "", 1);
            let matches = matches
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<String>>();

            return Some((Some(end), Some(matches)));
        }

        let mut unique = matches.clone();
        unique.sort_unstable();
        unique.dedup();

        let matches = matches
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>();

        if len == unique.len() {
            Some((None, Some(matches)))
        } else {
            Some((Some(matches[0].replace(input, "")), Some(matches)))
        }
    }

    fn paths_to_names(paths: &Vec<&str>) -> Vec<String> {
        paths
            .iter()
            .map(|r| r.split("/").last().unwrap().to_string())
            .collect::<Vec<String>>()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::env::current_dir;

        #[test]
        fn test_complete() {
            assert_eq!(None, complete("", &vec!(), &vec!()));

            assert_eq!(None, complete("foo", &vec!(), &vec!()));

            assert_eq!(None, complete("", &vec!("foo"), &vec!()));

            assert_eq!(None, complete("", &vec!(), &vec!("foo")));

            assert_eq!(None, complete("", &vec!("foo"), &vec!("bar")));

            let f = complete("foo", &vec!["bar"], &vec![]);
            assert_eq!(None, f);

            let r = "oo".to_string();
            let f = complete("f", &vec!["foo"], &vec![]);
            assert_eq!(Some((Some(r), None)), f);

            let r = vec!["foo".to_string(), "fooo".to_string()];
            let f = complete("f", &vec!["foo", "fooo"], &vec![]);
            assert_eq!(Some((None, Some(r))), f);

            let path = get_fixture_dir();

            assert_eq!(None, complete("bar", &vec!(), &vec!(&path)));

            let r = "xe".to_string();
            let f = complete("e", &vec![], &vec![&path]);
            assert_eq!(Some((Some(r), None)), f);

            let r = vec!["foo".to_string(), "fooo".to_string()];
            let f = complete("f", &vec![], &vec![&path]);
            assert_eq!(Some((None, Some(r))), f);

            let r = "ar".to_string();
            let f = complete("b", &vec!["bar"], &vec![&path]);
            assert_eq!(Some((Some(r), None)), f);

            let r = "xe".to_string();
            let f = complete("e", &vec!["bar"], &vec![&path]);
            assert_eq!(Some((Some(r), None)), f);

            let r = vec!["bar".to_string(), "barr".to_string()];
            let f = complete("b", &vec!["bar", "barr"], &vec![&path]);
            assert_eq!(Some((None, Some(r))), f);

            let r = vec!["foo".to_string(), "fooo".to_string()];
            let f = complete("f", &vec!["bar"], &vec![&path]);
            assert_eq!(Some((None, Some(r))), f);

            let r = vec!["foo".to_string(), "fooo".to_string()];
            let f = complete("f", &vec!["foo", "fooo"], &vec![&path]);
            assert_eq!(Some((None, Some(r))), f);
        }

        #[test]
        fn test_paths_to_names() {
            let paths = vec!["foo/bar", "/baz/maz", "/vaz/gaz/"];
            let r = vec!["bar".to_string(), "maz".to_string(), "".to_string()];
            assert_eq!(r, paths_to_names(&paths));
        }

        #[test]
        fn test_complete_input() {
            assert_eq!(None, complete_input("", &vec!()));

            assert_eq!(None, complete_input("foo", &vec!()));

            assert_eq!(None, complete_input("", &vec!("foo")));

            assert_eq!(None, complete_input("foo", &vec!("bar")));

            assert_eq!(None, complete_input("foo", &vec!("foo")));

            assert_eq!(None, complete_input("foo", &vec!("foo", "bar")));

            assert_eq!(None, complete_input("foo", &vec!("foo", "foo")));

            let end = "oo".to_string();
            let matches = vec!["foo".to_string()];
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("f", &vec!("foo"))
            );

            let end = "oo".to_string();
            let matches = vec!["foo".to_string()];
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("f", &vec!("foo", "bar"))
            );

            let end = "oo".to_string();
            let matches = vec!["foo".to_string(), "foo".to_string()];
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("f", &vec!("foo", "foo"))
            );

            let end: Option<String> = None;
            let matches = vec!["foo".to_string(), "fooo".to_string()];
            assert_eq!(
                Some((end, Some(matches))),
                complete_input("f", &vec!("foo", "fooo"))
            );

            let end = "o".to_string();
            let matches = vec!["fooo".to_string()];
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("foo", &vec!("foo", "fooo"))
            );
        }

        fn get_fixture_dir() -> String {
            // ends with a slash
            format!("{}/test/fixture/keyboard/", get_current_dir())
        }

        fn get_current_dir() -> String {
            // does not end with a slash
            current_dir().unwrap().to_str().unwrap().to_string()
        }
    }
}
