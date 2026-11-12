mod fs;
mod variant;

use self::fs::paths_to_names;
use self::fs::{find_all_starts_with_as_names, find_bins_starts_with_as_names};
use self::variant::{complete_to_variants, get_prefixed_variant, Completion};
use crate::complete::fs::to_find_data;
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
        complete_path(find_data.file_prefix().unwrap_or(""), find_data.find_path())
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

    if let Some(names) = find_bins_starts_with_as_names(starts_with, paths) {
        let names = names.iter().map(|s| s.as_str()).collect();

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

fn complete_path(starts_with: &str, path: &str) -> Option<Completion> {
    let paths = vec![path];
    let found = find_all_starts_with_as_names(starts_with, &paths);

    if found.is_none() {
        return None;
    }

    let found: Vec<String> = found
        .unwrap()
        .into_iter()
        .filter(|r| r.as_str() != starts_with)
        .collect();

    let len = found.len();

    if len == 0 {
        return None;
    }

    if len == 1 {
        let selected = found[0].clone();
        let selected = selected.replacen(starts_with, "", 1);
        let selected = whitespace_if_file(selected);

        return Some(Completion::from_selected(selected));
    }

    let variants = found.iter().map(|r| r.as_str()).collect();

    if let Some(prefixed) = get_prefixed_variant(starts_with, variants) {
        let selected = prefixed.replacen(starts_with, "", 1);
        let selected = trim_slash_if_dir(selected);
        Some(Completion::from_selected(selected))
    } else {
        let mut variants = found;
        variants.sort_unstable();
        Some(Completion::from_variants(variants))
    }
}

fn trim_slash_if_dir(selected: String) -> String {
    if selected.ends_with("/") {
        selected.trim_end_matches("/").to_string()
    } else {
        selected
    }
}

fn whitespace_if_file(selected: String) -> String {
    if selected.ends_with("/") {
        selected
    } else {
        format!("{} ", selected)
    }
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

        let find_data = to_find_data("", "");
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        assert!(complete_path(input, path).is_some());

        let find_data = to_find_data("", fixture_dir.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
        let variants = completion.get_variants().unwrap();
        assert_eq!(6, variants.len());
        assert!(variants.contains(&"bar/"));
        assert!(variants.contains(&"fo/"));
        assert!(variants.contains(&"foo/"));
        assert!(variants.contains(&"b"));
        assert!(variants.contains(&"f"));
        assert!(variants.contains(&"f.txt"));
        assert!(completion.get_selected().is_none());

        let find_data = to_find_data("f", fixture_dir.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
        let variants = completion.get_variants().unwrap();
        assert_eq!(3, variants.len());
        assert!(variants.contains(&"fo/"));
        assert!(variants.contains(&"foo/"));
        assert!(variants.contains(&"f.txt"));
        assert!(completion.get_selected().is_none());

        let find_data = to_find_data("foo", fixture_dir.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("/", selected);
        assert!(completion.get_variants().is_none());

        let find_data = to_find_data("foo/", fixture_dir.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        assert!(complete_path(input, path).is_none());

        let fixture_path = format!("{}fo/", fixture_dir.to_string());
        let find_data = to_find_data("", fixture_path.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("b ", selected);
        assert!(completion.get_variants().is_none());

        let fixture_path = format!("{}foo/", fixture_dir.to_string());
        let find_data = to_find_data("f", fixture_path.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("o", selected);
        assert!(completion.get_variants().is_none());

        let fixture_path = format!("{}foo/", fixture_dir.to_string());
        let find_data = to_find_data("", fixture_path.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
        let selected = completion.get_selected().unwrap();
        assert_eq!("fo", selected);
        assert!(completion.get_variants().is_none());

        let fixture_path = format!("{}foo/", fixture_dir.to_string());
        let find_data = to_find_data("foo", fixture_path.as_str());
        let input = find_data.file_prefix().unwrap_or("");
        let path = find_data.find_path();
        let completion = complete_path(input, path).unwrap();
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
    fn test_trim_slash_if_dir() -> Result<(), Error> {
        assert_eq!("dir", trim_slash_if_dir("dir/".to_string()));
        assert_eq!("file", trim_slash_if_dir("file".to_string()));
        assert_eq!("file.txt", trim_slash_if_dir("file.txt".to_string()));

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

        let variants = vec!["fbzn.txt", "fbz/", "fb/"];
        assert_eq!("fb/", get_prefixed_variant("f", variants).unwrap());

        Ok(())
    }

    fn get_fixture_dir() -> Result<String, Error> {
        // ends with a slash
        Ok(format!("{}/test/fixture/complete/", get_current_dir()?))
    }
}
