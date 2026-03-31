use crate::fs::find_files;

pub fn complete_input(input: &str, commands: &Vec<&str>, paths: &Vec<&str>) -> Option<Completion> {
    let mut variants: Option<Vec<String>> = None;

    if let Some(completion) = complete(input, commands) {
        if completion.is_selected() {
            return Some(completion);
        }

        variants = completion
            .get_variants()
            .as_ref()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
    }

    let only_executable = true;
    if let Some(r) = find_files(input, only_executable, paths) {
        let r = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();
        let r = paths_to_names(&r);
        let names = r.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

        if let Some(completion) = complete(input, &names) {
            if completion.is_selected() {
                return Some(completion);
            }

            let f = completion
                .get_variants()
                .as_ref()
                .map(|v| v.iter().map(|s| s.to_string()).collect());

            if let Some(mut r) = variants {
                r.append(&mut f.unwrap());
                variants = Some(r);
            } else {
                variants = f;
            }
        }
    }

    if let Some(mut r) = variants {
        r.sort_unstable();
        r.dedup();

        let r = r.iter().map(|s| s.to_string()).collect();

        Some(Completion::new_variants(r))
    } else {
        None
    }
}

fn complete(input: &str, variants: &Vec<&str>) -> Option<Completion> {
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
        let selected = format!("{} ", short.replacen(input, "", 1));
        return Some(Completion::new_selected(selected));
    }

    let is_chain = matches
        .iter()
        .filter(|&r| r != short)
        .all(|r| r.starts_with(short.as_str()));

    if is_chain {
        let selected = short.replacen(input, "", 1);
        Some(Completion::new_selected(selected))
    } else {
        Some(Completion::new_variants(matches))
    }
}

fn paths_to_names(paths: &Vec<&str>) -> Vec<String> {
    paths
        .iter()
        .map(|r| r.split("/").last().unwrap().to_string())
        .collect::<Vec<String>>()
}

#[derive(Debug, PartialEq)]
pub struct Completion {
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

    pub fn is_selected(&self) -> bool {
        self.selected.is_some()
    }

    pub fn get_selected(&self) -> Option<&str> {
        self.selected.as_deref()
    }

    pub fn get_variants(&self) -> Option<Vec<&str>> {
        self.variants
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
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
        let f = Completion::new_selected("oo ".to_string());
        assert_eq!(Some(f), r);

        let r = complete_input("f", &vec!["fooo"], &vec![format!("{}1/", path).as_str()]);
        let f = Completion::new_selected("ooo ".to_string());
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
        let f = Completion::new_variants(m);
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
        let f = Completion::new_selected("o".to_string());
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["fo", "foo"]);
        let f = Completion::new_selected("o".to_string());
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["foo", "foo"]);
        let f = Completion::new_selected("oo ".to_string());
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["foo"]);
        let f = Completion::new_selected("oo ".to_string());
        assert_eq!(Some(f), r);

        let r = complete("f", &vec!["fo", "foo", "fi", "fii"]);
        let m = vec!["fi", "fii", "fo", "foo"]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>();
        let f = Completion::new_variants(m);
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
