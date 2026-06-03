use crate::fs::{find_bins_starts_with, find_files_starts_with, FindFilesResult};
use crate::rule::Comprasion;
use crate::setting::Setting;

pub fn complete_input(input: &str, setting: &Setting) -> Option<Completion> {
    let args: Vec<&str> = input.split(" ").collect();
    let last = args.last();
    let len = args.len();

    if len == 1 {
        complete_command(
            last.unwrap(),
            &setting.available_commands(),
            &setting.bin_paths(),
        )
    } else if len > 1 {
        let file_search_data = get_file_search_data(last.unwrap(), setting.current_dir());
        let search_path = file_search_data.path();
        let file_prefix = file_search_data.file_prefix().unwrap_or("");

        complete_file(file_prefix, search_path)
    } else {
        None
    }
}

fn get_file_search_data(input: &str, current_dir: &str) -> FileSearchData {
    if !input.contains("/") {
        let input = if input.is_empty() {
            None
        } else {
            Some(input.to_string())
        };

        return FileSearchData::from(current_dir.to_string(), input);
    }

    let input: Vec<&str> = input.split("/").collect();
    let file_prefix = input.last().unwrap();

    if file_prefix == &"" {
        FileSearchData::from(input.join("/"), None)
    } else {
        FileSearchData::from(
            format!("{}/", input[0..input.len() - 1].join("/")),
            Some(file_prefix.to_string()),
        )
    }
}

struct FileSearchData {
    path: String,
    file_prefix: Option<String>,
}

impl FileSearchData {
    fn from(path: String, file_prefix: Option<String>) -> FileSearchData {
        FileSearchData { path, file_prefix }
    }

    fn path(&self) -> &str {
        self.path.as_str()
    }

    fn file_prefix(&self) -> Option<&str> {
        self.file_prefix.as_deref()
    }
}

fn complete_command(input: &str, commands: &Vec<&str>, paths: &Vec<&str>) -> Option<Completion> {
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

    if let FindFilesResult::Some(r) = find_bins_starts_with(input, paths) {
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

fn complete_file(input: &str, path: &str) -> Option<Completion> {
    let mut variants: Option<Vec<String>> = None;

    if let FindFilesResult::Some(r) = find_files_starts_with(input, &vec![path]) {
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
    if variants.is_empty() {
        return None;
    }

    let mut matches: Vec<String> = vec![];

    for r in variants {
        if Comprasion::PatternStartsWithNotEqual(r.to_string()).assert(input) {
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
        .all(|r| Comprasion::PatternStartsWith(r.to_string()).assert(short.as_str()));

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::env::get_current_dir;
//     use crate::fmt::NewLine;

//     #[test]
//     fn test_complete_input() {
//         let setting = Setting::from(NewLine::new(), vec![get_fixture_dir()], "".to_string())
//         let path = get_fixture_dir();
//         let current_path = "";

//         let r = complete_input(
//             "f",
//             &vec!["bar"],
//             &vec![format!("{}1/", path).as_str()],
//             current_path,
//         );
//         let f = Completion::new_selected("oo ".to_string());
//         assert_eq!(Some(f), r);

//         let r = complete_input(
//             "f",
//             &vec!["fooo"],
//             &vec![format!("{}1/", path).as_str()],
//             current_path,
//         );
//         let f = Completion::new_selected("ooo ".to_string());
//         assert_eq!(Some(f), r);

//         let r = complete_input(
//             "f",
//             &vec!["foo", "fii"],
//             &vec![format!("{}2/", path).as_str()],
//             current_path,
//         );
//         let m = vec!["fii", "foo", "fyy"]
//             .iter()
//             .map(|r| r.to_string())
//             .collect::<Vec<String>>();
//         let f = Completion::new_variants(m);
//         assert_eq!(Some(f), r);
//     }

//     #[test]
//     fn test_paths_to_names() {
//         let paths = vec!["foo/bar", "/baz/maz", "/vaz/gaz/"];
//         let r = vec!["bar".to_string(), "maz".to_string(), "".to_string()];
//         assert_eq!(r, paths_to_names(&paths));
//     }

//     #[test]
//     fn test_complete() {
//         assert_eq!(None, complete("foo", &vec!("foo")));

//         assert_eq!(None, complete("foo", &vec!("bar")));

//         assert_eq!(None, complete("foo", &vec!("foo", "foo")));

//         let r = complete("f", &vec!["fo", "foo", "fooo"]);
//         let f = Completion::new_selected("o".to_string());
//         assert_eq!(Some(f), r);

//         let r = complete("f", &vec!["fo", "foo"]);
//         let f = Completion::new_selected("o".to_string());
//         assert_eq!(Some(f), r);

//         let r = complete("f", &vec!["foo", "foo"]);
//         let f = Completion::new_selected("oo ".to_string());
//         assert_eq!(Some(f), r);

//         let r = complete("f", &vec!["foo"]);
//         let f = Completion::new_selected("oo ".to_string());
//         assert_eq!(Some(f), r);

//         let r = complete("f", &vec!["fo", "foo", "fi", "fii"]);
//         let m = vec!["fi", "fii", "fo", "foo"]
//             .iter()
//             .map(|r| r.to_string())
//             .collect::<Vec<String>>();
//         let f = Completion::new_variants(m);
//         assert_eq!(Some(f), r);
//     }

//     fn get_fixture_dir() -> String {
//         // ends with a slash
//         format!("{}/test/fixture/keyboard/", get_current_dir())
//     }
// }
