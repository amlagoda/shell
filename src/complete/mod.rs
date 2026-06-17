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
        complete_path(&find_data)
    } else {
        None
    }
}

fn complete_command(
    starts_with: &str,
    commands: &Vec<&str>,
    paths: &Vec<&str>,
) -> Option<Completion> {
    let mut variants = vec![];

    if let Some(completion) = complete_to_variants(starts_with, commands) {
        if completion.is_selected() {
            return Some(completion);
        }

        let found = completion.get_variants().unwrap();
        let found = found.iter().map(|r| r.to_string()).collect();
        variants = found;
    }

    if let FindFilesResult::Some(paths) = find_bins_starts_with(starts_with, paths) {
        let paths = paths.iter().map(|r| r.as_str()).collect();
        let names = paths_to_names(&paths);
        let names = names.iter().map(|r| r.as_str()).collect();

        if let Some(completion) = complete_to_variants(starts_with, &names) {
            if completion.is_selected() {
                return Some(completion);
            }

            let found = completion.get_variants().unwrap();
            let mut found = found.iter().map(|r| r.to_string()).collect();
            variants.append(&mut found);
        }
    }

    variants.sort_unstable();
    variants.dedup();

    if variants.is_empty() {
        return None;
    }

    Some(Completion::from_variants(variants))
}

fn complete_to_variants(starts_with: &str, variants: &Vec<&str>) -> Option<Completion> {
    if variants.is_empty() {
        return None;
    }

    if starts_with.is_empty() {
        let variants = variants.iter().map(|r| r.to_string()).collect();
        return Some(Completion::from_variants(variants));
    }

    let mut found = vec![];

    for variant in variants {
        if Comprasion::PatternStartsWithNotEqual(variant.to_string()).assert(starts_with) {
            found.push(variant.to_string());
        }
    }

    let len = found.len();

    if len == 0 {
        return None;
    }

    let short = found.iter().min_by_key(|r| r.len()).unwrap();

    if len == 1 {
        let selected = format!("{} ", short.replacen(starts_with, "", 1));
        return Some(Completion::from_selected(selected));
    }

    let variants = found.iter().map(|r| r.as_str()).collect();

    if let Some(prefixed) = get_prefixed_variant(starts_with, variants) {
        let selected = prefixed.replacen(starts_with, "", 1);
        Some(Completion::from_selected(selected))
    } else {
        Some(Completion::from_variants(found))
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
    let found: Vec<String> = paths_to_names(&found)
        .into_iter()
        .filter(|r| r != starts_with)
        .collect();
    let len = found.len();

    if len == 0 {
        return None;
    }

    if len == 1 {
        let selected = found[0].to_string();
        let selected = selected.replacen(starts_with, "", 1);
        let selected = whitespace_if_file(selected);

        return Some(Completion::from_selected(selected));
    }

    let variants = found.iter().map(|r| r.as_str()).collect();

    if let Some(prefixed) = get_prefixed_variant(starts_with, variants) {
        let selected = prefixed.replacen(starts_with, "", 1);
        Some(Completion::from_selected(selected))
    } else {
        Some(Completion::from_variants(found))
    }
}

fn whitespace_if_file(selected: String) -> String {
    if selected.ends_with("/") {
        selected
    } else {
        format!("{} ", selected)
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

fn paths_to_names(paths: &Vec<&str>) -> Vec<String> {
    let mut names = vec![];

    for path in paths {
        let name = if path.ends_with("/") {
            let name = path.trim_end_matches(['/']);
            format!("{}/", name.split("/").last().unwrap())
        } else {
            path.split("/").last().unwrap().to_string()
        };

        names.push(name);
    }

    names
}

fn get_prefixed_variant(current: &str, mut variants: Vec<&str>) -> Option<String> {
    if variants.is_empty() {
        return None;
    }

    if variants.len() == 1 {
        return Some(variants[0].to_string());
    }

    let short = if current.is_empty() {
        variants.iter().min_by_key(|r| r.len()).unwrap()
    } else {
        variants = variants
            .iter()
            .filter(|r| **r != current)
            .map(|r| *r)
            .collect();

        variants.iter().min_by_key(|r| r.len()).unwrap()
    };

    let is_chain = variants
        .iter()
        .all(|r| Comprasion::PatternStartsWith(r.to_string()).assert(short));

    if !is_chain {
        return None;
    }

    Some(short.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::get_current_dir;
    use std::io::Error;

    #[test]
    fn test_complete_to_variants() -> Result<(), Error> {
        let variants = vec![];
        assert!(complete_to_variants("", &variants).is_none());

        let variants = vec!["f", "b"];
        let variants = complete_to_variants("", &variants).unwrap();
        let variants = variants.get_variants().unwrap();
        assert_eq!(vec!["f", "b"], variants);

        let variants = vec!["f", "b"];
        assert!(complete_to_variants("c", &variants).is_none());

        let variants = vec!["fo", "b"];
        let selected = complete_to_variants("f", &variants).unwrap();
        let selected = selected.get_selected().unwrap();
        assert_eq!("o ", selected);

        let variants = vec!["fo", "fy"];
        let variants = complete_to_variants("f", &variants).unwrap();
        let variants = variants.get_variants().unwrap();
        assert_eq!(vec!["fo", "fy"], variants);

        let variants = vec!["fo", "foo"];
        let selected = complete_to_variants("f", &variants).unwrap();
        let selected = selected.get_selected().unwrap();
        assert_eq!("o", selected);

        Ok(())
    }

    #[test]
    fn test_complete_path() -> Result<(), Error> {
        let fixture_dir = get_fixture_dir()?;

        let find_data = FileFindData::from("".to_string(), None);
        assert!(complete_path(&find_data).is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), None);
        let completion = complete_path(&find_data).unwrap();
        let variants = completion.get_variants().unwrap();
        assert_eq!(6, variants.len());
        assert!(variants.contains(&"bar/"));
        assert!(variants.contains(&"fo/"));
        assert!(variants.contains(&"foo/"));
        assert!(variants.contains(&"b"));
        assert!(variants.contains(&"f"));
        assert!(variants.contains(&"f.txt"));
        assert!(completion.get_selected().is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), Some("f".to_string()));
        let completion = complete_path(&find_data).unwrap();
        let variants = completion.get_variants().unwrap();
        assert_eq!(3, variants.len());
        assert!(variants.contains(&"fo/"));
        assert!(variants.contains(&"foo/"));
        assert!(variants.contains(&"f.txt"));
        assert!(completion.get_selected().is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), Some("foo".to_string()));
        let completion = complete_path(&find_data).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("/", selected);
        assert!(completion.get_variants().is_none());

        let find_data = FileFindData::from(fixture_dir.to_string(), Some("foo/".to_string()));
        assert!(complete_path(&find_data).is_none());

        let path = format!("{}fo/", fixture_dir.to_string());
        let find_data = FileFindData::from(path, None);
        let completion = complete_path(&find_data).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("b ", selected);
        assert!(completion.get_variants().is_none());

        let path = format!("{}foo/", fixture_dir.to_string());
        let find_data = FileFindData::from(path, Some("f".to_string()));
        let completion = complete_path(&find_data).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("o", selected);
        assert!(completion.get_variants().is_none());

        let path = format!("{}foo/", fixture_dir.to_string());
        let find_data = FileFindData::from(path, None);
        let completion = complete_path(&find_data).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("fo", selected);
        assert!(completion.get_variants().is_none());

        let path = format!("{}foo/", fixture_dir.to_string());
        let find_data = FileFindData::from(path, Some("foo".to_string()));
        let completion = complete_path(&find_data).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("o ", selected);
        assert!(completion.get_variants().is_none());

        Ok(())
    }

    #[test]
    fn test_whitespace_if_file() -> Result<(), Error> {
        assert_eq!("dir/", whitespace_if_file("dir/".to_string()));
        assert_eq!("file ", whitespace_if_file("file".to_string()));
        assert_eq!("file.txt ", whitespace_if_file("file.txt".to_string()));

        Ok(())
    }

    #[test]
    fn test_get_prefixed_variant() -> Result<(), Error> {
        let variants = vec![];
        assert!(get_prefixed_variant("", variants).is_none());

        let variants = vec![];
        assert!(get_prefixed_variant("f", variants).is_none());

        let variants = vec!["f"];
        assert_eq!("f", get_prefixed_variant("", variants).unwrap());

        let variants = vec!["f"];
        assert_eq!("f", get_prefixed_variant("f", variants).unwrap());

        let variants = vec!["f", "fo", "b"];
        assert!(get_prefixed_variant("", variants).is_none());

        let variants = vec!["f", "fo", "foo"];
        assert_eq!("f", get_prefixed_variant("", variants).unwrap());

        let variants = vec!["f", "fo", "foo", "fy"];
        assert!(get_prefixed_variant("f", variants).is_none());

        let variants = vec!["f", "fo", "foo"];
        assert_eq!("fo", get_prefixed_variant("f", variants).unwrap());

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

    #[test]
    fn test_paths_to_names() -> Result<(), Error> {
        let paths = vec!["/f/b/c.txt", "f/b/c", "/f/b/z/"];
        let names = vec!["c.txt", "c", "z/"];
        assert_eq!(names, paths_to_names(&paths));

        Ok(())
    }

    fn get_fixture_dir() -> Result<String, Error> {
        // ends with a slash
        Ok(format!("{}/test/fixture/complete/", get_current_dir()?))
    }
}
