mod structure;

use self::structure::{Completion, FileFindData};
use crate::fs::{find_all_starts_with, find_bins_starts_with, FindFilesResult};
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
        let find_data = to_find_data(last.unwrap(), setting.current_dir());
        let find_path = find_data.find_path();
        let file_prefix = find_data.file_prefix().unwrap_or("");

        complete_file(file_prefix, find_path)
    } else {
        None
    }
}

fn complete_path(find_data: &FileFindData) -> Option<Completion> {
    let starts_with = find_data.file_prefix().unwrap_or("");
    let paths = vec![find_data.find_path()];

    let found = find_all_starts_with(starts_with, &paths);

    // ignore errors
    if !found.is_some() {
        return None;
    }

    let found = found.unwrap();
    let found = found.iter().map(|r| r.as_str()).collect();
    let found = paths_to_names(&found);

    if found.len() == 1 {
        let selected: String = found[0].to_string();
        Some(Completion::from_selected(selected))
    } else {
        Some(Completion::from_variants(found))
    }
}

fn to_find_data(input: &str, default_path: &str) -> FileFindData {
    let input = input.trim();

    let default_path = if !default_path.trim().ends_with("/") {
        format!("{}/", default_path.trim())
    } else {
        default_path.trim().to_string()
    };

    if input.is_empty() {
        return FileFindData::from(default_path, None);
    }

    if !input.contains("/") {
        return FileFindData::from(default_path, Some(input.to_string()));
    }

    let input: Vec<&str> = input.split("/").collect();
    let file_prefix = input.last().unwrap();

    if file_prefix.is_empty() {
        FileFindData::from(input.join("/"), None)
    } else {
        let find_path = format!("{}/", input[0..input.len() - 1].join("/"));
        FileFindData::from(find_path, Some(file_prefix.to_string()))
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

        Some(Completion::from_variants(r))
    } else {
        None
    }
}

fn complete_file(input: &str, path: &str) -> Option<Completion> {
    let mut variants: Option<Vec<String>> = None;

    if let FindFilesResult::Some(r) = find_all_starts_with(input, &vec![path]) {
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

        Some(Completion::from_variants(r))
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
        return Some(Completion::from_selected(selected));
    }

    let is_chain = matches
        .iter()
        .filter(|&r| r != short)
        .all(|r| Comprasion::PatternStartsWith(r.to_string()).assert(short.as_str()));

    if is_chain {
        let selected = short.replacen(input, "", 1);
        Some(Completion::from_selected(selected))
    } else {
        Some(Completion::from_variants(matches))
    }
}

fn paths_to_names(paths: &Vec<&str>) -> Vec<String> {
    let mut names = vec![];

    for path in paths {
        let name = if path.ends_with("/") {
            let name = path.trim_end_matches(|r| r == '/');
            format!("{}/", name.split("/").last().unwrap())
        } else {
            path.split("/").last().unwrap().to_string()
        };

        names.push(name);
    }

    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::get_current_dir;
    use std::io::Error;

    #[test]
    fn test_complete_path() -> Result<(), Error> {
        let fixture_dir = get_fixture_dir()?;

        let find_data = FileFindData::from("".to_string(), None);
        assert!(complete_path(&find_data).is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), None);
        let completion = complete_path(&find_data).unwrap();
        let variants = completion.get_variants().unwrap();
        assert!(variants.len() == 5);
        assert!(variants.contains(&"fo/"));
        assert!(variants.contains(&"foo/"));
        assert!(variants.contains(&"b"));
        assert!(variants.contains(&"f"));
        assert!(variants.contains(&"f.txt"));
        assert!(completion.get_selected().is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), Some("f".to_string()));
        let completion = complete_path(&find_data).unwrap();
        let variants = completion.get_variants().unwrap();
        assert!(variants.len() == 4);
        assert!(variants.contains(&"fo/"));
        assert!(variants.contains(&"foo/"));
        assert!(variants.contains(&"f"));
        assert!(variants.contains(&"f.txt"));
        assert!(completion.get_selected().is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), Some("foo".to_string()));
        let completion = complete_path(&find_data).unwrap();
        let selected = completion.get_selected().unwrap();
        assert!("foo/" == selected);
        assert!(completion.get_variants().is_none());

        Ok(())
    }

    #[test]
    fn test_paths_to_names() -> Result<(), Error> {
        let paths = vec!["/f/b/c.txt", "f/b/c", "/f/b/z/"];
        let names = vec!["c.txt", "c", "z/"];
        assert_eq!(names, paths_to_names(&paths));

        Ok(())
    }

    #[test]
    fn test_to_find_data() -> Result<(), Error> {
        let data = to_find_data("", "");
        assert_eq!("/", data.find_path());
        assert_eq!(None, data.file_prefix());

        let data = to_find_data(" ", " ");
        assert_eq!("/", data.find_path());
        assert_eq!(None, data.file_prefix());

        let data = to_find_data("f", "d");
        assert_eq!("d/", data.find_path());
        assert_eq!(Some("f"), data.file_prefix());

        let data = to_find_data("/f", "/d");
        assert_eq!("/", data.find_path());
        assert_eq!(Some("f"), data.file_prefix());

        let data = to_find_data("f/", "d/");
        assert_eq!("f/", data.find_path());
        assert_eq!(None, data.file_prefix());

        let data = to_find_data("/f/", "/d/");
        assert_eq!("/f/", data.find_path());
        assert_eq!(None, data.file_prefix());

        let data = to_find_data("f/b", "d");
        assert_eq!("f/", data.find_path());
        assert_eq!(Some("b"), data.file_prefix());

        let data = to_find_data("/f/b", "d");
        assert_eq!("/f/", data.find_path());
        assert_eq!(Some("b"), data.file_prefix());

        let data = to_find_data("/f/b/", "d");
        assert_eq!("/f/b/", data.find_path());
        assert_eq!(None, data.file_prefix());

        Ok(())
    }

    fn get_fixture_dir() -> Result<String, Error> {
        // ends with a slash
        Ok(format!("{}/test/fixture/complete/", get_current_dir()?))
    }
}
