use crate::fs::search_executable_files_in_paths;

pub fn complete_input(
    input: &str,
    commands: &Vec<&str>,
    paths: &Vec<&str>,
) -> Option<(Option<String>, Option<Vec<String>>)> {
    let mut variants: Option<Vec<String>> = None;

    if let Some((end, var)) = complete(input, commands) {
        if end.is_some() {
            return Some((end, None));
        }

        variants = var;
    }

    if let Some(r) = search_executable_files_in_paths(input, paths) {
        let r = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();
        let r = paths_to_names(&r);
        let names = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

        if let Some((end, var)) = complete(input, &names) {
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

fn complete(input: &str, variants: &Vec<&str>) -> Option<(Option<String>, Option<Vec<String>>)> {
    if input.is_empty() || variants.is_empty() {
        return None;
    }

    let mut matches: Vec<String> = vec![];

    for r in variants {
        if r.starts_with(input) && r != &input {
            matches.push(r.to_string());
        }
    }

    matches.sort_unstable();
    matches.dedup();

    let len = matches.len();

    if len == 0 {
        return None;
    }

    let short = matches.iter().min_by_key(|r| r.len()).unwrap();

    if len == 1 {
        let end = format!("{} ", short.replacen(input, "", 1));
        return Some((Some(end), None));
    }

    let is_chain = matches
        .iter()
        .filter(|&r| r != short)
        .all(|r| r.starts_with(short.as_str()));

    if is_chain {
        Some((Some(short.replacen(input, "", 1)), None))
    } else {
        Some((None, Some(matches)))
    }
}

fn paths_to_names(paths: &Vec<&str>) -> Vec<String> {
    paths
        .iter()
        .map(|r| r.split("/").last().unwrap().to_string())
        .collect::<Vec<String>>()
}

struct Completion {
    selected: Option<String>,
    variants: Option<Vec<String>>,
}

impl Completion {
    fn new_selected(selected: String) -> Completion {
        Completion {
            selected: Some(selected),
            variants: None,
        }
    }

    fn new_variants(variants: Vec<String>) -> Completion {
        Completion {
            selected: None,
            variants: Some(variants),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn test_complete_input() {
        let path = get_fixture_dir();

        let r = complete_input("f", &vec!["bar"], &vec![format!("{}1/", path).as_str()]);
        let f = (Some("oo ".to_string()), None);
        assert_eq!(Some(f), r);

        let r = complete_input("f", &vec!["fooo"], &vec![format!("{}1/", path).as_str()]);
        let f = (Some("ooo ".to_string()), None);
        assert_eq!(Some(f), r);

        let r = complete_input(
            "f",
            &vec!["foo", "fii"],
            &vec![format!("{}2/", path).as_str()],
        );
        let m = vec!["fii", "foo", "fyy"]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>();
        let f = (None, Some(m));
        assert_eq!(Some(f), r);
    }

    #[test]
    fn test_paths_to_names() {
        let paths = vec!["foo/bar", "/baz/maz", "/vaz/gaz/"];
        let r = vec!["bar".to_string(), "maz".to_string(), "".to_string()];
        assert_eq!(r, paths_to_names(&paths));
    }

    #[test]
    fn test_complete() {
        assert_eq!(None, complete("foo", &vec!("foo")));

        assert_eq!(None, complete("foo", &vec!("bar")));

        assert_eq!(None, complete("foo", &vec!("foo", "foo")));

        let r = complete("f", &vec!["fo", "foo", "fooo"]);
        let f = (Some("o".to_string()), None);
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["fo", "foo"]);
        let f = (Some("o".to_string()), None);
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["foo", "foo"]);
        let f = (Some("oo ".to_string()), None);
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["foo"]);
        let f = (Some("oo ".to_string()), None);
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["fo", "foo", "fi", "fii"]);
        let m = vec!["fi", "fii", "fo", "foo"]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>();
        let f = (None, Some(m));
        assert_eq!(Some(f), r);
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
