pub mod keyboard {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    pub fn handle_key(
        mut input: String,
        key: &KeyEvent,
    ) -> (String, Option<String>, bool, bool, bool) {
        let mut to_print: Option<String> = None;
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

            KeyCode::Tab => match input.as_str() {
                "ech" => {
                    input.push_str("o ");
                    to_print = Some("o ".to_string());
                }
                "exi" => {
                    input.push_str("t ");
                    to_print = Some("t ".to_string());
                }
                _ => to_print = Some("\x07".to_string()),
            },

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

        (input, to_print, is_enter, is_exit, is_backspace)
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
            if r.starts_with(input) {
                matches.push(r.to_string());
            }
        }

        let len = matches.len();

        if len == 0 {
            return None;
        }

        if len == 1 {
            let end = matches[0].replace(input, "");
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

            let (end, matches) = ("oo".to_string(), vec!["foo".to_string()]);
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("f", &vec!("foo"))
            );

            let (end, matches) = ("".to_string(), vec!["foo".to_string()]);
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("foo", &vec!("foo"))
            );

            let (end, matches) = ("oo".to_string(), vec!["foo".to_string()]);
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("f", &vec!("foo", "bar"))
            );

            let (end, matches) = ("".to_string(), vec!["foo".to_string()]);
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("foo", &vec!("foo", "bar"))
            );

            let (end, matches) = ("oo".to_string(), vec!["foo".to_string(), "foo".to_string()]);
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("f", &vec!("foo", "foo"))
            );

            let (end, matches) = ("".to_string(), vec!["foo".to_string(), "foo".to_string()]);
            assert_eq!(
                Some((Some(end), Some(matches))),
                complete_input("foo", &vec!("foo", "foo"))
            );

            let end: Option<String> = None;
            let matches = vec!["foo".to_string(), "fooo".to_string()];
            assert_eq!(
                Some((end, Some(matches))),
                complete_input("f", &vec!("foo", "fooo"))
            );
        }
    }
}
